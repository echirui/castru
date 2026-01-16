# Feature Specification: Torrent Full Download and Playback

**Feature Branch**: `015-torrent-full-download`  
**Created**: 2026-01-15  
**Status**: Draft  
**Input**: User description: "torrentの再生がまだできません。ダウンロードの進捗を百分率で表示し、まずは、全てダウンロードしてから再生できるようにしたいです。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Monitor Download Progress (Priority: P1)

As a user, I want to see the current download progress of a torrent as a percentage in the terminal, so that I know how much more time I need to wait before playback begins.

**Why this priority**: High. Essential for user feedback during the waiting period.

**Independent Test**: Start casting a torrent and observe the terminal. The display should show a numeric percentage (0-100%) that updates as data is received.

**Acceptance Scenarios**:

1. **Given** a magnet link or .torrent file is provided, **When** the download starts, **Then** the TUI should display "Downloading: XX.X%".
2. **Given** the download is in progress, **When** more pieces are downloaded, **Then** the percentage should increase accurately.

---

### User Story 2 - Automated Playback After Completion (Priority: P1)

As a user, I want the system to wait until the entire torrent is downloaded before attempting to cast it to my device, so that playback is smooth and free from buffering issues caused by slow torrent speeds.

**Why this priority**: High. Directly addresses the user's request to "download everything first".

**Independent Test**: Start casting a small torrent. Verify that the Chromecast receiver remains idle or on a "Waiting" screen until the local download reaches 100%, at which point the media should automatically load and play.

**Acceptance Scenarios**:

1. **Given** a torrent download is active, **When** the progress is less than 100%, **Then** the application should not send the LOAD command to the Chromecast.
2. **Given** the download reaches 100%, **When** all files are verified, **Then** the application should immediately trigger playback on the target device.

---

### User Story 3 - Error Handling during Download (Priority: P2)

As a user, I want to be informed if the download fails or stalls, so that I don't wait indefinitely for a playback that will never start.

**Why this priority**: Medium. Important for robustness.

**Independent Test**: Try to download a torrent with no peers. The system should eventually report a timeout or a "Stalled" status.

**Acceptance Scenarios**:

1. **Given** a download that has no progress for a significant time, **When** a timeout threshold is reached, **Then** the TUI should show an error message.

---

### Edge Cases

- **Multiple Files in Torrent**: If a torrent contains multiple files, how does the system choose which one to play? (Assumption: Choose the largest file or follow existing logic).
- **Restarting App**: If the app is restarted with the same torrent, does it resume from the existing percentage? (Assumption: Yes, if the library supports it).
- **Insufficient Disk Space**: What happens if the download cannot complete?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST calculate the total download progress of the selected media file within the torrent as a percentage.
- **FR-002**: System MUST update the TUI state with the current download percentage at least once per second.
- **FR-003**: System MUST block the `load_media` process (sending the LOAD message to the receiver) until the download progress reaches 100%.
- **FR-004**: System MUST verify the integrity of the downloaded file before initiating playback.
- **FR-005**: System MUST provide a clear visual indicator in the TUI when the state transitions from "Downloading" to "Casting".

### Key Entities *(include if feature involves data)*

- **Torrent Session**: Manages the peer connections and piece downloads.
- **Download Progress**: A value derived from (pieces_downloaded / total_pieces) * 100.
- **Media File**: The specific file within the torrent targeted for playback.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Percentage display accuracy within 0.1%.
- **SC-002**: Playback initiates within 2 seconds of the download reaching 100%.
- **SC-003**: 0% chance of "Buffering" or "Media Not Found" errors on the receiver due to incomplete local files.
- **SC-004**: User can successfully play a 100MB torrent file from start to finish after waiting for the full download.

## Assumptions

- The current torrent library (`librqbit`) provides a way to get the total size and downloaded bytes or pieces.
- The system has enough disk space to store the entire torrent.
- The user is aware that they must wait for the full download.