use async_trait::async_trait;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct CastEvent {
    pub namespace: String,
    pub payload: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CastError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TLS Error: {0}")]
    Tls(String),
    #[error("Protobuf Error: {0}")]
    Decode(#[from] prost::DecodeError),
    #[error("Protocol Error: {0}")]
    Protocol(String),
}

/// Core trait for interacting with a Cast Device
#[async_trait]
pub trait CastClient {
    /// Connect to a device at the given address
    async fn connect(host: &str, port: u16) -> Result<Self, CastError> where Self: Sized;

    /// Send a raw message to a specific namespace
    async fn send_message(&mut self, namespace: &str, source: &str, dest: &str, payload: &str) -> Result<(), CastError>;

    /// Launch an application by ID
    async fn launch_app(&mut self, app_id: &str) -> Result<(), CastError>;

    /// Subscribe to incoming events
    fn events(&self) -> mpsc::Receiver<CastEvent>;
    
    /// Keep the connection alive (usually internal, but exposed for manual control if needed)
    async fn heartbeat(&mut self) -> Result<(), CastError>;
}
