use castru::{CastClient, discover_devices_async};
use castru::server::{StreamServer, get_mime_type};
use castru::controllers::default_media_receiver::DefaultMediaReceiver;
use castru::controllers::tui::{TuiController, TuiCommand};
use castru::controllers::media::{PlaybackStatus, MediaSource}; // Added MediaSource
use castru::protocol::media::MediaInformation;
use castru::discovery::CastDevice;
use std::env;
use std::error::Error;
use std::time::Duration;
use std::path::Path;
use std::net::IpAddr;
use std::collections::VecDeque;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
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
                   target_ip = Some(args[i+1].clone());
                   i += 1; 
                }
            }
            "--name" => {
                if i + 1 < args.len() {
                   target_name = Some(args[i+1].clone());
                   i += 1;
                }
            }
            val => {
                inputs.push(val.to_string());
            }
        }
        i += 1;
    }
    
    CastOptions { target_ip, target_name, inputs }
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
         let path = Path::new(&input);
         if path.exists() {
             playlist.push_back(MediaSource::FilePath(input));
         } else {
             playlist.push_back(MediaSource::Url(input));
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
    client.connect_receiver().await?;

    let mut app = DefaultMediaReceiver::new(&client);
    app.launch().await?;
    println!("Default Media Receiver launched.");

    // 4. Start TUI
    let tui = TuiController::new();
    let mut tui_rx = tui.start()?;
    
    // TUI Loop
    let mut current_status = PlaybackStatus::Idle; 
    
    struct AppState {
        is_transcoding: bool,
        current_time: f64,
        source: Option<MediaSource>,
        current_media_idx: usize,
    }
    
    let mut app_state = AppState { 
        is_transcoding: false, 
        current_time: 0.0, 
        source: None,
        current_media_idx: 0,
    };

    // Load first item
    if let Some(source) = playlist.front() {
        app_state.current_media_idx = 0;
        app_state.source = Some(source.clone());
        match load_media(&app, &server, source, &server_url_base, 0.0).await {
            Ok(is_tx) => {
                 app_state.is_transcoding = is_tx;
                 app_state.current_time = 0.0;
            },
            Err(e) => eprintln!("Failed to load media: {}", e),
        }
    }
    
    // Event Loop
    let mut events = client.events();
    use castru::protocol::media::{MediaResponse, NAMESPACE as MEDIA_NAMESPACE};

    loop {
        tokio::select! {
            Some(cmd) = tui_rx.recv() => {
                match cmd {
                    TuiCommand::Quit => break,
                    TuiCommand::Pause => {
                        let _ = app.pause(1).await;
                        current_status = PlaybackStatus::Paused;
                    },
                    TuiCommand::Play => {
                         let _ = app.play(1).await;
                         current_status = PlaybackStatus::Playing;
                    },
                    TuiCommand::Next => {
                        app_state.current_media_idx += 1;
                         if let Some(source) = playlist.get(app_state.current_media_idx) {
                             app_state.source = Some(source.clone());
                            if let Ok(is_tx) = load_media(&app, &server, source, &server_url_base, 0.0).await {
                                 app_state.is_transcoding = is_tx;
                                 app_state.current_time = 0.0;
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
                                if let Ok(is_tx) = load_media(&app, &server, source, &server_url_base, 0.0).await {
                                     app_state.is_transcoding = is_tx;
                                     app_state.current_time = 0.0;
                                }
                             }
                        }
                    },
                    TuiCommand::SeekForward(s) => {
                         let new_time = app_state.current_time + s as f64;
                         if app_state.is_transcoding {
                             if let Some(src) = &app_state.source {
                                 // Reload
                                 if let Ok(is_tx) = load_media(&app, &server, src, &server_url_base, new_time).await {
                                     app_state.is_transcoding = is_tx;
                                     // Don't reset time here, we want to display the target time
                                     app_state.current_time = new_time;
                                 }
                             }
                         } else {
                             let _ = app.seek(1, new_time as f32).await;
                             app_state.current_time = new_time; 
                         }
                    },
                    TuiCommand::SeekBackward(s) => {
                         let new_time = (app_state.current_time - s as f64).max(0.0);
                         if app_state.is_transcoding {
                             if let Some(src) = &app_state.source {
                                 // Reload
                                 if let Ok(is_tx) = load_media(&app, &server, src, &server_url_base, new_time).await {
                                     app_state.is_transcoding = is_tx;
                                     app_state.current_time = new_time;
                                 }
                             }
                         } else {
                             let _ = app.seek(1, new_time as f32).await;
                             app_state.current_time = new_time; 
                         }
                    },
                    _ => {}
                }
                // Update TUI
                let _ = tui.draw_status(&format!("{:?}", current_status), app_state.current_time as f32, None);
            }
            Ok(event) = events.recv() => {
                 if event.namespace == MEDIA_NAMESPACE {
                     if let Ok(MediaResponse::MediaStatus { status, .. }) = serde_json::from_str::<MediaResponse>(&event.payload) {
                          if let Some(s) = status.first() {
                              app_state.current_time = s.current_time as f64;
                              
                              match s.player_state.as_str() {
                                  "PLAYING" => current_status = PlaybackStatus::Playing,
                                  "PAUSED" => current_status = PlaybackStatus::Paused,
                                  "BUFFERING" => current_status = PlaybackStatus::Buffering,
                                  "IDLE" => {
                                       current_status = PlaybackStatus::Idle;
                                       if s.idle_reason.as_deref() == Some("FINISHED") {
                                            // Auto-Next
                                            app_state.current_media_idx += 1;
                                             if let Some(source) = playlist.get(app_state.current_media_idx) {
                                                 app_state.source = Some(source.clone());
                                                if let Ok(is_tx) = load_media(&app, &server, source, &server_url_base, 0.0).await {
                                                     app_state.is_transcoding = is_tx;
                                                     app_state.current_time = 0.0;
                                                }
                                             } else {
                                                 // End of playlist
                                                 // Keep idle
                                             }
                                       }
                                  },
                                  _ => {}
                              }
                              let _ = tui.draw_status(&format!("{:?}", current_status), app_state.current_time as f32, None);
                          }
                     }
                 }
            }
        }
    }

    tui.stop();
    Ok(())
}

