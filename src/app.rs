use crate::config::Config;
use crate::controllers::default_media_receiver::DefaultMediaReceiver;
use crate::controllers::media::{MediaSource, PlaybackStatus};
use crate::controllers::receiver::ReceiverController;
use crate::controllers::tui::{TuiCommand, TuiController, TuiState};
use crate::discovery::{CastDevice, discover_devices_async};
use crate::protocol::media::{MediaInformation, MediaResponse, NAMESPACE as MEDIA_NAMESPACE, MediaTrack};
use crate::protocol::receiver::{ReceiverResponse, NAMESPACE as RECEIVER_NAMESPACE};
use crate::server::{get_mime_type, StreamServer, StreamSource};
use crate::torrent::{TorrentConfig, TorrentManager, TorrentStreamInfo};
use crate::transcode::{needs_transcoding, probe_media, spawn_ffmpeg, MediaProbeResult, TranscodeConfig};
use crate::CastClient;

use std::collections::VecDeque;
use std::error::Error;
use std::net::IpAddr;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use librqbit::ManagedTorrent;

enum InternalEvent {
    ProbeCompleted {
        duration: Option<f64>,
        video_codec: Option<String>,
        audio_codec: Option<String>,
    },
}

use tokio::sync::mpsc;

struct AppState {
    is_transcoding: bool,
    seek_offset: f64,
    current_time: f64,
    last_known_time: f64,
    last_update_instant: std::time::Instant,
    pause_start_time: Option<std::time::Instant>,
    total_duration: Option<f64>,
    volume_level: Option<f32>,
    is_muted: bool,
    source: Option<MediaSource>,
    current_media_idx: usize,
    video_codec: Option<String>,
    audio_codec: Option<String>,
    device_name: String,
    animation_frame: usize,
    media_session_id: Option<i32>,
    torrent_progress: Option<f32>,
    torrent_file_name: Option<String>,
    torrent_handle: Option<Arc<ManagedTorrent>>,
    subtitles: Option<String>,
}

const TORRENT_BUFFER_PCT_THRESHOLD: f32 = 3.0;
const TORRENT_BUFFER_SIZE_THRESHOLD: u64 = 10 * 1024 * 1024; // 10MB
const WATCHDOG_TIMEOUT_SEC: u64 = 30;
const BUFFER_UNDERRUN_THRESHOLD: f32 = 0.5; // percent
const BUFFER_RESUME_THRESHOLD: f32 = 2.0; // percent

pub struct CastNowCore {
    config: Config,
}

impl CastNowCore {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        // Setup Logging if requested
        if let Some(ref log_path) = self.config.log_file {
            setup_logging(log_path)?;
        }

        // 0. Prepare Playlist
        let mut playlist = VecDeque::new();
        for input in &self.config.inputs {
            if input.starts_with("magnet:?") {
                playlist.push_back(MediaSource::Magnet(input.clone()));
            } else if input.ends_with(".torrent") {
                playlist.push_back(MediaSource::TorrentFile(input.clone()));
            } else {
                let path = Path::new(&input);
                if path.exists() {
                    playlist.push_back(MediaSource::FilePath(input.clone()));
                } else {
                    playlist.push_back(MediaSource::Url(input.clone()));
                }
            }
        }

        if playlist.is_empty() {
            return Err("No valid media sources found".into());
        }

        // 1. Setup Server (lazy init)
        let mut server = StreamServer::new();
        let bind_ip = if let Some(ip) = &self.config.myip {
            ip.clone()
        } else {
            get_local_ip().ok_or("Could not determine local IP")?.to_string()
        };
        let server_url_base = server.start(&bind_ip, self.config.port).await?;
        log::info!("Server started at {}", server_url_base);

        // Setup Torrent Manager
        let torrent_manager = Arc::new(TorrentManager::new(TorrentConfig::default()).await?);

        // 2. Discover or Target device
        let device = if let Some(ip_str) = &self.config.target_ip {
            if !self.config.quiet { println!("Targeting specific IP: {}", ip_str); }
            CastDevice {
                ip: ip_str.parse().map_err(|_| "Invalid IP address provided")?,
                port: 8009,
                friendly_name: "Direct Connect".to_string(),
                model_name: "Unknown".to_string(),
                uuid: "Unknown".to_string(),
            }
        } else {
            if !self.config.quiet { println!("Searching for Cast devices..."); }
            let mut rx = discover_devices_async()?;
            let matching_device;

            let timeout = tokio::time::sleep(Duration::from_secs(10));
            tokio::pin!(timeout);

            loop {
                tokio::select! {
                    Some(d) = rx.recv() => {
                        if let Some(ref name) = self.config.target_name {
                            if d.friendly_name == *name {
                                println!("Found matching device: {}", d.friendly_name);
                                matching_device = Some(d);
                                break;
                            }
                        } else {
                            // First found
                            println!("Found device: {}", d.friendly_name);
                            matching_device = Some(d);
                            break;
                        }
                    }
                    _ = &mut timeout => {
                        matching_device = None;
                        break;
                    }
                }
            }
            matching_device.ok_or("No matching device found")?
        };
        
