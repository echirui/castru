use castru::controllers::default_media_receiver::DefaultMediaReceiver;
use castru::protocol::media::MediaInformation;
use castru::CastClient;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: full_lifecycle <IP> <MEDIA_URL>");
        return Ok(());
    }
    let ip = &args[1];
    let media_url = &args[2];

    let client = CastClient::connect(ip, 8009).await?;
    // client.connect_receiver().await?; // Connect receiver is handled inside launch mostly, but the initial connection is needed.
    // Actually CastClient::connect establishes TLS.
    // connect_receiver handles CONNECT to "receiver-0"

    // We should probably ensure the client is fully ready.
    client.connect_receiver().await?;

    println!("Connected to Cast device at {}.", ip);

    let mut app = DefaultMediaReceiver::new(&client);

    println!("Launching Default Media Receiver...");
    app.launch().await?;
    println!("App launched and connected!");

    let media_info = MediaInformation {
        content_id: media_url.to_string(),
        stream_type: "BUFFERED".to_string(),
        content_type: "video/mp4".to_string(),
        metadata: None,
        tracks: None,
    };

    println!("Loading media: {}", media_url);
    app.load(media_info, true, 0.0, None).await?;
    println!("Media loaded! Playing for 10 seconds...");

    sleep(Duration::from_secs(10)).await;

    // Note: session_id 1 is a common default for the first item, but strictly should be parsed from status.
    println!("Pausing...");
    if let Err(e) = app.pause(1).await {
        println!("Failed to pause (might be wrong session ID): {}", e);
    }

    sleep(Duration::from_secs(3)).await;

    println!("Resuming...");
    if let Err(e) = app.play(1).await {
        println!("Failed to play: {}", e);
    }

    sleep(Duration::from_secs(5)).await;

    println!("Stopping...");
    if let Err(e) = app.stop(1).await {
        println!("Failed to stop: {}", e);
    }

    Ok(())
}
