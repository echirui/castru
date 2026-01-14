use std::path::Path;
use async_trait::async_trait;
use crate::error::CastError;

#[async_trait]
pub trait StreamServer {
    /// Starts the HTTP server on an available port.
    /// Returns the base URL (e.g., "http://192.168.1.5:4321").
    async fn start(&mut self) -> Result<String, CastError>;

    /// Stops the server.
    async fn stop(&mut self) -> Result<(), CastError>;

    /// Sets the file to be served.
    fn set_file(&mut self, path: &Path, mime_type: &str) -> Result<(), CastError>;

    /// Returns the currently assigned port.
    fn port(&self) -> u16;
}