        if !self.config.quiet { println!("Found {}", device.friendly_name); }

        // 3. Connect and Launch
        if !self.config.quiet { println!("Connecting to {}...", device.ip); }
        let mut client = CastClient::connect(&device.ip.to_string(), device.port).await?;
        let mut receiver_ctrl = ReceiverController::new(&client);
        client.connect_receiver().await?;

        let mut app = DefaultMediaReceiver::new(&client);
        app.launch().await?;
        log::info!("Default Media Receiver launched.");

        // Apply volume if specified
        if let Some(vol) = self.config.volume {
            let _ = receiver_ctrl.set_volume(vol).await;
        }

        // 4. Start TUI
        let tui = TuiController::new();
        let mut tui_rx = tui.start()?;

        // TUI Loop
        let mut current_status = PlaybackStatus::Idle;

        let mut app_state = AppState {
            is_transcoding: false,
            seek_offset: 0.0,
            current_time: 0.0,
            last_known_time: 0.0,
            last_update_instant: std::time::Instant::now(),
            pause_start_time: None,
            total_duration: None,
            volume_level: Some(1.0),
            is_muted: false,
            source: None,
            current_media_idx: 0,
            video_codec: None,
            audio_codec: None,
            device_name: device.friendly_name.clone(),
            animation_frame: 0,
            media_session_id: None,
            torrent_progress: None,
            torrent_file_name: None,
            torrent_handle: None,
            subtitles: self.config.subtitles.clone(),
        };

        let mut events = client.events();
        let (probe_tx, mut probe_rx) = mpsc::channel(16);

        // Load first item
        if let Some(source) = playlist.front() {
            app_state.current_media_idx = 0;
            app_state.source = Some(source.clone());
            match load_media(
                &app,
                &server,
                source,
                &server_url_base,
                0.0,
                &torrent_manager,
                &tui,
                &mut app_state,
                Some(probe_tx.clone()),
            )
            .await
            {
                Ok((is_tx, probe, offset)) => {
                    app_state.is_transcoding = is_tx;
                    app_state.seek_offset = offset;
                    app_state.current_time = offset;
                    app_state.last_known_time = offset;
                    app_state.last_update_instant = std::time::Instant::now();
                    app_state.total_duration = probe.duration;
                    app_state.video_codec = probe.video_codec;
                    app_state.audio_codec = probe.audio_codec;
                }
                Err(e) => log::error!("Failed to load media: {}", e),
            }
        }

        // Event Loop
        let mut animation_interval = tokio::time::interval(Duration::from_millis(150));
        let mut watchdog_interval = tokio::time::interval(Duration::from_secs(1));

