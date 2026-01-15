use crate::client::CastClient;
use crate::error::CastError;
use crate::proto::CastMessage;
use crate::protocol::media::{self, MediaRequest, MediaInformation};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum MediaSource {
    Url(String),
    FilePath(String),
}

#[derive(Debug)]
pub struct Playlist {
    pub queue: VecDeque<MediaSource>,
    pub repeat: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlaybackStatus {
    Idle,
    Buffering,
    Playing,
    Paused,
    Finished,
}

/// Controller for the Media namespace.
///
/// Handles loading media, playback controls (play/pause/seek), and volume for a specific session.
pub struct MediaController {
    client: CastClient,
    transport_id: String,
}

impl MediaController {
    /// Creates a new MediaController for a specific transport ID (session).
    pub fn new(client: &CastClient, transport_id: &str) -> Self {
        Self {
            client: client.clone(),
            transport_id: transport_id.to_string(),
        }
    }

    /// Loads media content.
    pub async fn load(&self, media: MediaInformation, autoplay: bool, current_time: f32) -> Result<(), CastError> {
        let request_id = 1; // TODO: Randomize
        let msg = MediaRequest::Load {
            request_id,
            session_id: self.transport_id.clone(),
            media,
            autoplay,
            current_time,
        };
        self.send_media_request(msg).await
    }

    pub async fn play(&self, media_session_id: i32) -> Result<(), CastError> {
        let request_id = 1;
        let msg = MediaRequest::Play {
            request_id,
            media_session_id,
        };
        self.send_media_request(msg).await
    }

    pub async fn pause(&self, media_session_id: i32) -> Result<(), CastError> {
        let request_id = 1;
        let msg = MediaRequest::Pause {
            request_id,
            media_session_id,
        };
        self.send_media_request(msg).await
    }

    /// Toggles between Play and Pause based on the current status.
    pub async fn toggle_pause(&self, media_session_id: i32, current_status: &PlaybackStatus) -> Result<(), CastError> {
        match current_status {
             PlaybackStatus::Playing => self.pause(media_session_id).await,
             PlaybackStatus::Paused | PlaybackStatus::Idle => self.play(media_session_id).await,
             _ => Ok(()),
        }
    }

    pub async fn stop(&self, media_session_id: i32) -> Result<(), CastError> {
        let request_id = 1;
        let msg = MediaRequest::Stop {
            request_id,
            media_session_id,
        };
        self.send_media_request(msg).await
    }

    pub async fn seek(&self, media_session_id: i32, time: f32) -> Result<(), CastError> {
        let request_id = 1;
        let msg = MediaRequest::Seek {
            request_id,
            media_session_id,
            current_time: time,
            resume_state: None,
        };
        self.send_media_request(msg).await
    }

    pub async fn set_volume(&self, media_session_id: i32, level: f32) -> Result<(), CastError> {
         Ok(())
    }
    
    pub async fn set_stream_mute(&self, media_session_id: i32, muted: bool) -> Result<(), CastError> {
         Ok(())
    }

    async fn send_media_request(&self, request: MediaRequest) -> Result<(), CastError> {
        let payload = serde_json::to_string(&request).unwrap();
        let msg = CastMessage {
            protocol_version: 0,
            source_id: "sender-0".to_string(),
            destination_id: self.transport_id.clone(),
            namespace: media::NAMESPACE.to_string(),
            payload_type: 0,
            payload_utf8: Some(payload),
            payload_binary: None,
        };
        self.client.send_message(msg).await
    }
}
