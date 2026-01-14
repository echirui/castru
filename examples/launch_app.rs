use castru::CastClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: launch_app <IP> <APP_ID>");
        return Ok(());
    }
    let ip = &args[1];
    let app_id = &args[2];

    println!("Connecting to {}...", ip);
    let client = CastClient::connect(ip, 8009).await?;
    
    println!("Connecting to receiver...");
    client.connect_receiver().await?;

    println!("Launching app {}...", app_id);
    client.launch_app(app_id).await?;

    let mut rx = client.events();
    println!("Listening for events...");
    
    while let Ok(event) = rx.recv().await {
        println!("Event [{}]: {}", event.namespace, event.payload);
        if event.payload.contains("RECEIVER_STATUS") {
            println!("Got status update!");
        }
    }

    Ok(())
}
