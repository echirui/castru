use castru::controllers::default_media_receiver::DefaultMediaReceiver;
use castru::controllers::media::{MediaSource, PlaybackStatus};
use castru::controllers::receiver::ReceiverController;
use castru::controllers::tui::{TuiCommand, TuiController};
use castru::discovery::CastDevice;
use castru::protocol::media::{MediaInformation, MediaResponse, NAMESPACE as MEDIA_NAMESPACE};
use castru::protocol::receiver::{ReceiverResponse, NAMESPACE as RECEIVER_NAMESPACE};
use castru::server::{get_mime_type, StreamServer, StreamSource};
use castru::torrent::{TorrentConfig, TorrentManager};
use castru::{discover_devices_async, CastClient};

use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::net::IpAddr;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Panic hook for TUI cleanup using crossterm
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        // Try to restore terminal
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::cursor::Show
        );
        default_hook(info);
    }));

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "scan" => {
            scan_devices().await?;
        }
        "cast" => {
            if args.len() < 3 {
                println!("Usage: castru cast [OPTIONS] <FILE_OR_URL> [FILE_OR_URL...]");
                println!("Options:");
                println!("  --ip <IP>      Connect to specific IP");
                println!("  --name <NAME>  Connect to device with specific Friendly Name");
                return Ok(());
            }

            let cast_args = &args[2..];
            let opts = parse_cast_args(cast_args);

            cast_media_playlist(opts).await?;
        }
        "launch" => {
            if args.len() < 4 {
                println!("Usage: castru launch <IP> <APP_ID>");
                return Ok(());
            }
            let ip = &args[2];
            let app_id = &args[3];
            launch_app(ip, app_id).await?;
        }
        "connect" => {
            if args.len() < 3 {
                println!("Usage: castru connect <IP>");
                return Ok(());
            }
            let ip = &args[2];
            connect_only(ip).await?;
        }
        _ => {
            print_usage();
        }
    }

    Ok(())
}

fn print_usage() {
    println!("Usage:");
    println!("  castru scan");
    println!("  castru cast [OPTIONS] <FILE_OR_URL> [FILE_OR_URL...]");
    println!("  castru connect <IP>");
    println!("  castru launch <IP> <APP_ID>");
}

struct CastOptions {
    target_ip: Option<String>,
    target_name: Option<String>,
    inputs: Vec<String>,
}

fn parse_cast_args(args: &[String]) -> CastOptions {
    let mut target_ip = None;
    let mut target_name = None;
    let mut inputs = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--ip" => {
                if i + 1 < args.len() {
                    target_ip = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--name" => {
                if i + 1 < args.len() {
                    target_name = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            val => {
                inputs.push(val.to_string());
            }
        }
        i += 1;
    }

    CastOptions {
        target_ip,
        target_name,
        inputs,
    }
}

