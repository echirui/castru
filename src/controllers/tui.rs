
//! Terminal User Interface (TUI) controller.

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
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
    TogglePlay, // Added
    Stop,
    Next,
    Previous,
    SeekForward(u64), // Seconds
    SeekBackward(u64), // Seconds
    VolumeUp,
    VolumeDown,
    ToggleMute,
    Quit,
}

pub struct TuiState {
    pub status: String,
    pub current_time: f32,
    pub total_duration: Option<f32>,
    pub volume_level: Option<f32>, // 0.0 to 1.0
    pub is_muted: bool,
    pub media_title: Option<String>,
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
        execute!(stdout(), EnterAlternateScreen, Hide).map_err(|e| CastError::Tui(e.to_string()))?;

        thread::spawn(move || {
            loop {
                if event::poll(Duration::from_millis(100)).unwrap_or(false) {
                    if let Ok(Event::Key(KeyEvent { code, modifiers, .. })) = event::read() {
                        let command = match code {
                             KeyCode::Char(' ') => Some(TuiCommand::TogglePlay),
                             KeyCode::Char('k') => Some(TuiCommand::TogglePlay),
                             KeyCode::Char('q') | KeyCode::Esc => Some(TuiCommand::Quit),
                             KeyCode::Right => Some(TuiCommand::SeekForward(30)),
                             KeyCode::Left => Some(TuiCommand::SeekBackward(15)),
                             KeyCode::Up => Some(TuiCommand::VolumeUp),
                             KeyCode::Down => Some(TuiCommand::VolumeDown),
                             KeyCode::Char('m') => Some(TuiCommand::ToggleMute),
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
        let _ = execute!(stdout(), Show, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }

    pub fn draw(&self, state: &TuiState) -> Result<(), CastError> {
        let mut stdout = stdout();
        
        let (cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));
        let cy = rows / 2;
        
        // Prepare Status String
        let status_color = match state.status.to_uppercase().as_str() {
            "PLAYING" => Color::Green,
            "PAUSED" => Color::Yellow,
            "BUFFERING" => Color::Blue,
            _ => Color::Grey,
        };

        // Format Time
        let curr_fmt = format_duration(state.current_time);
        let dur_fmt = if let Some(d) = state.total_duration { format_duration(d) } else { "--:--".to_string() };
        let time_str = format!("{} / {}", curr_fmt, dur_fmt);
        
        // Volume
        let vol_str = if state.is_muted {
            "(Muted)".to_string()
        } else {
            match state.volume_level {
                Some(v) => format!("Vol: {:.0}%", v * 100.0),
                None => "Vol: --%".to_string(),
            }
        };

        execute!(stdout, Clear(ClearType::All)).map_err(|e| CastError::Tui(e.to_string()))?;

        // Title
        if let Some(ref title) = state.media_title {
             let t_x = (cols as usize).saturating_sub(title.len()) / 2;
             execute!(stdout, MoveTo(t_x as u16, cy.saturating_sub(4)), Print(title)).ok();
        }

        // Status
        let status_text = format!("[ {} ]", state.status.to_uppercase());
        let s_x = (cols as usize).saturating_sub(status_text.len()) / 2;
        execute!(stdout, 
            MoveTo(s_x as u16, cy.saturating_sub(2)), 
            SetForegroundColor(status_color), 
            Print(&status_text), 
            ResetColor
        ).ok();

        // Time
        let tm_x = (cols as usize).saturating_sub(time_str.len()) / 2;
        execute!(stdout, MoveTo(tm_x as u16, cy), Print(&time_str)).ok();

        // Bar (Seekbar)
        let bar_width = (cols as usize).saturating_sub(10).max(10);
        let bar_x = (cols as usize - bar_width) / 2;
        let progress_bar = render_progress_bar(state.current_time, state.total_duration, bar_width);
        execute!(stdout, MoveTo(bar_x as u16, cy + 2), Print(progress_bar)).ok();

        // Volume
         let v_x = (cols as usize).saturating_sub(vol_str.len()) / 2;
        execute!(stdout, MoveTo(v_x as u16, cy + 4), Print(vol_str)).ok();

        // Footer
        let footer = "[Space] Toggle  [Arrow] Seek/Vol  [M] Mute  [Q] Quit";
        let f_x = (cols as usize).saturating_sub(footer.len()) / 2;
        execute!(stdout, MoveTo(f_x as u16, rows.saturating_sub(2)), SetForegroundColor(Color::DarkGrey), Print(footer), ResetColor).ok();
        
        stdout.flush().map_err(CastError::Io)?;
        Ok(())
    }
}

fn format_duration(seconds: f32) -> String {
    let seconds = if seconds.is_nan() || seconds < 0.0 { 0.0 } else { seconds };
    let secs = seconds as u64;
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        format!("{:02}:{:02}", m, s)
    }
}

fn render_progress_bar(current: f32, total: Option<f32>, width: usize) -> String {
    if width < 2 { return "".to_string(); }
    let total = match total {
        Some(t) if t > 0.0 => t,
        _ => return format!("[{}]", " ".repeat(width - 2)),
    };
    
    let ratio = (current / total).clamp(0.0, 1.0);
    let bar_len = width - 2;
    let filled_len = (ratio * bar_len as f32).round() as usize;
    let empty_len = bar_len.saturating_sub(filled_len);
    
    // Use block chars for btop feel? Or classic '='?
    // User asked for "seekbar like". Block is better.
    // '█' is good.
    let bar_body = "█".repeat(filled_len);
    
    // Add arrow head? Standard progress bar usually just fills.
    // Let's stick to simple filling.
    
    // Use background color for empty part? Or just spaces.
    // Let's use '░' for empty part.
    let empty_body = "░".repeat(empty_len);

    format!("{}{}", bar_body, empty_body)
}

impl Drop for TuiController {
    fn drop(&mut self) {
        let _ = execute!(stdout(), Show, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

