use castru::controllers::default_media_receiver::DefaultMediaReceiver;
use castru::controllers::media::{MediaSource, PlaybackStatus};
use castru::controllers::receiver::ReceiverController;
use castru::controllers::tui::{TuiCommand, TuiController, TuiState};
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
use librqbit::ManagedTorrent;

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
    torrent_progress: Option<f32>,
    torrent_file_name: Option<String>,
    torrent_handle: Option<Arc<ManagedTorrent>>,
    subtitles: Option<String>,
}

const TORRENT_BUFFER_PCT_THRESHOLD: f32 = 3.0;
const TORRENT_BUFFER_SIZE_THRESHOLD: u64 = 10 * 1024 * 1024; // 10MB

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
    println!();
    println!("Options for 'cast':");
    println!("  --ip <IP>      Connect to specific IP");
    println!("  --name <NAME>  Connect to device with specific Friendly Name");
    println!("  --log <FILE>   Output logs to specific file");
    println!("  --myip <IP>    Specify local interface IP to bind to");
    println!("  --port <PORT>  Specify internal server port");
    println!("  --subtitles <FILE>  Load sidecar subtitle file");
    println!("  --volume <0.0-1.0>  Set initial volume");
    println!("  --loop         Loop the playlist");
    println!("  --quiet        Suppress non-critical output");
}

struct CastOptions {
    target_ip: Option<String>,
    target_name: Option<String>,
    log_file: Option<String>,
    inputs: Vec<String>,
    myip: Option<String>,
    port: Option<u16>,
    subtitles: Option<String>,
    volume: Option<f32>,
    loop_playlist: bool,
    quiet: bool,
}

