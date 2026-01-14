# Data Model: Platform and Media

## Receiver / Platform

### Application
- **app_id**: `String`
- **display_name**: `String`
- **session_id**: `String`
- **transport_id**: `String`
- **status_text**: `String`
- **is_idle_screen**: `bool`

### ReceiverStatus
- **request_id**: `i32`
- **applications**: `Vec<Application>`
- **volume**: `Volume`

## Media

### MediaStatus
- **media_session_id**: `i32`
- **playback_rate**: `f32`
- **player_state**: `String` (IDLE, PLAYING, PAUSED, BUFFERING)
- **current_time**: `f32`
- **media**: `MediaInformation` (optional)

### MediaInformation
- **content_id**: `String` (URL)
- **stream_type**: `String` (BUFFERED, LIVE, NONE)
- **content_type**: `String` (MIME type)
- **metadata**: `MediaMetadata` (optional)

### LoadRequest
- **type**: "LOAD"
- **sessionId**: `String` (Receiver session ID, usually source_id of message?) No, destination is transportId. Load message often includes `sessionId`? No, it's sent to the media receiver.
- **media**: `MediaInformation`
- **autoplay**: `bool`
- **currentTime**: `f32`
