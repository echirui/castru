use castru::controllers::media::MediaController;
use castru::protocol::media::MediaInformation;
use castru::{CastClient, CastError};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: full_media <IP> <MEDIA_URL>");
        return Ok(());
    }
    let ip = &args[1];
    let media_url = &args[2];

    let client = CastClient::connect(ip, 8009).await?;
    client.connect_receiver().await?;

    // In a real flow, we'd launch an app (e.g. Default Media Receiver CC1AD845),
    // get the transportId, and then use MediaController.
    // For this example, we assume we have a transportId (user must provide or we assume one?)
    // Actually, launch_app is needed.
    // Let's use CastClient methods for launch for now (as ReceiverController is separate).
    client.launch_app("CC1AD845").await?; // Default Media Receiver

    println!("Waiting for app launch and status...");
    sleep(Duration::from_secs(5)).await;

    // We need to find the transportId from events.
    // This example is simplified; in reality we'd parse events.
    // Assuming we found it:
    let transport_id = "web-1"; // Placeholder

    let media_ctrl = MediaController::new(&client, transport_id);

    let media_info = MediaInformation {
        content_id: media_url.to_string(),
        stream_type: "BUFFERED".to_string(),
        content_type: "video/mp4".to_string(),
        metadata: None,
    };

    println!("Loading media...");
    media_ctrl.load(media_info, true, 0.0).await?;

    Ok(())
}
