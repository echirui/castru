use crate::client::CastClient;
use crate::error::CastError;
use crate::proto::CastMessage;
use crate::protocol::receiver::{self, ReceiverRequest};
use crate::protocol::connection::{self, Connection};

/// Controller for the Receiver namespace (Platform).
///
/// Handles launching apps, stopping apps, and checking device status.
pub struct ReceiverController {
    client: CastClient,
}

impl ReceiverController {
    /// Creates a new ReceiverController.
    pub fn new(client: &CastClient) -> Self {
        Self {
            client: client.clone(),
        }
    }

    /// Launches an application by its App ID.
    pub async fn launch_app(&self, app_id: &str) -> Result<(), CastError> {
        self.client.launch_app(app_id).await
    }

    /// Stops an application session.
    pub async fn stop_app(&self, session_id: &str) -> Result<(), CastError> {
        let request_id = 1; // TODO: Randomize
        let msg = ReceiverRequest::Stop {
            request_id,
            session_id: session_id.to_string(),
        };
        self.send_receiver_request(msg).await
    }

    /// Requests the current status of the receiver (volume, running apps).
    pub async fn get_status(&self) -> Result<(), CastError> {
        let request_id = 1;
        let msg = ReceiverRequest::GetStatus {
            request_id,
        };
        self.send_receiver_request(msg).await
    }
    /// Joins an existing application session by connecting to its transport ID.
    pub async fn join_session(&self, transport_id: &str) -> Result<(), CastError> {
        let msg = Connection::Connect;
        let payload = serde_json::to_string(&msg).unwrap();
        
        let cast_msg = CastMessage {
            protocol_version: 0,
            source_id: "sender-0".to_string(),
            destination_id: transport_id.to_string(),
            namespace: connection::NAMESPACE.to_string(),
            payload_type: 0,
            payload_utf8: Some(payload),
            payload_binary: None,
        };
        self.client.send_message(cast_msg).await
    }
    pub async fn set_volume(&self, level: f32) -> Result<(), CastError> {
        let request_id = 1;
        let msg = ReceiverRequest::SetVolume {
            request_id,
            volume: receiver::Volume { level: Some(level), muted: None },
        };
        self.send_receiver_request(msg).await
    }

    pub async fn set_mute(&self, muted: bool) -> Result<(), CastError> {
        let request_id = 1;
        let msg = ReceiverRequest::SetVolume {
            request_id,
            volume: receiver::Volume { level: None, muted: Some(muted) },
        };
        self.send_receiver_request(msg).await
    }


    async fn send_receiver_request(&self, request: ReceiverRequest) -> Result<(), CastError> {
        let payload = serde_json::to_string(&request).unwrap();
        let msg = CastMessage {
            protocol_version: 0,
            source_id: "sender-0".to_string(),
            destination_id: "receiver-0".to_string(),
            namespace: receiver::NAMESPACE.to_string(),
            payload_type: 0,
            payload_utf8: Some(payload),
            payload_binary: None,
        };
        self.client.send_message(msg).await
    }
}
