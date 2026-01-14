use castru::{CastClient, CastError};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: media_control <IP>");
        return Ok(());
    }
    let ip = &args[1];

    println!("Connecting to {}...", ip);
    let client = CastClient::connect(ip, 8009).await?;
    client.connect_receiver().await?;

    println!("Setting volume to 0.5...");
    client.set_volume(0.5).await?;

    // Note: To test seek/get_status, we need an active media session.
    // This example assumes one exists or just demonstrates volume.
    // If we had a transportId (from RECEIVER_STATUS) we could do:
    // client.media_get_status(transport_id, None).await?;

    Ok(())
}
