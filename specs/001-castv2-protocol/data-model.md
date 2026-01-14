# Data Model: CastV2 Protocol

## Core Entities

### CastMessage (Protobuf)
Defined in `cast_channel.proto`.
- **protocol_version**: Enum (CASTV2_1_0)
- **source_id**: String (e.g., "sender-0")
- **destination_id**: String (e.g., "receiver-0")
- **namespace**: String (e.g., "urn:x-cast:com.google.cast.tp.heartbeat")
- **payload_type**: Enum (STRING = 0, BINARY = 1)
- **payload_utf8**: String (optional)
- **payload_binary**: Bytes (optional)

### CastHeader
- **length**: u32 (Big Endian) - Represents the size of the following Protobuf message.

## Internal Structures

### CastCommand
Enum representing high-level commands.
- `Ping`
- `Pong`
- `Connect`
- `Launch { app_id: String }`
- `Load { media_id: String, ... }`

### ConnectionState
- `Connected`
- `Disconnected`
- `Handshaking`
- `Authenticated`

### CastDevice
- **ip**: IpAddr
- **port**: u16 (default 8009)
- **name**: String
