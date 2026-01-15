use crate::codec::CastCodec;
use crate::error::CastError;
use crate::proto::CastMessage;
use crate::protocol::connection::{self, Connection};
use crate::protocol::heartbeat::{self, Heartbeat};
use crate::protocol::media::{self, MediaRequest};
use crate::protocol::receiver::{self, ReceiverRequest, Volume};
use crate::tls::create_tls_connector;
use bytes::BytesMut;
use rustls::ServerName;
use std::convert::TryFrom;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc};
use tokio::time::{self, Duration};

/// Event received from the Cast device
#[derive(Debug, Clone)]
pub struct CastEvent {
    pub namespace: String,
    pub payload: String,
}

/// A client for communicating with a Google Cast device.
///
/// Handles TLS connection, message framing, heartbeats, and event dispatching.
#[derive(Clone)]
pub struct CastClient {
    command_tx: mpsc::Sender<CastMessage>,
    event_tx: broadcast::Sender<CastEvent>,
}

use crate::controllers::media::MediaController;
use crate::controllers::receiver::ReceiverController;

impl CastClient {
    pub fn receiver(&self) -> ReceiverController {
        ReceiverController::new(self)
    }

    pub fn media(&self, transport_id: &str) -> MediaController {
        MediaController::new(self, transport_id)
    }

    /// Connects to a Cast device at the given host (IP) and port (default 8009).
    ///
    /// Establishes a TLS connection and starts a background task for heartbeats and message reading.
    pub async fn connect(host: &str, port: u16) -> Result<Self, CastError> {
        let (command_tx, mut command_rx) = mpsc::channel::<CastMessage>(32);
        let (event_tx, _) = broadcast::channel::<CastEvent>(32);
        let event_tx_clone = event_tx.clone();

        let host_owned = host.to_string();

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(5));
            let mut retry_delay = Duration::from_secs(1);