        loop {
            tokio::select! {
                Some(event) = probe_rx.recv() => {
                    match event {
                        InternalEvent::ProbeCompleted { duration, video_codec, audio_codec } => {
                            if let Some(d) = duration {
                                app_state.total_duration = Some(d);
                            }
                            if video_codec.is_some() {
                                app_state.video_codec = video_codec;
                            }
                            if audio_codec.is_some() {
                                app_state.audio_codec = audio_codec;
                            }
                        }
                    }
                }
                _ = watchdog_interval.tick() => {
                    // Auto-recovery from Waiting (System/User Pause or Error)
                    if matches!(current_status, PlaybackStatus::Waiting) {
                        if let Some(pause_start) = app_state.pause_start_time {
                            if pause_start.elapsed() > Duration::from_secs(10) {
                                // Attempt full reload to recover from potential transcoding crashes or idle states
                                if let Some(source) = app_state.source.clone() {
                                    log::info!("Auto-recovery: 10s wait elapsed. Attempting reload...");
                                    let curr_time = app_state.current_time;
                                    if let Ok((is_tx, probe, offset)) = load_media(
                                        &app,
                                        &server,
                                        &source,
                                        &server_url_base,
                                        curr_time,
                                        &torrent_manager,
                                        &tui,
                                        &mut app_state,
                                        Some(probe_tx.clone()),
                                    ).await {
                                         app_state.is_transcoding = is_tx;
                                         app_state.seek_offset = offset;
                                         app_state.last_known_time = app_state.current_time;
                                         app_state.last_update_instant = std::time::Instant::now();
                                         app_state.total_duration = probe.duration;
                                         app_state.video_codec = probe.video_codec;
                                         app_state.audio_codec = probe.audio_codec;
                                         // If success, Waiting status will change to PLAYING/BUFFERING via events
                                    }
                                }
                                // Reset wait time to avoid spamming
                                app_state.pause_start_time = Some(std::time::Instant::now()); 
                            }
                        }
                    }

                    // Watchdog: If playing but time hasn't advanced for X seconds, resume.
                    if matches!(current_status, PlaybackStatus::Playing)
                        && app_state.last_update_instant.elapsed() > Duration::from_secs(WATCHDOG_TIMEOUT_SEC) {
                            if let Some(source) = app_state.source.clone() {
                                let curr_time = app_state.current_time;
                                match load_media(&app, &server, &source, &server_url_base, curr_time, &torrent_manager, &tui, &mut app_state, Some(probe_tx.clone())).await {
                                    Ok((is_tx, probe, offset)) => {
                                        app_state.is_transcoding = is_tx;
                                        app_state.seek_offset = offset;
                                        app_state.last_known_time = app_state.current_time;
                                        app_state.last_update_instant = std::time::Instant::now();
                                        app_state.total_duration = probe.duration;
                                        app_state.video_codec = probe.video_codec;
                                        app_state.audio_codec = probe.audio_codec;
                                    },
                                    Err(e) => eprintln!("Watchdog resume failed: {}", e),
                                }
                            }
                        }
                },
                Some(cmd) = tui_rx.recv() => {
                    match cmd {
                        TuiCommand::Quit => break,
                        TuiCommand::TogglePlay => {
                            let sid = app_state.media_session_id.unwrap_or(1);
                            match current_status {
                                PlaybackStatus::Playing | PlaybackStatus::Buffering => {
                                    let _ = app.pause(sid).await;
                                    log::info!("User toggled pause. Status: {:?} -> Waiting", current_status);
                                    current_status = PlaybackStatus::Waiting;
                                    app_state.pause_start_time = Some(std::time::Instant::now());
                                },
                                _ => {
                                    let _ = app.play(sid).await;
                                    log::info!("User toggled play. Status: {:?} -> Playing", current_status);
                                    current_status = PlaybackStatus::Playing;
                                    app_state.last_update_instant = std::time::Instant::now();
                                }
                            }
                        },
                        TuiCommand::Pause => {
                            let sid = app_state.media_session_id.unwrap_or(1);
                            let _ = app.pause(sid).await;
                            log::info!("User paused. Status: {:?} -> Waiting", current_status);
                            current_status = PlaybackStatus::Waiting;
                            app_state.pause_start_time = Some(std::time::Instant::now());
                        },
                        TuiCommand::Play => {
                             let sid = app_state.media_session_id.unwrap_or(1);
                             let _ = app.play(sid).await;
                             log::info!("User played. Status: {:?} -> Playing", current_status);
                             current_status = PlaybackStatus::Playing;
                             app_state.pause_start_time = None;
                        },
                        TuiCommand::Next => {
                            app_state.current_media_idx += 1;
                             if let Some(source) = playlist.get(app_state.current_media_idx) {
                                 app_state.source = Some(source.clone());
                                if let Ok((is_tx, probe, offset)) = load_media(&app, &server, source, &server_url_base, 0.0, &torrent_manager, &tui, &mut app_state, Some(probe_tx.clone())).await {
                                     app_state.is_transcoding = is_tx;
                                     app_state.seek_offset = offset;
                                     app_state.current_time = offset;
                                     app_state.last_known_time = offset;
                                     app_state.last_update_instant = std::time::Instant::now();
                                     app_state.total_duration = probe.duration;
                                     app_state.video_codec = probe.video_codec;
                                     app_state.audio_codec = probe.audio_codec;
                                }
                             } else {
                                 app_state.current_media_idx -= 1;
                             }
                        },
                        TuiCommand::Previous => {
                            if app_state.current_media_idx > 0 {
                                app_state.current_media_idx -= 1;
                                 if let Some(source) = playlist.get(app_state.current_media_idx) {
                                    app_state.source = Some(source.clone());
                                    if let Ok((is_tx, probe, offset)) = load_media(&app, &server, source, &server_url_base, 0.0, &torrent_manager, &tui, &mut app_state, Some(probe_tx.clone())).await {
                                         app_state.is_transcoding = is_tx;
                                         app_state.seek_offset = offset;
                                         app_state.current_time = offset;
                                         app_state.last_known_time = offset;
                                         app_state.last_update_instant = std::time::Instant::now();
                                         app_state.total_duration = probe.duration;
                                         app_state.video_codec = probe.video_codec;
                                         app_state.audio_codec = probe.audio_codec;
                                    }
                                 }
                            }
                        },
                        TuiCommand::SeekForward(s) => {
                             let new_time = app_state.current_time + s as f64;
                             if app_state.is_transcoding {
                                 if let Some(src) = app_state.source.clone() {
                                     match load_media(&app, &server, &src, &server_url_base, new_time, &torrent_manager, &tui, &mut app_state, Some(probe_tx.clone())).await {
                                          Ok((is_tx, probe, offset)) => {
                                             app_state.is_transcoding = is_tx;
                                             app_state.seek_offset = offset;
                                             app_state.current_time = new_time;
                                             app_state.last_known_time = new_time;
                                             app_state.last_update_instant = std::time::Instant::now();
                                             app_state.total_duration = probe.duration;
                                             app_state.video_codec = probe.video_codec;
                                             app_state.audio_codec = probe.audio_codec;
                                         },
                                         Err(e) => eprintln!("SeekForward load error: {}", e),
                                     }
                                 }
                             } else {
                                 let _ = app.seek(app_state.media_session_id.unwrap_or(1), new_time as f32).await;
                                 app_state.current_time = new_time;
                                 app_state.last_known_time = new_time;
                                 app_state.last_update_instant = std::time::Instant::now();
                             }
                        },
                        TuiCommand::SeekBackward(s) => {
                             let new_time = (app_state.current_time - s as f64).max(0.0);
                             if app_state.is_transcoding {
                                 if let Some(src) = app_state.source.clone() {
                                     match load_media(&app, &server, &src, &server_url_base, new_time, &torrent_manager, &tui, &mut app_state, Some(probe_tx.clone())).await {
                                          Ok((is_tx, probe, offset)) => {
                                             app_state.is_transcoding = is_tx;
                                             app_state.seek_offset = offset;
                                             app_state.current_time = new_time;
                                             app_state.last_known_time = new_time;
                                             app_state.last_update_instant = std::time::Instant::now();
                                             app_state.total_duration = probe.duration;
                                             app_state.video_codec = probe.video_codec;
                                             app_state.audio_codec = probe.audio_codec;
                                         },
                                         Err(e) => eprintln!("SeekBackward load error: {}", e),
                                     }
                                 }
                             } else {
                                 let _ = app.seek(app_state.media_session_id.unwrap_or(1), new_time as f32).await;
                                 app_state.current_time = new_time;
                                 app_state.last_known_time = new_time;
                                 app_state.last_update_instant = std::time::Instant::now();
                             }
                        },
                        TuiCommand::VolumeUp => {
                            let new_vol = (app_state.volume_level.unwrap_or(0.0) + 0.05).min(1.0);
                            let _ = receiver_ctrl.set_volume(new_vol).await;
                            app_state.volume_level = Some(new_vol);
                        },
                        TuiCommand::VolumeDown => {
                            let new_vol = (app_state.volume_level.unwrap_or(0.0) - 0.05).max(0.0);
                            let _ = receiver_ctrl.set_volume(new_vol).await;
                            app_state.volume_level = Some(new_vol);
                        },
                        TuiCommand::ToggleMute => {
                            let new_mute = !app_state.is_muted;
                            let _ = receiver_ctrl.set_mute(new_mute).await;
                            app_state.is_muted = new_mute;
                        },
                        TuiCommand::Stop => {
                            let sid = app_state.media_session_id.unwrap_or(1);
                            let _ = app.pause(sid).await;
                            log::info!("User stopped. Status: {:?} -> Finished", current_status);
                            current_status = PlaybackStatus::Finished;
                            app_state.torrent_handle = None;
                            app_state.torrent_progress = None;
                        },
                        TuiCommand::Reconnect => {
                            log::info!("User reconnecting. Status: {:?} -> Reconnecting", current_status);
                            current_status = PlaybackStatus::Reconnecting;
                            // Draw RECONNECTING
                             let tui_state = TuiState {
                                status: "RECONNECTING".to_string(),
                                current_time: app_state.current_time as f32,
                                total_duration: app_state.total_duration.map(|d| d as f32),
                                volume_level: app_state.volume_level,
                                is_muted: app_state.is_muted,
                                media_title: None,
                                video_codec: app_state.video_codec.clone(),
                                audio_codec: app_state.audio_codec.clone(),
                                device_name: app_state.device_name.clone(),
                                animation_frame: app_state.animation_frame,
                                torrent_progress: app_state.torrent_progress,
                            };
                            let _ = tui.draw(&tui_state);

                            match CastClient::connect(&device.ip.to_string(), device.port).await {
                                Ok(new_client) => {
                                    client = new_client;
                                    receiver_ctrl = ReceiverController::new(&client);
                                    let _ = client.connect_receiver().await;
                                    app = DefaultMediaReceiver::new(&client);
                                    let _ = app.launch().await;
                                    events = client.events();
                                }
                                Err(e) => {
                                    eprintln!("Reconnect failed: {}", e);
                                    log::error!("Reconnect failed: {}", e);
                                    current_status = PlaybackStatus::Idle;
                                }
                            }
                        },
                    }
                    let tui_state = TuiState {
                        status: format!("{:?}", current_status),
                        current_time: app_state.current_time as f32,
                        total_duration: app_state.total_duration.map(|d| d as f32),
                        volume_level: app_state.volume_level,
                        is_muted: app_state.is_muted,
                        media_title: None,
                        video_codec: app_state.video_codec.clone(),
                        audio_codec: app_state.audio_codec.clone(),
                        device_name: app_state.device_name.clone(),
                        animation_frame: app_state.animation_frame,
                        torrent_progress: app_state.torrent_progress,
                    };
                    let _ = tui.draw(&tui_state);
                }
                Ok(event) = events.recv() => {
                    // Handle events (same logic as main.rs)
                     if event.namespace == MEDIA_NAMESPACE {
                         if let Ok(MediaResponse::MediaStatus { status, .. }) = serde_json::from_str::<MediaResponse>(&event.payload) {
                              if let Some(s) = status.first() {
                                  let reported_time = s.current_time as f64;
                                  if app_state.is_transcoding {
                                      app_state.current_time = reported_time + app_state.seek_offset;
                                  } else {
                                      app_state.current_time = reported_time;
                                  }

                                  if (app_state.current_time - app_state.last_known_time).abs() > 0.1 {
                                      app_state.last_known_time = app_state.current_time;
                                      app_state.last_update_instant = std::time::Instant::now();
                                  }

                                  app_state.media_session_id = Some(s.media_session_id);
                                  if let Some(vol) = &s.volume {
                                      app_state.volume_level = vol.level;
                                      if let Some(muted) = vol.muted {
                                          app_state.is_muted = muted;
                                      }
                                  }

                                  match s.player_state.as_str() {
                                      "PLAYING" => {
                                          if current_status != PlaybackStatus::Playing {
                                              log::info!("Receiver reported PLAYING. Status: {:?} -> Playing", current_status);
                                          }
                                          current_status = PlaybackStatus::Playing;
                                          app_state.pause_start_time = None;
                                      },
                                      "PAUSED" => {
                                          if current_status == PlaybackStatus::Buffering {
                                              // Keep buffering state if we initiated it (don't switch to Waiting)
                                              log::debug!("Receiver reported PAUSED (Buffering). Keeping Buffering state.");
                                              current_status = PlaybackStatus::Buffering;
                                          } else {
                                              // Any other pause (System or User) -> Waiting
                                              if current_status != PlaybackStatus::Waiting {
                                                  log::info!("Receiver reported PAUSED. Status: {:?} -> Waiting", current_status);
                                              }
                                              current_status = PlaybackStatus::Waiting;
                                              if app_state.pause_start_time.is_none() {
                                                  app_state.pause_start_time = Some(std::time::Instant::now());
                                              }
                                          }
                                      },
                                      "BUFFERING" => {
                                          if current_status != PlaybackStatus::Buffering {
                                              log::info!("Receiver reported BUFFERING. Status: {:?} -> Buffering", current_status);
                                          }
                                          current_status = PlaybackStatus::Buffering;
                                      },
                                                                            "IDLE" => {
                                                                                 if current_status != PlaybackStatus::Idle {
                                                                                     log::info!("Receiver reported IDLE ({:?}). Status: {:?} -> Idle", s.idle_reason, current_status);
                                                                                 }
                                                                                 current_status = PlaybackStatus::Idle;
                                                                                                                            if s.idle_reason.as_deref() == Some("ERROR") || s.idle_reason.as_deref() == Some("INTERRUPTED") {
                                                                                                                                 log::warn!("Detected Error/Interrupted status ({:?}). Transitioning to Waiting for auto-recovery...", s.idle_reason);
                                                                                                                                 current_status = PlaybackStatus::Waiting;
                                                                                                                                 if app_state.pause_start_time.is_none() {
                                                                                                                                     app_state.pause_start_time = Some(std::time::Instant::now());
                                                                                                                                 }
                                                                                                                            } else if s.idle_reason.as_deref() == Some("FINISHED") {                                                                                                                                            // Check if we really finished or if it was a drop
                                                                                                                                            let total = app_state.total_duration.unwrap_or(0.0);
                                                                                                                                            // Use a threshold (e.g. within 10s of end)
                                                                                                                                                                                                                                    if total > 0.0 && (total - app_state.current_time) > 10.0 {
                                                                                                                                                                                                                                        // Premature finish (likely transcoding crash/eof)
                                                                                                                                                                                                                                        log::warn!("Premature FINISHED detected (Current: {:.1}, Total: {:.1}). Status: {:?} -> Waiting", app_state.current_time, total, current_status);
                                                                                                                                                                                                                                        current_status = PlaybackStatus::Waiting;
                                                                                                                                                                                                                                        if app_state.pause_start_time.is_none() {
                                                                                                                                                                                                                                            app_state.pause_start_time = Some(std::time::Instant::now());
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                    } else {                                                                                                                                                                                            log::info!("Track finished normally. Loading next...");
                                                                                                                                                                                            let next_idx = app_state.current_media_idx + 1;                                                                                                                                                if next_idx < playlist.len() || self.config.loop_playlist {
                                                                                                                                                    let target_idx = if next_idx < playlist.len() { next_idx } else { 0 };
                                                                                                                                                    if let Some(source) = playlist.get(target_idx) {
                                                                                                                                                        app_state.current_media_idx = target_idx;
                                                                                                                                                        app_state.source = Some(source.clone());
                                                                                                                                                        match load_media(
                                                                                                                                                            &app,
                                                                                                                                                            &server,
                                                                                                                                                            source,
                                                                                                                                                            &server_url_base,
                                                                                                                                                            0.0,
                                                                                                                                                            &torrent_manager,
                                                                                                                                                            &tui,
                                                                                                                                                            &mut app_state,
                                                                                                                                                            Some(probe_tx.clone()),
                                                                                                                                                        ).await {
                                                                                                                                                            Ok((is_tx, probe, offset)) => {
                                                                                                                                                                 app_state.is_transcoding = is_tx;
                                                                                                                                                                 app_state.seek_offset = offset;
                                                                                                                                                                 app_state.current_time = offset;
                                                                                                                                                                 app_state.last_known_time = offset;
                                                                                                                                                                 app_state.last_update_instant = std::time::Instant::now();
                                                                                                                                                                                                                              app_state.total_duration = probe.duration;
                                                                                                                                                                                                                              app_state.video_codec = probe.video_codec;
                                                                                                                                                                                                                              app_state.audio_codec = probe.audio_codec;
                                                                                                                                                                                                                              // app_state.user_paused = false;
                                                                                                                                                                                                                         },
                                                                                                                                                                                                                         Err(e) => log::error!("Failed to load next media: {}", e),
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                       }
                                                                                                                                  },
                                                                                                                                  _ => {}
                                                                                                                              }
                                  let tui_state = TuiState {
                                        status: format!("{:?}", current_status),
                                        current_time: app_state.current_time as f32,
                                        total_duration: app_state.total_duration.map(|d| d as f32),
                                        volume_level: app_state.volume_level,
                                        is_muted: app_state.is_muted,
                                        media_title: None,
                                        video_codec: app_state.video_codec.clone(),
                                        audio_codec: app_state.audio_codec.clone(),
                                        device_name: app_state.device_name.clone(),
                                        animation_frame: app_state.animation_frame,
                                        torrent_progress: app_state.torrent_progress,
                                    };
                                    let _ = tui.draw(&tui_state);
                              }
                         }
                     } else if event.namespace == RECEIVER_NAMESPACE {
                          if let Ok(ReceiverResponse::ReceiverStatus { status, .. }) = serde_json::from_str::<ReceiverResponse>(&event.payload) {
                              if let Some(vol) = status.volume {
                                  app_state.volume_level = vol.level;
                                  if let Some(muted) = vol.muted {
                                      app_state.is_muted = muted;
                                  }
                                   let tui_state = TuiState {
                                        status: format!("{:?}", current_status),
                                        current_time: app_state.current_time as f32,
                                        total_duration: app_state.total_duration.map(|d| d as f32),
                                        volume_level: app_state.volume_level,
                                        is_muted: app_state.is_muted,
                                        media_title: None,
                                        video_codec: app_state.video_codec.clone(),
                                        audio_codec: app_state.audio_codec.clone(),
                                        device_name: app_state.device_name.clone(),
                                        animation_frame: app_state.animation_frame,
                                        torrent_progress: app_state.torrent_progress,
                                    };
                                    let _ = tui.draw(&tui_state);
                              }
                          }
                     }
                }
                _ = animation_interval.tick() => {
                     app_state.animation_frame = app_state.animation_frame.wrapping_add(1);
                     if matches!(current_status, PlaybackStatus::Playing) {
                          app_state.current_time += 0.15;
                     }
                     
                     if let Some(handle) = &app_state.torrent_handle {
                         let stats = handle.stats();
                         let total = stats.total_bytes;
                         if total > 0 {
                             let pct = (stats.progress_bytes as f32 / total as f32) * 100.0;
                             app_state.torrent_progress = Some(pct);

                             // Auto-buffering logic
                             if let Some(total_dur) = app_state.total_duration {
                                 if total_dur > 0.0 {
                                     let played_pct = (app_state.current_time / total_dur) * 100.0;
                                     let margin = pct - played_pct as f32;
                                     let sid = app_state.media_session_id.unwrap_or(1);

                                     if margin < BUFFER_UNDERRUN_THRESHOLD && pct < 100.0 {
                                         if matches!(current_status, PlaybackStatus::Playing) {
                                             log::info!("Auto-buffering: Margin {:.1}% < Threshold. Pausing. Status: {:?} -> Buffering", margin, current_status);
                                             let _ = app.pause(sid).await;
                                             current_status = PlaybackStatus::Buffering;
                                         }
                                     } else if (margin > BUFFER_RESUME_THRESHOLD || pct >= 100.0)
                                         && matches!(current_status, PlaybackStatus::Buffering) {
                                             log::info!("Auto-buffering: Margin {:.1}% > Threshold. Resuming. Status: {:?} -> Playing", margin, current_status);
                                             let _ = app.play(sid).await;
                                             current_status = PlaybackStatus::Playing;
                                             app_state.last_update_instant = std::time::Instant::now();
                                         }
                                 }
                             }
                         }
                     }

                     let tui_state = TuiState {
                         status: format!("{:?}", current_status),
                         current_time: app_state.current_time as f32,
                         total_duration: app_state.total_duration.map(|d| d as f32),
                         volume_level: app_state.volume_level,
                         is_muted: app_state.is_muted,
                         media_title: None,
                         video_codec: app_state.video_codec.clone(),
                         audio_codec: app_state.audio_codec.clone(),
                         device_name: app_state.device_name.clone(),
                         animation_frame: app_state.animation_frame,
                         torrent_progress: app_state.torrent_progress,
                     };
                     let _ = tui.draw(&tui_state);
                }
            }
        }

        tui.stop();
        Ok(())
    }
}

// Helper functions (load_media, wait_for_torrent_download, setup_logging, get_local_ip)
// ... Copy from main.rs ...

pub async fn scan_devices() -> Result<(), Box<dyn Error>> {
    println!("Scanning for Google Cast devices (will run for 10s)...");
    let mut rx = discover_devices_async()?;

    let timeout = tokio::time::sleep(Duration::from_secs(10));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(device) = rx.recv() => {
                println!("Found Device:");
                println!("  Name: {}", device.friendly_name);
                println!("  Model: {}", device.model_name);
                println!("  IP: {}:{}", device.ip, device.port);
                println!("  UUID: {}", device.uuid);
                println!("--------------------------------");
            }
            _ = &mut timeout => {
                println!("Scan finished.");
                break;
            }
        }
    }
    Ok(())
}

pub async fn connect_only(ip: &str) -> Result<(), Box<dyn Error>> {
    println!("Connecting to {}...", ip);
    let client = CastClient::connect(ip, 8009).await?;
    println!("Connected! Waiting for events (Ctrl+C to exit)...");
    let mut rx = client.events();
    while let Ok(event) = rx.recv().await {
        println!("[{}] {}", event.namespace, event.payload);
    }
    Ok(())
}

pub async fn launch_app(ip: &str, app_id: &str) -> Result<(), Box<dyn Error>> {
    println!("Connecting to {}...", ip);
    let client = CastClient::connect(ip, 8009).await?;
    println!("Connecting to receiver...");
    client.connect_receiver().await?;
    println!("Launching app {}...", app_id);
    client.launch_app(app_id).await?;
    println!("App launched. Listening for status...");
    let mut rx = client.events();
    while let Ok(event) = rx.recv().await {
        println!("[{}] {}", event.namespace, event.payload);
    }
    Ok(())
}

fn setup_logging(path: &str) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::create(path)?;
    simplelog::WriteLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        file,
    )?;
    Ok(())
}

