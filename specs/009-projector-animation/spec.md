# Feature Specification: Projector Animation

**Feature Branch**: `009-projector-animation`
**Created**: 2026-01-15
**Status**: Draft
**Input**: User description: "アニメーションを 3D Cubeから映写機のアスキーアートにしたいです。 Plaintext .-------. .-------. / _/_ \ / _/_ \ | ( / ) |=========| ( / ) | \ / \ / \ / \ / '-------' '-------' __________|_____________|__________ | | | _______________________ |===> . . . | | ( ( O ) ) | | | |_______________________| | |___________________________________| / \ Frame 2 Plaintext .-------. .-------. / _|_ \ / _|_ \ | ( - ) |=========| ( - ) | \ _|_ / \ _|_ / '-------' '-------' __________|_____________|__________ | | | _______________________ |===> . . | | ( ( O ) ) | | | |_______________________| | |___________________________________| / \ Frame 3 Plaintext .-------. .-------. / \_\ \ / \_\ \ | ( \ ) |=========| ( \ ) | \ / / / \ / / / '-------' '-------' __________|_____________|__________ | | | _______________________ |===> . . . | | ( ( O ) ) | | | |_______________________| | |___________________________________| / \ Frame 4 Plaintext .-------. .-------. / | \ / | \ | ( | ) |=========| ( | ) | \ | / \ | / '-------' '-------' __________|_____________|__________ | | | _______________________ |===> . . | | ( ( O ) ) | | | |_______________________| | |___________________________________| /"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - View Projector Animation (Priority: P1)

As a user, I want to see a projector ASCII art animation instead of the 3D cube, so that I have a distinct visual indicator for the application's state.

**Why this priority**: This is the core request of the feature to change the visual feedback.

**Independent Test**: Can be fully tested by running the application and observing the terminal output.

**Acceptance Scenarios**:

1. **Given** the application is running in a state that displays animation, **When** I observe the terminal, **Then** I see the projector ASCII art animation.
2. **Given** the animation is displayed, **When** I watch the animation loop, **Then** I see the "film reels" rotating and the "light beam" effect as per the specified frames.
3. **Given** the application is running, **When** I check the animation, **Then** I do NOT see the previous 3D Cube animation.

### Edge Cases

- What happens when the terminal size is too small for the ASCII art?
  - The system should attempt to display it, possibly clipped, or handle it gracefully as per existing TUI behavior.
- What happens if the animation refresh rate is too fast or slow?
  - The animation should play at a readable speed where the motion of the reels is discernible.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST replace the existing "3D Cube" animation with the "Projector" ASCII art animation.
- **FR-002**: The animation MUST consist of the 4 frames provided in the user description.
- **FR-003**: The animation MUST loop continuously while active.
- **FR-004**: The animation frames MUST depict the rotation of the reels (top circles) and the projection of light/dots.

### Key Entities *(include if feature involves data)*

- **Frame**: A static ASCII art string representing one state of the animation.
- **Animation Sequence**: An ordered list of Frames that play in a loop.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The "3D Cube" animation is completely removed from the active display.
- **SC-002**: The "Projector" animation is visible and identifiable as the provided ASCII art.
- **SC-003**: All 4 frames of the animation are displayed in the correct order.
- **SC-004**: The animation plays smoothly without visual glitches in a standard terminal environment.