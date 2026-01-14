use crate::protocol::media::MediaStatus;
use crate::error::CastError;

/// High-level controller for media playback.
pub trait MediaController {
    /// Loads a media item (URL).
    async fn load(&mut self, url: &str, mime_type: &str) -> Result<(), CastError>;

    /// Toggles play/pause.
    async fn toggle_pause(&mut self) -> Result<(), CastError>;

    /// Seeks to a specific time.
    async fn seek(&mut self, time: f64) -> Result<(), CastError>;

    /// Stops playback.
    async fn stop(&mut self) -> Result<(), CastError>;

    /// Gets the current status of the media player.
    async fn get_status(&mut self) -> Result<Option<MediaStatus>, CastError>;
}
