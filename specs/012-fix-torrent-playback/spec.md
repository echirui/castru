# Feature Specification: Fix Torrent Playback

**Feature Branch**: `012-fix-torrent-playback`
**Created**: 2026-01-15
**Status**: Draft
**Input**: "torrent を指定した時にPLAYINGにはなるものの再生されません。修正をお願いします。" (When specifying a torrent, it becomes PLAYING but does not play. Please fix it.)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Reliable Torrent Playback (Priority: P1)

As a user, I want the video to actually start playing when I cast a torrent, so that I don't see a black screen or stuck loading indicator despite the status saying "PLAYING".

**Why this priority**: The core feature (torrent streaming) is currently broken/unusable.

**Independent Test**:
1.  Run `castru cast "magnet:?..."` with a known working magnet link.
2.  Observe the TV/Receiver.
3.  **Pass**: Video and audio start playing within a reasonable time (< 1 minute).
4.  **Fail**: Status says "PLAYING" but screen is black or stuck at 0:00 indefinitely.

**Acceptance Scenarios**:

1.  **Given** a valid magnet link, **When** I cast it, **Then** the application waits for sufficient buffering and begins serving valid media data, resulting in visible playback.
2.  **Given** the torrent download is slower than playback, **When** the player catches up to the download, **Then** the stream should pause/block waiting for data instead of sending invalid data (zeroes) which corrupts the stream.

### Edge Cases

- **Slow Download**: If peers are slow, the player should buffer (spin) rather than fail.
- **Sparse File**: Ensuring we don't read "holes" in the sparse file as valid data.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The `GrowingFile` adapter MUST check with the torrent engine if a requested byte range is actually downloaded before reading from the disk.
- **FR-002**: If the requested data is not yet downloaded, the read operation MUST block (return `Poll::Pending`) until the data becomes available.
- **FR-003**: The system MUST prioritize the pieces required by the current read cursor.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Playback starts successfully for the Big Buck Bunny test magnet link.
- **SC-002**: `GrowingFile` never returns successfully read bytes that are all zero unless the actual content is zero (verified by logs or checksums, though hard to test automatically).

## Assumptions

- The issue is caused by reading from sparse files before data is written, returning zeroes to the HTTP client.