use castru::transcode::{probe_media, needs_transcoding, spawn_ffmpeg, TranscodeConfig, MediaProbeResult};

async fn load_media(
    app: &DefaultMediaReceiver, 
    server: &StreamServer, 
    source: &MediaSource, 
    server_base: &str,
    start_time: f64
) -> Result<bool, Box<dyn Error>> {
     let (url, content_type, is_transcoding) = match source {
        MediaSource::FilePath(path_str) => {
            let path = Path::new(path_str);
            
            // Probe media
            let probe = match probe_media(path).await {
                Ok(p) => p,
                Err(e) => {
                     eprintln!("Warning: Probe failed: {}, assuming supported.", e);
                     MediaProbeResult { video_codec: None, audio_codec: None, duration: None }
                }
            };

            if needs_transcoding(&probe) {
                 println!("Transcoding needed (Video: {:?}, Audio: {:?})", probe.video_codec, probe.audio_codec);
                 let config = TranscodeConfig {
                     input_path: path.to_path_buf(),
                     start_time, // Use provided start_time
                     target_video_codec: "libx264".to_string(),
                     target_audio_codec: "aac".to_string(),
                 };
                 
                 let pipeline = spawn_ffmpeg(&config)?;
                 server.set_transcode_output(pipeline).await;
                 
                 (server_base.to_string(), "video/mp4".to_string(), true)
            } else {
                server.set_file(path.to_path_buf()).await;
                (server_base.to_string(), get_mime_type(path).to_string(), false)
            }
        },
        MediaSource::Url(u) => (u.clone(), "video/mp4".to_string(), false),
     };

    let media_info = MediaInformation {
        content_id: url,
        stream_type: "BUFFERED".to_string(),
        content_type,
        metadata: None,
    };
    
    // For non-transcoding, we use start_time in load command.
    // For transcoding, we handled it in ffmpeg -ss, so we can tell Cast to start at 0.0 relative to the stream?
    // OR we tell Cast to start at `start_time` but that might confuse it if the stream doesn't match?
    // If we pipe ffmpeg output, it is a new stream starting from 0 (bytes).
    // If we say start_time=30.0, Cast might expect timestamps to be 30.0+.
    // ffmpeg -ss output usually resets timestamps to 0 unless -copyts is used.
    // Let's assume safely 0.0 for transcoding stream start.
    
    let play_position = if is_transcoding { 0.0 } else { start_time as f32 };

    app.load(media_info, true, play_position).await?;
    Ok(is_transcoding)
}

// Simple heuristic to get local IP (for now binds to 0.0.0.0 effectively but proper IP needed for URL)
// This is actually tricky. We need the network interface IP that communicates with the Chromecast.
// Since we used mDNS, we might be able to infer, but mDNS library blocks.
// For MVP, allow picking first non-loopback?
fn get_local_ip() -> Option<IpAddr> {
    use std::net::UdpSocket;
    // Trick to get local IP by connecting to a public DNS (not actually sending data)
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