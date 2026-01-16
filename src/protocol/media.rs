use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MediaRequest {
    #[serde(rename = "GET_STATUS")]
    GetStatus {
        #[serde(rename = "requestId")]
        request_id: i32,
        #[serde(rename = "mediaSessionId", skip_serializing_if = "Option::is_none")]
        media_session_id: Option<i32>,
    },
    #[serde(rename = "SEEK")]
    Seek {
        #[serde(rename = "requestId")]
        request_id: i32,
        #[serde(rename = "mediaSessionId")]
        media_session_id: i32,
        #[serde(rename = "currentTime")]
        current_time: f32,
        #[serde(rename = "resumeState", skip_serializing_if = "Option::is_none")]
        resume_state: Option<String>,
    },
    #[serde(rename = "LOAD")]
    Load {
        #[serde(rename = "requestId")]
        request_id: i32,
        #[serde(rename = "sessionId")]
        session_id: String, // Destination transportId usually receives this, but payload has session? No, load is sent to receiver.
        // Actually LOAD is sent to receiver-0 or the app? It's sent to the app.
        // The payload usually doesn't need sessionId if it's establishing one, but sometimes it does.
        // CastV2 spec says LOAD doesn't have mediaSessionId yet.
        // It has `sessionId` which is the *receiver* session? No.
        // Let's stick to standard fields.
        media: MediaInformation,
        #[serde(rename = "autoplay")]
        autoplay: bool,
        #[serde(rename = "currentTime")]
        current_time: f32,
        #[serde(rename = "activeTrackIds", skip_serializing_if = "Option::is_none")]
        active_track_ids: Option<Vec<i32>>,
    },
    #[serde(rename = "PLAY")]
    Play {
        #[serde(rename = "requestId")]
        request_id: i32,
        #[serde(rename = "mediaSessionId")]
        media_session_id: i32,
    },
    #[serde(rename = "PAUSE")]
    Pause {
        #[serde(rename = "requestId")]
        request_id: i32,
        #[serde(rename = "mediaSessionId")]
        media_session_id: i32,
    },
    #[serde(rename = "STOP")]
    Stop {
        #[serde(rename = "requestId")]
        request_id: i32,
        #[serde(rename = "mediaSessionId")]
        media_session_id: i32,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaInformation {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "streamType")]
    pub stream_type: String, // BUFFERED, LIVE, NONE
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MediaMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Vec<MediaTrack>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaTrack {
    #[serde(rename = "trackId")]
    pub track_id: i32, // Unique ID
    #[serde(rename = "type")]
    pub track_type: String, // TEXT, AUDIO, VIDEO
    #[serde(rename = "trackContentId", skip_serializing_if = "Option::is_none")]
    pub track_content_id: Option<String>, // URL of the track content
    #[serde(rename = "trackContentType", skip_serializing_if = "Option::is_none")]
    pub track_content_type: Option<String>, // MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "subtype", skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>, // SUBTITLES, CAPTIONS, DESCRIPTIONS, CHAPTERS, METADATA
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaMetadata {
    #[serde(rename = "metadataType")]
    pub metadata_type: i32, // 0: Generic, 1: Movie, 2: TV Show, 3: Music Track, 4: Photo
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub images: Option<Vec<Image>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub url: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MediaResponse {
    #[serde(rename = "MEDIA_STATUS")]
    MediaStatus {
        #[serde(rename = "requestId")]
        request_id: i32,
        status: Vec<MediaStatus>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaStatus {
    #[serde(rename = "mediaSessionId")]
    pub media_session_id: i32,
    #[serde(rename = "playbackRate")]
    pub playback_rate: f32,
    #[serde(rename = "playerState")]
    pub player_state: String,
    #[serde(rename = "currentTime")]
    pub current_time: f32,
    #[serde(rename = "supportedMediaCommands")]
    pub supported_media_commands: i32,
    pub volume: Option<Volume>,
    #[serde(rename = "idleReason")]
    pub idle_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Volume {
    pub level: Option<f32>,
    pub muted: Option<bool>,
}

pub const NAMESPACE: &str = "urn:x-cast:com.google.cast.media";
