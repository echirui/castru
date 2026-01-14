use castru::{CastClient, CastError};
use castru::controllers::receiver::ReceiverController;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: platform_control <IP>");
        return Ok(());
    }
    let ip = &args[1];

    let client = CastClient::connect(ip, 8009).await?;
    client.connect_receiver().await?; // Connect to receiver-0

    let receiver_ctrl = ReceiverController::new(&client);

    println!("Getting status...");
    receiver_ctrl.get_status().await?;

    let mut rx = client.events();
    
    // Listen for one status message to see apps
    if let Ok(event) = rx.recv().await {
        println!("Event: {}", event.payload);
        // In real app, parse event.payload -> ReceiverResponse -> get sessionId -> receiver_ctrl.stop_app(sessionId)
    }

    Ok(())
}
