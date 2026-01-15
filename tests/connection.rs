use castru::CastClient;
use tokio::time::{sleep, Duration};

#[tokio::test]
#[ignore]
async fn test_real_connection() {
    let ip = std::env::var("CAST_DEVICE_IP").unwrap_or("192.168.1.100".to_string());
    let client = CastClient::connect(&ip, 8009).await;
    match client {
        Ok(client) => {
            println!("Connected successfully! Sending CONNECT to receiver...");
            let res = client.connect_receiver().await;
            match res {
                Ok(_) => println!("Sent CONNECT message."),
                Err(e) => panic!("Failed to send CONNECT: {}", e),
            }
            sleep(Duration::from_secs(5)).await;
            println!("Done.");
        }
        Err(e) => panic!("Connection failed: {}", e),
    }
}
