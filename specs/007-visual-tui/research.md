# Research: Visual TUI (btop-style)

**Decision**: Use `crossterm` for all TUI operations.
**Rationale**: `crossterm` is already a dependency and sufficient for clearing screens, handling raw input, and drawing text/colors. Adding `tui` (ratatui) would introduce complexity and dependencies not strictly needed for this scope.
**Alternatives Considered**:
- `ratatui`: Powerful but overkill for the current requirement.
- `termion`: Unix-only, less cross-platform.
