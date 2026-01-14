
//! Terminal User Interface (TUI) controller.

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType};
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::execute;
use crossterm::style::{Print, ResetColor, SetForegroundColor, Color};
use std::io::{stdout, Write};
use std::time::Duration;
use crate::error::CastError;
use tokio::sync::mpsc;
use std::thread;

#[derive(Debug, Clone, PartialEq)]
pub enum TuiCommand {
    Play,
    Pause,
    Stop,
    Next,
    Previous,
    SeekForward(u64), // Seconds
    SeekBackward(u64), // Seconds
    VolumeUp,
    VolumeDown,
    Quit,
}

pub struct TuiController;

impl Default for TuiController {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiController {
    pub fn new() -> Self {
        Self
    }

    /// Enter raw mode and start listening for events in a background thread.
    /// Returns a receiver for TuiCommands.
    pub fn start(&self) -> Result<mpsc::Receiver<TuiCommand>, CastError> {
        let (tx, rx) = mpsc::channel(32);
        
        enable_raw_mode().map_err(|e| CastError::Tui(e.to_string()))?;
        execute!(stdout(), Hide).map_err(|e| CastError::Tui(e.to_string()))?;

        thread::spawn(move || {
            loop {
                if event::poll(Duration::from_millis(100)).unwrap_or(false) {
                    if let Ok(Event::Key(KeyEvent { code, modifiers, .. })) = event::read() {
                        let command = match code {
                             KeyCode::Char(' ') => Some(TuiCommand::Pause),
                             KeyCode::Char('k') => Some(TuiCommand::Pause),
                             KeyCode::Char('q') | KeyCode::Esc => Some(TuiCommand::Quit),
                             KeyCode::Right => Some(TuiCommand::SeekForward(10)),
                             KeyCode::Left => Some(TuiCommand::SeekBackward(10)),
                             KeyCode::Up => Some(TuiCommand::VolumeUp),
                             KeyCode::Down => Some(TuiCommand::VolumeDown),
                             KeyCode::Char('n') => Some(TuiCommand::Next),
                             KeyCode::Char('p') => Some(TuiCommand::Previous),
                             _ => {
                                 if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
                                     Some(TuiCommand::Quit)
                                 } else {
                                     None
                                 }
                             }
                        };

                        if let Some(cmd) = command {
                            if tx.blocking_send(cmd.clone()).is_err() {
                                break;
                            }
                            if matches!(cmd, TuiCommand::Quit) {
                                break;
                            }
                        }
                    }
                }
            }
        });

        Ok(rx)
    }

    pub fn stop(&self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), Show);
    }

    pub fn draw_status(&self, status_text: &str, current_time: f32, duration: Option<f32>) -> Result<(), CastError> {
        let mut stdout = stdout();
        
        let (cols, _rows) = crossterm::terminal::size().unwrap_or((80, 24));
        
        execute!(stdout, MoveTo(0, 0), Clear(ClearType::CurrentLine)).map_err(|e| CastError::Tui(e.to_string()))?;
        
        // Format time
        let curr_fmt = format_time(current_time);
        let dur_fmt = if let Some(d) = duration { format_time(d) } else { "--:--".to_string() };

        // Progress Bar
        let progress_str = if let Some(dur) = duration {
            if dur > 0.0 {
                let width = (cols as usize).saturating_sub(40); // Leave space for text
                let ratio = (current_time / dur).clamp(0.0, 1.0);
                let filled = (ratio * width as f32) as usize;
                let empty = width.saturating_sub(filled);
                format!("[{}{}]", "=".repeat(filled), " ".repeat(empty))
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };

        execute!(stdout, 
            SetForegroundColor(Color::Cyan),
            Print(format!("{} | {} / {} {}", status_text, curr_fmt, dur_fmt, progress_str)),
            ResetColor
        ).map_err(|e| CastError::Tui(e.to_string()))?;
        
        stdout.flush().map_err(CastError::Io)?;
        Ok(())
    }
}

fn format_time(seconds: f32) -> String {
    let secs = seconds as u64;
    let m = secs / 60;
    let s = secs % 60;
    format!("{:02}:{:02}", m, s)
}

impl Drop for TuiController {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), Show);
    }
}