            loop {
                // Connection attempt loop
                let addr = format!("{}:{}", host_owned, port);
                let tcp_stream_res = TcpStream::connect(&addr).await;

                if let Err(e) = tcp_stream_res {
                    eprintln!("Connection failed: {}. Retrying in {:?}...", e, retry_delay);
                    time::sleep(retry_delay).await;
                    retry_delay = std::cmp::min(retry_delay * 2, Duration::from_secs(30));
                    continue;
                }
                let tcp_stream = tcp_stream_res.unwrap();

                let connector = create_tls_connector();
                // Re-parse domain (should be cheap)
                let domain = match host_owned.parse::<std::net::IpAddr>() {
                    Ok(ip) => ServerName::IpAddress(ip),
                    Err(_) => ServerName::try_from(host_owned.as_str()).unwrap(), // Should handle error better
                };

                let stream_res = connector.connect(domain, tcp_stream).await;
                if let Err(e) = stream_res {
                    eprintln!(
                        "TLS Handshake failed: {}. Retrying in {:?}...",
                        e, retry_delay
                    );
                    time::sleep(retry_delay).await;
                    retry_delay = std::cmp::min(retry_delay * 2, Duration::from_secs(30));
                    continue;
                }
                let stream = stream_res.unwrap();
                let (mut reader, mut writer) = tokio::io::split(stream);
                let mut buf = BytesMut::with_capacity(1024);

                println!("Connected to {}", host_owned);
                retry_delay = Duration::from_secs(1); // Reset on success

                // Inner loop for active connection
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            let ping = Heartbeat::Ping;
                            let payload = serde_json::to_string(&ping).unwrap();
                            let msg = CastMessage {
                                protocol_version: 0,
                                source_id: "sender-0".to_string(),
                                destination_id: "receiver-0".to_string(),
                                namespace: heartbeat::NAMESPACE.to_string(),
                                payload_type: 0,
                                payload_utf8: Some(payload),
                                payload_binary: None,
                            };

                            let mut encode_buf = BytesMut::new();
                            if CastCodec::encode(&msg, &mut encode_buf).is_ok() {
                                 if let Err(_e) = writer.write_all(&encode_buf).await {
                                     break; // Reconnect
                                 }
                            }
                        }
                        Some(msg) = command_rx.recv() => {
                            let mut encode_buf = BytesMut::new();
                            if CastCodec::encode(&msg, &mut encode_buf).is_ok() {
                                 if let Err(_e) = writer.write_all(&encode_buf).await {
                                     break; // Reconnect
                                 }
                            }
                        }
                        res = reader.read_buf(&mut buf) => {
                             match res {
                                 Ok(0) => break, // EOF, Reconnect
                                 Ok(_) => {
                                     loop {
                                         match CastCodec::decode(&mut buf) {
                                             Ok(Some(msg)) => {
                                                 if let Some(payload) = &msg.payload_utf8 {
                                                     let event = CastEvent {
                                                         namespace: msg.namespace.clone(),
                                                         payload: payload.clone(),
                                                     };
                                                     let _ = event_tx_clone.send(event);
                                                 }
                                             }
                                             Ok(None) => break,
                                             Err(_e) => {
                                                 break;
                                             },
                                         }
                                     }
                                 }
                                 Err(_e) => break, // Reconnect
                             }
                        }
                    }
                }
                println!("Disconnected. Retrying in 5s...");
                time::sleep(Duration::from_secs(5)).await;
            }
        });

        Ok(Self {
            command_tx,
            event_tx,
        })
    }

    pub async fn send_message(&self, msg: CastMessage) -> Result<(), CastError> {
        self.command_tx
            .send(msg)
            .await
            .map_err(|_| CastError::Protocol("Channel closed".into()))
    }

    pub async fn connect_receiver(&self) -> Result<(), CastError> {
        let msg = Connection::Connect;
        let payload = serde_json::to_string(&msg).unwrap();

        let cast_msg = CastMessage {
            protocol_version: 0,
            source_id: "sender-0".to_string(),
            destination_id: "receiver-0".to_string(),
            namespace: connection::NAMESPACE.to_string(),
            payload_type: 0,
            payload_utf8: Some(payload),
            payload_binary: None,
        };
        self.send_message(cast_msg).await
    }

    pub async fn launch_app(&self, app_id: &str) -> Result<(), CastError> {
        let request_id = 1; // TODO: Manage request IDs dynamically
        let msg = ReceiverRequest::Launch {
            app_id: app_id.to_string(),
            request_id,
        };
        let payload = serde_json::to_string(&msg).unwrap();

        let cast_msg = CastMessage {
            protocol_version: 0,
            source_id: "sender-0".to_string(),
            destination_id: "receiver-0".to_string(),
            namespace: receiver::NAMESPACE.to_string(),
            payload_type: 0,
            payload_utf8: Some(payload),
            payload_binary: None,
        };
        self.send_message(cast_msg).await
    }

    pub async fn media_seek(
        &self,
        destination_id: &str,
        media_session_id: i32,
        time: f32,
    ) -> Result<(), CastError> {
        let request_id = 1; // TODO: Manage request IDs dynamically
        let msg = MediaRequest::Seek {
            request_id,
            media_session_id,
            current_time: time,
            resume_state: None,
        };
        let payload = serde_json::to_string(&msg).unwrap();

        let cast_msg = CastMessage {
            protocol_version: 0,
            source_id: "sender-0".to_string(),
            destination_id: destination_id.to_string(),
            namespace: media::NAMESPACE.to_string(),
            payload_type: 0,
            payload_utf8: Some(payload),
            payload_binary: None,
        };
        self.send_message(cast_msg).await
    }

    pub async fn media_get_status(
        &self,
        destination_id: &str,
        media_session_id: Option<i32>,
    ) -> Result<(), CastError> {
        let request_id = 1; // TODO: Manage request IDs dynamically
        let msg = MediaRequest::GetStatus {
            request_id,
            media_session_id,
        };
        let payload = serde_json::to_string(&msg).unwrap();

        let cast_msg = CastMessage {
            protocol_version: 0,
            source_id: "sender-0".to_string(),
            destination_id: destination_id.to_string(),
            namespace: media::NAMESPACE.to_string(),
            payload_type: 0,
            payload_utf8: Some(payload),
            payload_binary: None,
        };
        self.send_message(cast_msg).await
    }

    pub async fn set_volume(&self, level: f32) -> Result<(), CastError> {
        let request_id = 1; // TODO: Manage request IDs dynamically
        let msg = ReceiverRequest::SetVolume {
            request_id,
            volume: Volume {
                level: Some(level),
                muted: None,
            },
        };
        let payload = serde_json::to_string(&msg).unwrap();

        let cast_msg = CastMessage {
            protocol_version: 0,
            source_id: "sender-0".to_string(),
            destination_id: "receiver-0".to_string(),
            namespace: receiver::NAMESPACE.to_string(),
            payload_type: 0,
            payload_utf8: Some(payload),
            payload_binary: None,
        };
        self.send_message(cast_msg).await
    }

    pub fn events(&self) -> broadcast::Receiver<CastEvent> {
        self.event_tx.subscribe()
    }
}
