# Specification: TUI Enhancements & Animation

## 1. Overview
Enhance the Visual TUI with detailed media information, device status, and a dynamic "spinning DVD" animation to improve the visual experience and providing more technical context to the user.

## 2. Problem Statement
- **Missing Info**: The current TUI shows playback time but lacks technical details like Codec (Video/Audio) and the specific Cast Device being used.
- **Static Feel**: While the TUI has a seekbar, it feels static. The user requested a "spinning DVD" animation to make it feel more alive and indicative of playback.

## 3. Goals
- **Rich Metadata**: Display Video/Audio Codecs and Media Title.
- **Device Context**: Show the connected Cast Device name and details.
- **Visual Delight**: Implement a "Spinning DVD" ASCII animation that animates during playback.

## 4. User Stories

### US.1 Detailed Media Metadata
- **As a** user,
- **I want** to see the media title and codec information (e.g., h264, aac),
- **So that** I know exactly what file format is being streamed.

### US.2 Device Information
- **As a** user,
- **I want** to see the name of the Cast device I am connected to,
- **So that** I can confirm I am casting to the right room/hardware.

### US.3 DVD Animation
- **As a** user,
- **I want** to see a "spinning DVD" animation when media is playing,
- **So that** the interface feels active and retro/fun.

## 5. Technical Requirements

### 5.1 Metadata
- The system must capture and store media codec information (video/audio) during the loading process.
- The system must retain the connected device's name and details for display.
- This information must be accessible to the UI rendering component.

### 5.2 Animation
- Implement a frame-based ASCII animation representing a spinning disc.
- The animation must cycle frames at a regular interval (e.g., 100-200ms) to create motion.
- Animation must be active only when the playback status is 'Playing'.

### 5.3 Layout Update
- The UI layout must be updated to include:
    - **Header**: Connected Device Name.
    - **Center**: Animation and Media Title.
    - **Details**: Format/Codec information.
    - **Footer**: Existing controls (Seekbar, Hints).

## 6. Success Criteria
- [ ] TUI displays specific Video and Audio codecs (if available).
- [ ] TUI displays the Cast Device name.
- [ ] A "spinning DVD" style ASCII animation is visible and moves when playing.
- [ ] Animation stops when paused.