fn parse_cast_args(args: &[String]) -> CastOptions {
    let mut target_ip = None;
    let mut target_name = None;
    let mut log_file = None;
    let mut inputs = Vec::new();
    let mut myip = None;
    let mut port = None;
    let mut subtitles = None;
    let mut volume = None;
    let mut loop_playlist = false;
    let mut quiet = false;

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
            "--log" => {
                if i + 1 < args.len() {
                    log_file = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--myip" => {
                if i + 1 < args.len() {
                    myip = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--port" => {
                if i + 1 < args.len() {
                    if let Ok(p) = args[i + 1].parse::<u16>() {
                        port = Some(p);
                    }
                    i += 1;
                }
            }
            "--subtitles" => {
                if i + 1 < args.len() {
                    subtitles = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--volume" => {
                if i + 1 < args.len() {
                    if let Ok(v) = args[i + 1].parse::<f32>() {
                        volume = Some(v);
                    }
                    i += 1;
                }
            }
            "--loop" => {
                loop_playlist = true;
            }
            "--quiet" => {
                quiet = true;
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
        log_file,
        inputs,
        myip,
        port,
        subtitles,
        volume,
        loop_playlist,
        quiet,
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
    // Setup Logging if requested
    if let Some(ref log_path) = opts.log_file {
        setup_logging(log_path)?;
    }

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
    let bind_ip = if let Some(ip) = &opts.myip {
        ip.clone()
    } else {
        get_local_ip().ok_or("Could not determine local IP")?.to_string()
    };
    let server_url_base = server.start(&bind_ip, opts.port).await?;
    log::info!("Server started at {}", server_url_base);

    // Setup Torrent Manager
    let torrent_manager = Arc::new(TorrentManager::new(TorrentConfig::default()).await?);

    // 2. Discover or Target device
    let device = if let Some(ip_str) = opts.target_ip {
        if !opts.quiet { println!("Targeting specific IP: {}", ip_str); }
        CastDevice {
            ip: ip_str.parse().map_err(|_| "Invalid IP address provided")?,
            port: 8009,
            friendly_name: "Direct Connect".to_string(),
            model_name: "Unknown".to_string(),
            uuid: "Unknown".to_string(),
        }
    } else {
        if !opts.quiet { println!("Searching for Cast devices..."); }
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
    
    if !opts.quiet { println!("Found {}", device.friendly_name); }

    // 3. Connect and Launch
    if !opts.quiet { println!("Connecting to {}...", device.ip); }
    let mut client = CastClient::connect(&device.ip.to_string(), device.port).await?;
    let mut receiver_ctrl = ReceiverController::new(&client);
    client.connect_receiver().await?;

    let mut app = DefaultMediaReceiver::new(&client);
    app.launch().await?;
    log::info!("Default Media Receiver launched.");

    // Apply volume if specified
    if let Some(vol) = opts.volume {
        let _ = receiver_ctrl.set_volume(vol).await;
    }

    // 4. Start TUI
    let tui = TuiController::new();
    let mut tui_rx = tui.start()?;

    // TUI Loop
    let mut current_status = PlaybackStatus::Idle;

    const WATCHDOG_TIMEOUT_SEC: u64 = 30;





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
        torrent_progress: None,
        torrent_file_name: None,
        torrent_handle: None,
        subtitles: opts.subtitles.clone(),
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
            &tui,
            &mut app_state,
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
    let mut events = client.events();
    let mut animation_interval = tokio::time::interval(Duration::from_millis(150));
    let mut watchdog_interval = tokio::time::interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = watchdog_interval.tick() => {
                // Watchdog: If playing but time hasn't advanced for X seconds, resume.
                if matches!(current_status, PlaybackStatus::Playing) {
                    if app_state.last_update_instant.elapsed() > Duration::from_secs(WATCHDOG_TIMEOUT_SEC) {
                        if let Some(source) = app_state.source.clone() {
                            let curr_time = app_state.current_time;
                            match load_media(&app, &server, &source, &server_url_base, curr_time, &torrent_manager, &tui, &mut app_state).await {
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
                            if let Ok((is_tx, probe, offset)) = load_media(&app, &server, source, &server_url_base, 0.0, &torrent_manager, &tui, &mut app_state).await {
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
                                if let Ok((is_tx, probe, offset)) = load_media(&app, &server, source, &server_url_base, 0.0, &torrent_manager, &tui, &mut app_state).await {
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
                                 match load_media(&app, &server, &src, &server_url_base, new_time, &torrent_manager, &tui, &mut app_state).await {
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
                                 match load_media(&app, &server, &src, &server_url_base, new_time, &torrent_manager, &tui, &mut app_state).await {
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
                        let _ = app.pause(sid).await; // Simulating stop with pause if no stop available
                        current_status = PlaybackStatus::Finished;
                        app_state.torrent_handle = None;
                        app_state.torrent_progress = None;
                    },
                    TuiCommand::Reconnect => {
                        current_status = PlaybackStatus::Reconnecting;
                        // Force a draw to show RECONNECTING immediately
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
                                            log::warn!("Watchdog: Detected Error status. Resuming...");
                                            if let Some(source) = app_state.source.clone() {
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
                                                )
                                                .await
                                                {
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
                                            let next_idx = app_state.current_media_idx + 1;
                                            if next_idx < playlist.len() || opts.loop_playlist {
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
                                                        },
                                                        Err(e) => log::error!("Failed to load next media: {}", e),
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
                 
                 // Update torrent progress if downloading in background
                 if let Some(handle) = &app_state.torrent_handle {
                     let stats = handle.stats();
                     let total = stats.total_bytes;
                     if total > 0 {
                         app_state.torrent_progress = Some((stats.progress_bytes as f32 / total as f32) * 100.0);
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
    tui: &TuiController,
    app_state: &mut AppState,
) -> Result<(bool, MediaProbeResult, f64), Box<dyn Error>> {
    let mut applied_seek_offset = 0.0;
    app_state.torrent_handle = None; // Reset by default
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
            let info = torrent_manager.start_magnet(uri).await?;
            app_state.torrent_handle = Some(wait_for_torrent_download(&info, tui, app_state).await?);
            
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
            app_state.torrent_handle = Some(wait_for_torrent_download(&info, tui, app_state).await?);

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

    // Handle Subtitles
    let tracks = if let Some(sub_path_str) = &app_state.subtitles {
        let sub_path = Path::new(sub_path_str);
        if sub_path.exists() {
             server.set_subtitle(sub_path.to_path_buf()).await;
             use castru::protocol::media::MediaTrack;
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

use castru::torrent::TorrentStreamInfo;

async fn wait_for_torrent_download(
    info: &TorrentStreamInfo,
    tui: &TuiController,
    app_state: &mut AppState,
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

    loop {
        let stats = info.handle.stats();
        let downloaded = stats.progress_bytes;
        let pct = if info.total_size > 0 {
            (downloaded as f32 / info.total_size as f32) * 100.0
        } else {
            0.0
        };

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
            total_duration: None,
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

        // Early exit for streaming (User Story 1)
        if pct >= 100.0 || pct >= TORRENT_BUFFER_PCT_THRESHOLD || downloaded >= TORRENT_BUFFER_SIZE_THRESHOLD {
            break;
        }

        tokio::time::sleep(Duration::from_millis(200)).await;
        app_state.animation_frame = app_state.animation_frame.wrapping_add(1);
    }
    // Note: Do NOT reset torrent_progress here, as we want to continue showing it during playback
    Ok(info.handle.clone())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cast_args_basic() {
        let args = vec![
            "cast".to_string(),
            "--myip".to_string(),
            "192.168.1.50".to_string(),
            "--port".to_string(),
            "8888".to_string(),
            "--subtitles".to_string(),
            "sub.vtt".to_string(),
            "--volume".to_string(),
            "0.5".to_string(),
            "--loop".to_string(),
            "--quiet".to_string(),
            "video.mp4".to_string(),
        ];

        let opts = parse_cast_args(&args);

        assert_eq!(opts.myip, Some("192.168.1.50".to_string()));
        assert_eq!(opts.port, Some(8888));
        assert_eq!(opts.subtitles, Some("sub.vtt".to_string()));
        assert_eq!(opts.volume, Some(0.5));
        assert!(opts.loop_playlist);
        assert!(opts.quiet);
        assert_eq!(opts.inputs.len(), 1);
        assert_eq!(opts.inputs[0], "video.mp4");
    }

    #[test]
    fn test_parse_cast_args_defaults() {
        let args = vec![
            "cast".to_string(),
            "video.mp4".to_string(),
        ];

        let opts = parse_cast_args(&args);

        assert_eq!(opts.myip, None);
        assert_eq!(opts.port, None);
        assert_eq!(opts.subtitles, None);
        assert_eq!(opts.volume, None);
        assert!(!opts.loop_playlist);
        assert!(!opts.quiet);
    }
}
