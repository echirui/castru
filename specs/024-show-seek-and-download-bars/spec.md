# Feature Specification: Show Seek and Download Bars

**Feature Branch**: `024-show-seek-and-download-bars`
**Created**: 2026-01-18
**Status**: Draft
**Input**: User description: "torrent の場合は、seekバートダウンロードバーを両方表示するようにしてください。"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Simultaneous Seek and Download Bars (Priority: P1)

As a user streaming a torrent, I want to see both the playback progress (seek bar) and the download progress bar simultaneously in the TUI, so that I can visualize how much content is buffered relative to the playback position.

**Why this priority**: Currently, the TUI might toggle or overlay these, making it hard to see the buffer safety margin at a glance.

**Independent Test**: Stream a large torrent file. The TUI should display two distinct bars (or a combined visual) showing playback position and download completion percentage concurrently.

**Acceptance Scenarios**:

1. **Given** a playing torrent stream, **When** the TUI renders, **Then** both the playback progress bar and the download progress bar are visible.
2. **Given** a non-torrent stream (local file), **When** the TUI renders, **Then** only the playback progress bar is visible (as download is irrelevant/100%).
3. **Given** the download completes (100%), **When** the TUI renders, **Then** the download bar indicates completion (e.g., full or removed if designed that way, but explicit visibility is requested).

---

### User Story 2 - Visual Distinction (Priority: P2)

As a user, I want the two bars to be visually distinct (e.g., position, label, or color), so that I don't confuse playback progress with download progress.

**Why this priority**: clarity is essential for the dual-bar approach to be useful.

**Independent Test**: Observe the TUI. The bars should be clearly labeled (e.g., "Playback", "Download") or positioned consistently (e.g., Playback above Download).

**Acceptance Scenarios**:

1. **Given** the TUI with both bars, **When** I look at the display, **Then** I can clearly identify which bar corresponds to playback and which to download.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The TUI rendering logic MUST check if the source is a torrent.
- **FR-002**: If the source is a torrent, the system MUST render a download progress bar in addition to the playback seek bar.
- **FR-003**: The download progress bar MUST reflect the `torrent_progress` state from `AppState`.
- **FR-004**: The playback seek bar MUST reflect the `current_time` relative to `total_duration` (existing behavior).
- **FR-005**: The two bars MUST be positioned to avoid overlap or layout breakage.

### Key Entities

- **TuiState**: Existing entity, needs to support fields for rendering both bars (already has `torrent_progress`).
- **TuiController**: Needs updated drawing logic.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Both bars are visible 100% of the time during an active torrent stream.
- **SC-002**: The TUI layout remains stable (no flickering or misalignment) with the extra bar.
- **SC-003**: User can identify the download status within 1 second of looking at the TUI.