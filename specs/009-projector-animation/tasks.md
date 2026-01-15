# Tasks: Projector Animation

**Feature Branch**: `009-projector-animation`
**Status**: Ready
**Total Tasks**: 5

## Dependencies

- **Phase 1 (Setup)**: Prerequisites
- **Phase 2 (Foundational)**: None (No blocking foundational tasks needed)
- **Phase 3 (User Story 1)**: Core implementation (Replaces 3D Cube)
- **Phase 4 (Polish)**: Clean up and final checks

## Phase 1: Setup

- [x] T001 Verify development environment and run existing tests to ensure baseline stability
- [x] T002 Prepare `src/controllers/tui.rs` for modification (backup or verify state)

## Phase 2: Foundational

*No foundational tasks required. Changes are self-contained in `src/controllers/tui.rs`.*

## Phase 3: User Story 1 - View Projector Animation

**Goal**: Replace the 3D cube with the looping projector ASCII animation.
**Priority**: P1
**Independent Test**: Run the app (`cargo run --example launch_app`), play media, and verify the projector animation cycles through 4 frames and is centered.

- [x] T003 [US1] Create unit tests for `render_projector_frame` in `src/controllers/tui.rs` to verify frame cycling and output dimensions (TDD)
- [x] T004 [US1] Implement `render_projector_frame` function in `src/controllers/tui.rs` with the 4 hardcoded ASCII frames and remove `render_cube_frame` logic
- [x] T005 [US1] Update `get_animation_frames` in `src/controllers/tui.rs` to call `render_projector_frame`

## Phase 4: Polish & Cross-Cutting Concerns

- [x] T006 Verify animation smoothness and alignment in a standard terminal window
- [x] T007 Run `cargo clippy` and `cargo fmt` to ensure code quality

## Parallel Execution Opportunities

- T003 (Tests) and T004 (Implementation) are sequential due to TDD, but T006 (Verification) can be done by a QA role once T005 is complete.
- Generally, this is a linear task list due to the small scope (single file modification).

## Implementation Strategy

1.  **Safety**: We will first ensure we have a test that fails (expecting projector frames) or at least verifies the new logic.
2.  **Replacement**: We will remove the complex 3D math code and replace it with the static array indexing, which simplifies the code significantly.
3.  **Verification**: Visual verification is key here, as unit tests can only check string contents, not "looks".
