use castru::CastClient;

mod common;
use common::mock_device::MockDevice;

#[tokio::test]
async fn test_integration_connect() {
    let device = MockDevice::start().await;
    let port = device.port();

    // Host must match the cert name "localhost" or verify logic must ignore it.
    // Our CastClient uses NoCertificateVerification, so "localhost" or IP should work.
    let client = CastClient::connect("127.0.0.1", port).await;
    
    assert!(client.is_ok());
    let client = client.unwrap();

    // Attempt to send a message (e.g. CONNECT to receiver)
    let res = client.connect_receiver().await;
    // Since MockDevice just reads and discards, this write should succeed.
    assert!(res.is_ok());
}
