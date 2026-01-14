use thiserror::Error;

#[derive(Error, Debug)]
pub enum CastError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TLS Error: {0}")]
    Tls(String),
    #[error("Protobuf Decode Error: {0}")]
    Decode(#[from] prost::DecodeError),
    #[error("Protobuf Encode Error: {0}")]
    Encode(#[from] prost::EncodeError),
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Protocol Error: {0}")]
    Protocol(String),
    #[error("Streaming Error: {0}")]
    Streaming(String),
    #[error("TUI Error: {0}")]
    Tui(String),
    #[error("Probe Error: {0}")]
    Probe(String),
    #[error("Transcoding Error: {0}")]
    Transcoding(String),
}