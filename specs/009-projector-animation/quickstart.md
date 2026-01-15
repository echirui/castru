# Quickstart: Projector Animation

## Prerequisites

- Rust toolchain (cargo) installed.
- Terminal with support for ANSI escape codes (standard on macOS/Linux).

## Running the Animation

1. **Build the project**:
   ```bash
   cargo build
   ```

2. **Run the application**:
   ```bash
   # Use a sample media launch or discovery command that triggers the TUI
   cargo run --example launch_app
   ```
   *Note: Ensure the example used triggers the `TuiController`.*

3. **Verify the Animation**:
   - Observe the center of the terminal screen.
   - You should see the "Projector" ASCII art.
   - The reels (top circles) should rotate.
   - The light beam should animate (dots moving).
   - The animation should loop approximately every 0.6 seconds (depending on refresh rate).

## Troubleshooting

- **Animation is distorted**: Ensure your terminal window is at least 40x20 characters.
- **No animation**: Check if the application state is "PLAYING". The animation only plays in the playing state.
