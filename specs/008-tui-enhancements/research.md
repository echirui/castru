# Research: TUI Animation

**Decision**: Use Unicode geometric shapes to simulate a spinning disc.
**Rationale**: Shapes like `◐ ◓ ◑ ◒` provide a clean, "disc-like" rotation effect that is widely supported in modern terminals and fits the "DVD" metaphor better than a simple line spinner (`| / - \`).

**Candidates Evaluated**:
1. **Line Spinner** (`| / - \`): Too generic/system-like.
2. **ASCII Disc** (`( @ )` -> `(   @)`): Hard to make smooth in small space.
3. **Unicode Circles** (`◐ ◓ ◑ ◒`): looks like a spinning CD/DVD.