fn get_local_ip() -> Option<IpAddr> {
    use std::net::UdpSocket;
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip())
}

async fn load_media(
    app: &DefaultMediaReceiver,
    server: &StreamServer,
    source: &MediaSource,
    server_base: &str,
    start_time: f64,
    torrent_manager: &TorrentManager,
    tui: &TuiController,
    app_state: &mut AppState,
    probe_tx: Option<mpsc::Sender<InternalEvent>>,
) -> Result<(bool, MediaProbeResult, f64), Box<dyn Error>> {
    // ... Copy implementation from main.rs ...
    // Note: I already copied it above but didn't paste it all because of size.
    // I need to paste the FULL content of load_media and wait_for_torrent_download.
    // Since I can't see main.rs while writing, I'll rely on the previous read.
    // Wait, I'm writing the file NOW.
    
    // START OF COPIED FUNCTIONS
    let mut applied_seek_offset = 0.0;
    app_state.torrent_handle = None; 
    let (url, content_type, is_transcoding, probe) = match source {
        MediaSource::FilePath(path_str) => {
            let path = Path::new(path_str);
            let probe = match probe_media(path).await {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Warning: Probe failed: {}, assuming supported.", e);
                    MediaProbeResult {
                        video_codec: None,
                        audio_codec: None,
                        duration: None,
                        video_profile: None,
                        pix_fmt: None,
                    }
                }
            };

            if needs_transcoding(&probe) {
                applied_seek_offset = start_time;
                let config = TranscodeConfig {
                    input_path: path.to_path_buf(),
                    start_time,
                    target_video_codec: "libx264".to_string(),
                    target_audio_codec: "aac".to_string(),
                };
                let pipeline = spawn_ffmpeg(&config)?;
                server.set_transcode_output(pipeline).await;
                (
                    format!("{}/?t={}", server_base, start_time),
                    "video/mp4".to_string(),
                    true,
                    probe,
                )
            } else {
                server
                    .set_source(StreamSource::Static(path.to_path_buf()))
                    .await;
                (
                    server_base.to_string(),
                    get_mime_type(path).to_string(),
                    false,
                    probe,
                )
            }
        }
        MediaSource::Url(u) => (
            u.clone(),
            "video/mp4".to_string(),
            false,
            MediaProbeResult {
                video_codec: None,
                audio_codec: None,
                duration: None,
                video_profile: None,
                pix_fmt: None,
            },
        ),
        MediaSource::Magnet(uri) => {
            let init_state = TuiState {
                status: "METADATA FETCHING".to_string(),
                current_time: 0.0,
                total_duration: None,
                volume_level: app_state.volume_level,
                is_muted: app_state.is_muted,
                media_title: Some("Initializing Torrent...".to_string()),
                video_codec: None,
                audio_codec: None,
                device_name: app_state.device_name.clone(),
                animation_frame: app_state.animation_frame,
                torrent_progress: None,
            };
            let _ = tui.draw(&init_state);

            let info = torrent_manager.start_magnet(uri).await?;
            app_state.torrent_handle = Some(wait_for_torrent_download(&info, tui, app_state, probe_tx.clone()).await?);
            
            server
                .set_source(StreamSource::Growing {
                    path: info.path.clone(),
                    total_size: info.total_size,
                    handle: info.handle.clone(),
                    file_offset: info.file_offset,
                    piece_length: info.piece_length,
                })
                .await;

            let mime = get_mime_type(&info.path).to_string();
            (
                server_base.to_string(),
                mime,
                false,
                MediaProbeResult {
                    video_codec: None,
                    audio_codec: None,
                    duration: None,
                    video_profile: None,
                    pix_fmt: None,
                },
            )
        }
        MediaSource::TorrentFile(path_str) => {
            let info = torrent_manager.start_torrent_file(path_str).await?;
            app_state.torrent_handle = Some(wait_for_torrent_download(&info, tui, app_state, probe_tx.clone()).await?);

            server
                .set_source(StreamSource::Growing {
                    path: info.path.clone(),
                    total_size: info.total_size,
                    handle: info.handle.clone(),
                    file_offset: info.file_offset,
                    piece_length: info.piece_length,
                })
                .await;

            let mime = get_mime_type(&info.path).to_string();
            (
                server_base.to_string(),
                mime,
                false,
                MediaProbeResult {
                    video_codec: None,
                    audio_codec: None,
                    duration: None,
                    video_profile: None,
                    pix_fmt: None,
                },
            )
        }
    };

    let tracks = if let Some(sub_path_str) = &app_state.subtitles {
        let sub_path = Path::new(sub_path_str);
        if sub_path.exists() {
             server.set_subtitle(sub_path.to_path_buf()).await;
             Some(vec![MediaTrack {
                 track_id: 1,
                 track_type: "TEXT".to_string(),
                 track_content_id: Some(format!("{}/subtitle", server_base)),
                 track_content_type: Some("text/vtt".to_string()),
                 name: Some("Subtitle".to_string()),
                 language: Some("en".to_string()),
                 subtype: Some("SUBTITLES".to_string()),
             }])
        } else {
            None
        }
    } else {
        None
    };

    let media_info = MediaInformation {
        content_id: url,
        stream_type: "BUFFERED".to_string(),
        content_type,
        metadata: None,
        tracks,
    };

    let play_position = if is_transcoding {
        0.0
    } else {
        start_time as f32
    };

    let active_tracks = if media_info.tracks.is_some() {
        Some(vec![1])
    } else {
        None
    };

    app.load(media_info, true, play_position, active_tracks).await?;
    Ok((is_transcoding, probe, applied_seek_offset))
}