async fn scan_devices() -> Result<(), Box<dyn Error>> {
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

async fn cast_media_playlist(opts: CastOptions) -> Result<(), Box<dyn Error>> {
    // 0. Prepare Playlist
    let mut playlist = VecDeque::new();
    for input in opts.inputs {
        if input.starts_with("magnet:?") {
            playlist.push_back(MediaSource::Magnet(input));
        } else if input.ends_with(".torrent") {
            playlist.push_back(MediaSource::TorrentFile(input));
        } else {
            let path = Path::new(&input);
            if path.exists() {
                playlist.push_back(MediaSource::FilePath(input));
            } else {
                playlist.push_back(MediaSource::Url(input));
            }
        }
    }

    if playlist.is_empty() {
        return Err("No valid media sources found".into());
    }

    // 1. Setup Server (lazy init)
    let mut server = StreamServer::new();
    let local_ip = get_local_ip().ok_or("Could not determine local IP")?;
    let server_url_base = server.start(&local_ip.to_string()).await?;
    println!("Server started at {}", server_url_base);

    // Setup Torrent Manager
    let torrent_manager = Arc::new(TorrentManager::new(TorrentConfig::default()).await?);

    // 2. Discover or Target device
    let device = if let Some(ip_str) = opts.target_ip {
        println!("Targeting specific IP: {}", ip_str);
        CastDevice {
            ip: ip_str.parse().map_err(|_| "Invalid IP address provided")?,
            port: 8009,
            friendly_name: "Direct Connect".to_string(),
            model_name: "Unknown".to_string(),
            uuid: "Unknown".to_string(),
        }
    } else {
        println!("Searching for Cast devices...");
        let mut rx = discover_devices_async()?;
        let matching_device;

        let timeout = tokio::time::sleep(Duration::from_secs(10));
        tokio::pin!(timeout);

        loop {
            tokio::select! {
                Some(d) = rx.recv() => {
                    if let Some(ref name) = opts.target_name {
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

    println!("Found {}", device.friendly_name);

    // 3. Connect and Launch
    println!("Connecting to {}...", device.ip);
    let client = CastClient::connect(&device.ip.to_string(), device.port).await?;
    let receiver_ctrl = ReceiverController::new(&client);
    client.connect_receiver().await?;

    let mut app = DefaultMediaReceiver::new(&client);
    app.launch().await?;
    println!("Default Media Receiver launched.");

    // 4. Start TUI
    let tui = TuiController::new();
    let mut tui_rx = tui.start()?;

    // TUI Loop
    let mut current_status = PlaybackStatus::Idle;

    const WATCHDOG_TIMEOUT_SEC: u64 = 5;

    struct AppState {
        is_transcoding: bool,
        seek_offset: f64,
        current_time: f64,
        last_known_time: f64,
        last_update_instant: std::time::Instant,
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
    }

    let mut app_state = AppState {
        is_transcoding: false,
        seek_offset: 0.0,
        current_time: 0.0,
        last_known_time: 0.0,
        last_update_instant: std::time::Instant::now(),
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
    };

    // Load first item
    if let Some(source) = playlist.get(0) {
        app_state.current_media_idx = 0;
        app_state.source = Some(source.clone());
        match load_media(
            &app,
            &server,
            source,
            &server_url_base,
            0.0,
            &torrent_manager,
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
            Err(e) => eprintln!("Failed to load media: {}", e),
        }
    }

    // Event Loop
    let mut events = client.events();
    use castru::controllers::tui::TuiState;
    let mut animation_interval = tokio::time::interval(Duration::from_millis(150));
    let mut watchdog_interval = tokio::time::interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = watchdog_interval.tick() => {
                // Watchdog: If playing but time hasn't advanced for X seconds, resume.
                if matches!(current_status, PlaybackStatus::Playing) {
                    if app_state.last_update_instant.elapsed() > Duration::from_secs(WATCHDOG_TIMEOUT_SEC) {
                        if let Some(source) = &app_state.source {
                            match load_media(&app, &server, source, &server_url_base, app_state.current_time, &torrent_manager).await {
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
                                current_status = PlaybackStatus::Paused;
                            },
                            _ => {
                                let _ = app.play(sid).await;
                                current_status = PlaybackStatus::Playing;
                                app_state.last_update_instant = std::time::Instant::now();
                            }
                        }
                    },
                    TuiCommand::Pause => {
                        let sid = app_state.media_session_id.unwrap_or(1);
                        let _ = app.pause(sid).await;
                        current_status = PlaybackStatus::Paused;
                    },
                    TuiCommand::Play => {
                         let sid = app_state.media_session_id.unwrap_or(1);
                         let _ = app.play(sid).await;
                         current_status = PlaybackStatus::Playing;
                         app_state.last_update_instant = std::time::Instant::now();
                    },
                    TuiCommand::Next => {
                        app_state.current_media_idx += 1;
                         if let Some(source) = playlist.get(app_state.current_media_idx) {
                             app_state.source = Some(source.clone());
                            if let Ok((is_tx, probe, offset)) = load_media(&app, &server, source, &server_url_base, 0.0, &torrent_manager).await {
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
                                if let Ok((is_tx, probe, offset)) = load_media(&app, &server, source, &server_url_base, 0.0, &torrent_manager).await {
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
                             if let Some(src) = &app_state.source {
                                 match load_media(&app, &server, src, &server_url_base, new_time, &torrent_manager).await {
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
                             if let Some(src) = &app_state.source {
                                 match load_media(&app, &server, src, &server_url_base, new_time, &torrent_manager).await {
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
                };
                let _ = tui.draw(&tui_state);
            }
            Ok(event) = events.recv() => {
                 if event.namespace == MEDIA_NAMESPACE {
                     if let Ok(MediaResponse::MediaStatus { status, .. }) = serde_json::from_str::<MediaResponse>(&event.payload) {
                          if let Some(s) = status.first() {
                              let reported_time = s.current_time as f64;
                              if app_state.is_transcoding {
                                  app_state.current_time = reported_time + app_state.seek_offset;
                              } else {
                                  app_state.current_time = reported_time;
                              }

                              // Update watchdog state
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
                                  "PLAYING" => current_status = PlaybackStatus::Playing,
                                  "PAUSED" => current_status = PlaybackStatus::Paused,
                                  "BUFFERING" => current_status = PlaybackStatus::Buffering,
                                  "IDLE" => {
                                       current_status = PlaybackStatus::Idle;
                                       if s.idle_reason.as_deref() == Some("ERROR") {
                                            println!("Watchdog: Detected Error status. Resuming...");
                                            if let Some(source) = &app_state.source {
                                                if let Ok((is_tx, probe, offset)) = load_media(&app, &server, source, &server_url_base, app_state.current_time, &torrent_manager).await {
                                                     app_state.is_transcoding = is_tx;
                                                     app_state.seek_offset = offset;
                                                     app_state.last_known_time = app_state.current_time;
                                                     app_state.last_update_instant = std::time::Instant::now();
                                                     app_state.total_duration = probe.duration;
                                                     app_state.video_codec = probe.video_codec;
                                                     app_state.audio_codec = probe.audio_codec;
                                                }
                                            }
                                       } else if s.idle_reason.as_deref() == Some("FINISHED") {
                                            app_state.current_media_idx += 1;
                                             if let Some(source) = playlist.get(app_state.current_media_idx) {
                                                 app_state.source = Some(source.clone());
                                                if let Ok((is_tx, probe, offset)) = load_media(&app, &server, source, &server_url_base, 0.0, &torrent_manager).await {
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
                      };
                      let _ = tui.draw(&tui_state);
                 }
            }
        }
    }

    tui.stop();
    Ok(())
}

use castru::transcode::{
    needs_transcoding, probe_media, spawn_ffmpeg, MediaProbeResult, TranscodeConfig,
};

async fn load_media(
    app: &DefaultMediaReceiver,
    server: &StreamServer,
    source: &MediaSource,
    server_base: &str,
    start_time: f64,
    torrent_manager: &TorrentManager,
) -> Result<(bool, MediaProbeResult, f64), Box<dyn Error>> {
    let mut applied_seek_offset = 0.0;
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
            println!("Resolving magnet link...");
            let info = torrent_manager.start_magnet(uri).await?;

            // Buffering Loop
            println!("Buffering torrent... (Wait for ~3% or 10MB)");
            loop {
                let stats = info.handle.stats();
                let downloaded = stats.progress_bytes;
                let pct = if info.total_size > 0 {
                    downloaded as f64 / info.total_size as f64
                } else {
                    0.0
                };

                if pct >= 0.03 || downloaded > 10 * 1024 * 1024 {
                    println!("Buffering complete. Starting playback.");
                    break;
                }

                tokio::time::sleep(Duration::from_millis(500)).await;
            }

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
            println!("Loading torrent file...");
            let info = torrent_manager.start_torrent_file(path_str).await?;

            // Buffering Loop (Duplicate logic, could refactor)
            println!("Buffering torrent... (Wait for ~3% or 10MB)");
            loop {
                let stats = info.handle.stats();
                let downloaded = stats.progress_bytes;
                let pct = if info.total_size > 0 {
                    downloaded as f64 / info.total_size as f64
                } else {
                    0.0
                };

                if pct >= 0.03 || downloaded > 10 * 1024 * 1024 {
                    println!("Buffering complete. Starting playback.");
                    break;
                }

                tokio::time::sleep(Duration::from_millis(500)).await;
            }

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

    let media_info = MediaInformation {
        content_id: url,
        stream_type: "BUFFERED".to_string(),
        content_type,
        metadata: None,
    };

    let play_position = if is_transcoding {
        0.0
    } else {
        start_time as f32
    };

    app.load(media_info, true, play_position).await?;
    Ok((is_transcoding, probe, applied_seek_offset))
}

fn get_local_ip() -> Option<IpAddr> {
    use std::net::UdpSocket;
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip())
}

async fn connect_only(ip: &str) -> Result<(), Box<dyn Error>> {
    println!("Connecting to {}...", ip);
    let client = CastClient::connect(ip, 8009).await?;
    println!("Connected! Waiting for events (Ctrl+C to exit)...");
    let mut rx = client.events();
    while let Ok(event) = rx.recv().await {
        println!("[{}] {}", event.namespace, event.payload);
    }
    Ok(())
}

async fn launch_app(ip: &str, app_id: &str) -> Result<(), Box<dyn Error>> {
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
