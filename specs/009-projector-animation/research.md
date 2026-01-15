# Research: Projector Animation

**Feature**: Projector Animation (009-projector-animation)
**Status**: Complete
**Date**: 2026-01-15

## Technical Decisions

### 1. Animation Logic

**Decision**: Replace procedural 3D Cube generation with static ASCII frame cycling.

**Rationale**:
- The requirement is to display specific ASCII art frames provided by the user.
- Procedural generation is unnecessary and complex for this specific visual.
- Cycling through a pre-defined array of strings is more performant and easier to maintain.

**Implementation Details**:
- **Frame Storage**: Store the 4 frames as `const` arrays of strings or a slice of string literals.
- **Frame Selection**: Use `frame_index % 4` to select the current frame.
- **Centering**: Calculate padding dynamically based on terminal size to center the ASCII art.

### 2. Integration Point

**Decision**: Modify `src/controllers/tui.rs`.

**Rationale**:
- This file contains the existing `TuiController` and `render_cube_frame` logic.
- It is the centralized location for TUI rendering in the current architecture.

**Changes**:
- Remove `render_cube_frame` and its helper logic (rotation math, z-buffer).
- Add `render_projector_frame(frame: usize, width: usize, height: usize) -> Vec<String>`.
- Update `get_animation_frames` to call `render_projector_frame`.

### 3. Frame Data

The frames provided in the spec will be hardcoded.

**Frame 1**:
```text
      .-------.             .-------.
     /   _/_   \           /   _/_   \
     |  ( / )  |===========|  ( / )  |
      \   \ /  /           \   \ /  /
       '-------'             '-------'
      __________|_____________|__________
     |          |             |          |
     |          |_____________|__________| 
     |          |             |          |
     |===>  . . .             |          |
     |          |  ( ( O ) )  |          |
     |          |             |          |
     |__________|_____________|__________|
                |             |
                |_____________|
```
*Note: I will carefully adapt the provided raw text into valid Rust string literals, handling escape characters.*

## Alternatives Considered

### Procedural Generation for Projector
- **Pros**: Potentially smoother or more dynamic.
- **Cons**: Extremely difficult to match the specific artistic style requested; high effort for low value.
- **Verdict**: Rejected.

### External Asset Files
- **Pros**: Easier to update frames without recompiling.
- **Cons**: Adds file I/O complexity and deployment dependencies for a simple CLI tool.
- **Verdict**: Rejected. Hardcoding is sufficient.

## Open Questions

None. The requirements are clear and the codebase structure is well-understood.
