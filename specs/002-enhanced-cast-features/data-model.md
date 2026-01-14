# Data Model: Enhanced Cast Features

## Discovery

### CastDevice
- **ip**: `std::net::IpAddr`
- **port**: `u16` (default 8009)
- **friendly_name**: `String` (from mDNS TXT `fn`)
- **model_name**: `String` (from mDNS TXT `md`)
- **uuid**: `String` (from mDNS TXT `id`)

## Media Control

### MediaMetadata
- **title**: `String`
- **subtitle**: `String` (optional)
- **images**: `Vec<Image>` (url, height, width)

### MediaStatus
- **media_session_id**: `i32`
- **playback_rate**: `f32`
- **player_state**: `String` (PLAYING, PAUSED, BUFFERING, IDLE)
- **current_time**: `f32`
- **supported_media_commands**: `i32` (bitmask)
- **volume**: `Volume`

### Volume
- **level**: `f32` (0.0 to 1.0)
- **muted**: `bool`

## Reconnection

### ConnectionSettings
- **connect_timeout**: `Duration`
- **retry_attempts**: `u32`
- **retry_delay**: `Duration`
