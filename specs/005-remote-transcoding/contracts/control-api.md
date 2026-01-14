# Remote Control API Contract

The server optionally exposes an HTTP API for remote control.

## Endpoints

### POST /api/v1/playback/toggle
Toggles play/pause state.
- **Response**: 204 No Content

### POST /api/v1/playback/seek
Seeks to a specific position.
- **Request Body**:
  ```json
  {
    "offset": 30.5,
    "relative": true
  }
  ```
- **Response**: 204 No Content

### POST /api/v1/playback/stop
Stops playback and ends session.
- **Response**: 204 No Content