async fn wait_for_torrent_download(
    info: &TorrentStreamInfo,
    tui: &TuiController,
    app_state: &mut AppState,
    probe_tx: Option<mpsc::Sender<InternalEvent>>,
) -> Result<Arc<ManagedTorrent>, Box<dyn Error>> {
    app_state.torrent_file_name = Some(
        info.path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
    );
    let mut last_progress = 0.0;
    let mut stall_start = std::time::Instant::now();
    let mut probing_started = false;

    loop {
        let stats = info.handle.stats();
        let downloaded = stats.progress_bytes;
        let pct = if info.total_size > 0 {
            (downloaded as f32 / info.total_size as f32) * 100.0
        } else {
            0.0
        };

        // Start probing if downloaded enough bytes (e.g., > 5MB) and not already started
        if !probing_started && downloaded > 5 * 1024 * 1024 {
            if let Some(tx) = &probe_tx {
                probing_started = true;
                let tx = tx.clone();
                let path = info.path.clone();
                tokio::spawn(async move {
                    // Wait a bit more to ensure header is flushed to disk
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    match probe_media(&path).await {
                        Ok(probe) => {
                            let _ = tx.send(InternalEvent::ProbeCompleted {
                                duration: probe.duration,
                                video_codec: probe.video_codec,
                                audio_codec: probe.audio_codec,
                            }).await;
                        }
                        Err(e) => eprintln!("Background probe failed: {}", e),
                    }
                });
            }
        }

        if pct > last_progress {
            last_progress = pct;
            stall_start = std::time::Instant::now();
        } else if pct < 100.0 && stall_start.elapsed() > Duration::from_secs(30) {
            return Err("Torrent download stalled (no progress for 30s)".into());
        }

        app_state.torrent_progress = Some(pct);

        let tui_state = TuiState {
            status: "BUFFERING (TORRENT)".to_string(),
            current_time: 0.0,
            total_duration: app_state.total_duration.map(|d| d as f32),
            volume_level: app_state.volume_level,
            is_muted: app_state.is_muted,
            media_title: app_state.torrent_file_name.clone(),
            video_codec: None,
            audio_codec: None,
            device_name: app_state.device_name.clone(),
            animation_frame: app_state.animation_frame,
            torrent_progress: Some(pct),
        };
        let _ = tui.draw(&tui_state);

        if pct >= 100.0 || pct >= TORRENT_BUFFER_PCT_THRESHOLD || downloaded >= TORRENT_BUFFER_SIZE_THRESHOLD {
            break;
        }

        tokio::time::sleep(Duration::from_millis(200)).await;
        app_state.animation_frame = app_state.animation_frame.wrapping_add(1);
    }
    Ok(info.handle.clone())
}
