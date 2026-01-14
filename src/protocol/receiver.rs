use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ReceiverRequest {
    #[serde(rename = "LAUNCH")]
    Launch {
        #[serde(rename = "appId")]
        app_id: String,
        #[serde(rename = "requestId")]
        request_id: i32,
    },
    #[serde(rename = "GET_STATUS")]
    GetStatus {
        #[serde(rename = "requestId")]
        request_id: i32,
    },
    #[serde(rename = "SET_VOLUME")]
    SetVolume {
        #[serde(rename = "requestId")]
        request_id: i32,
        volume: Volume,
    },
    #[serde(rename = "STOP")]
    Stop {
        #[serde(rename = "requestId")]
        request_id: i32,
        #[serde(rename = "sessionId")]
        session_id: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ReceiverResponse {
    #[serde(rename = "RECEIVER_STATUS")]
    ReceiverStatus {
        #[serde(rename = "requestId")]
        request_id: i32,
        status: ReceiverStatusData,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceiverStatusData {
    #[serde(default)]
    pub applications: Vec<Application>,
    pub volume: Option<Volume>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Application {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "transportId")]
    pub transport_id: String,
    #[serde(rename = "statusText")]
    pub status_text: String,
    #[serde(rename = "isIdleScreen")]
    pub is_idle_screen: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Volume {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
}

pub const NAMESPACE: &str = "urn:x-cast:com.google.cast.receiver";