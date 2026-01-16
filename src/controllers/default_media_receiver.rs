use crate::client::CastClient;
use crate::controllers::media::MediaController;
use crate::controllers::receiver::ReceiverController;
use crate::error::CastError;
use crate::protocol::media::MediaInformation;
use crate::protocol::receiver::{ReceiverResponse, NAMESPACE as RECEIVER_NAMESPACE};

const DEFAULT_MEDIA_RECEIVER_ID: &str = "CC1AD845";

/// A high-level wrapper for the Default Media Receiver application.
///
/// This struct simplifies the process of launching the Default Media Receiver,
/// connecting to it, and loading media.
pub struct DefaultMediaReceiver {
    client: CastClient,
    receiver_controller: ReceiverController,
    media_controller: Option<MediaController>,
    app_id: String,
}

impl DefaultMediaReceiver {
    /// Creates a new DefaultMediaReceiver instance.
    pub fn new(client: &CastClient) -> Self {
        Self {
            client: client.clone(),
            receiver_controller: ReceiverController::new(client),
            media_controller: None,
            app_id: DEFAULT_MEDIA_RECEIVER_ID.to_string(),
        }
    }

    /// Launches the Default Media Receiver application and establishes a connection.
    ///
    /// This method will:
    /// 1. Send a LAUNCH request for the Default Media Receiver.
    /// 2. Wait for a RECEIVER_STATUS message indicating the app is running.
    /// 3. Connect to the application's transport ID.
    /// 4. Initialize the internal MediaController.
    pub async fn launch(&mut self) -> Result<(), CastError> {
        let mut rx = self.client.events();

        self.receiver_controller.launch_app(&self.app_id).await?;

        // Wait for the application to be ready
        // We look for a RECEIVER_STATUS message that contains our App ID with a transportId
        let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(15));
        tokio::pin!(timeout);

        loop {
            tokio::select! {
                Ok(event) = rx.recv() => {
                    if event.namespace == RECEIVER_NAMESPACE {
                        if let Ok(ReceiverResponse::ReceiverStatus { status, .. }) = serde_json::from_str::<ReceiverResponse>(&event.payload) {
                            if let Some(app) = status.applications.iter().find(|a| a.app_id == self.app_id) {
                                // App is running, connect to it
                                self.receiver_controller.join_session(&app.transport_id).await?;

                                // Initialize MediaController
                                self.media_controller = Some(MediaController::new(&self.client, &app.transport_id));
                                return Ok(());
                            }
                        }
                    }
                }
                _ = &mut timeout => {
                    return Err(CastError::Protocol("Timeout waiting for application launch".into()));
                }
            }
        }
    }

    /// Check if the receiver has been launched and connected.
    pub fn is_connected(&self) -> bool {
        self.media_controller.is_some()
    }

    /// Loads media content into the receiver.
    pub async fn load(
        &self,
        media: MediaInformation,
        autoplay: bool,
        current_time: f32,
        active_track_ids: Option<Vec<i32>>,
    ) -> Result<(), CastError> {
        if let Some(controller) = &self.media_controller {
            controller.load(media, autoplay, current_time, active_track_ids).await
        } else {
            Err(CastError::Protocol(
                "MediaController not initialized. Call launch() first.".into(),
            ))
        }
    }

    /// Plays the media with the given session ID.
    pub async fn play(&self, media_session_id: i32) -> Result<(), CastError> {
        if let Some(controller) = &self.media_controller {
            controller.play(media_session_id).await
        } else {
            Err(CastError::Protocol(
                "MediaController not initialized. Call launch() first.".into(),
            ))
        }
    }

    /// Pauses the media with the given session ID.
    pub async fn pause(&self, media_session_id: i32) -> Result<(), CastError> {
        if let Some(controller) = &self.media_controller {
            controller.pause(media_session_id).await
        } else {
            Err(CastError::Protocol(
                "MediaController not initialized. Call launch() first.".into(),
            ))
        }
    }

    /// Stops the media playback.
    pub async fn stop(&self, media_session_id: i32) -> Result<(), CastError> {
        if let Some(controller) = &self.media_controller {
            controller.stop(media_session_id).await
        } else {
            Err(CastError::Protocol(
                "MediaController not initialized. Call launch() first.".into(),
            ))
        }
    }

    /// Seeks to a specific time in the media.
    pub async fn seek(&self, media_session_id: i32, time: f32) -> Result<(), CastError> {
        if let Some(controller) = &self.media_controller {
            controller.seek(media_session_id, time).await
        } else {
            Err(CastError::Protocol(
                "MediaController not initialized. Call launch() first.".into(),
            ))
        }
    }
}
