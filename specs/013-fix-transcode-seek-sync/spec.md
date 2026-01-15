# Feature Specification: Transcoded Seek Sync Fix

**Feature Branch**: `013-fix-transcode-seek-sync`  
**Created**: 2026-01-15  
**Status**: Draft  
**Input**: User description: "mp4以外のコーデックの場合、seekした場合、時間表示と動画の時間に不整合が起きます。修正をお願いします。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Accurate Seek Synchronization (Priority: P1)

As a user watching a transcoded (non-native mp4) video, I want the seeker bar and time display to accurately reflect the video's actual playback position after I perform a seek operation, so that I can reliably navigate the content.

**Why this priority**: Correct temporal navigation is a fundamental requirement for media playback. Current inconsistency renders seeking in non-mp4 files confusing and broken.

**Independent Test**:
1. Play an MKV or AVI file that requires transcoding.
2. Seek forward 30 seconds.
3. Observe the time display in the TUI vs. the visual progress of the video.
4. **Pass**: The time display shows `00:30` (or original + 30s) and the video content matches that timestamp.
5. **Fail**: The time display resets to `00:00` or shows a time significantly different from the visual content.

**Acceptance Scenarios**:

1. **Given** a transcoded stream is playing at 1 minute, **When** I seek forward to 2 minutes, **Then** the video reloads at the correct scene and the TUI display updates to show 2 minutes.
2. **Given** a transcoded stream has been seeked multiple times, **When** I check the remaining time, **Then** it accurately reflects the distance to the end of the file.

### Edge Cases

- **Seeking near the end of the file**: Does the time display overflow or correctly cap at the duration?
- **Rapid successive seeks**: Does the internal offset calculation accumulate correctly without drifting?
- **Small seeks (e.g., +/- 5s)**: Is the adjustment precision high enough to avoid noticeable drift?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST maintain an internal `seek_offset` state for transcoded playback sessions.
- **FR-002**: When seeking in a transcoded stream, the system MUST restart the stream from the target timestamp.
- **FR-003**: The system MUST adjust the relative timestamps reported by the playback engine by adding the current `seek_offset` before displaying them to the user.
- **FR-004**: The system MUST ensure the total duration remains consistent regardless of the current seek position.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Time display drift after a single seek operation is less than 500ms compared to actual video timestamp.
- **SC-002**: The TUI progress bar correctly represents the percentage of playback relative to the total file duration after seeking.
- **SC-003**: Successive seeks (at least 5 in a row) do not result in more than 1 second of cumulative drift.

## Assumptions

- The underlying transcoding tool supports precise seeking to a specific timestamp.
- The playback status events from the device report time relative to the current stream start.
