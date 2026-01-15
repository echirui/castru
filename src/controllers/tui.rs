
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
    TogglePlay,
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
    // New metadata
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub device_name: String,
    pub animation_frame: usize,
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
        
        // Colors
        let status_color = match state.status.to_uppercase().as_str() {
            "PLAYING" => Color::Green,
            "PAUSED" => Color::Yellow,
            "BUFFERING" => Color::Blue,
            _ => Color::Grey,
        };

        // 1. Double the area (4x) / Fill screen Cube Background
        let w = cols as usize;
        let h = rows as usize;
        let anim_frames = if state.status.eq_ignore_ascii_case("PLAYING") {
             get_animation_frames(state.animation_frame, w, h)
        } else {
             get_static_frame(w, h)
        };
        
        // Render Cube (Background)
        for (i, line) in anim_frames.iter().enumerate() {
            let ry = i as u16;
            if ry >= rows { break; }
            let l_x = (cols as usize).saturating_sub(line.len()) / 2;
            execute!(stdout, 
                MoveTo(0, ry), 
                Clear(ClearType::CurrentLine), 
                MoveTo(l_x as u16, ry), 
                SetForegroundColor(Color::DarkGrey),
                Print(line), 
                ResetColor
            ).ok();
        }

        // Overlay Metadata (Floating on top)
        
        // Header
        let dev_str = format!(" Device: {} ", state.device_name);
        execute!(stdout, 
            MoveTo(2, 1), 
            SetForegroundColor(Color::Cyan), 
            Print(dev_str), 
            ResetColor
        ).ok();

        // 3. Title
        let title_str = format!(" {} ", state.media_title.as_deref().unwrap_or("No Media"));
        let t_y = cy.saturating_sub(2);
        let t_x = (cols as usize).saturating_sub(title_str.len()) / 2;
        execute!(stdout, 
            MoveTo(t_x as u16, t_y as u16), 
            SetForegroundColor(Color::White), 
            Print(title_str), 
            ResetColor
        ).ok();

        // 4. Status
        let status_text = format!(" [ {} ] ", state.status.to_uppercase());
        let s_y = t_y + 1;
        let s_x = (cols as usize).saturating_sub(status_text.len()) / 2;
        execute!(stdout, 
            MoveTo(s_x as u16, s_y as u16), 
            SetForegroundColor(status_color), 
            Print(status_text), 
            ResetColor
        ).ok();

        // 5. Time
        let curr_fmt = format_duration(state.current_time);
        let dur_fmt = if let Some(d) = state.total_duration { format_duration(d) } else { "--:--".to_string() };
        let time_str = format!(" {} / {} ", curr_fmt, dur_fmt);
        let tm_y = s_y + 1;
        let tm_x = (cols as usize).saturating_sub(time_str.len()) / 2;
        execute!(stdout, 
            MoveTo(tm_x as u16, tm_y as u16), 
            SetForegroundColor(Color::White),
            Print(&time_str),
            ResetColor
        ).ok();

        // 6. Seekbar
        let bar_width = (cols as usize).saturating_sub(20).max(10);
        let bar_x = (cols as usize - bar_width) / 2;
        let bar_y = tm_y + 1;
        let progress_bar = render_progress_bar(state.current_time, state.total_duration, bar_width);
        execute!(stdout, 
            MoveTo(bar_x as u16, bar_y as u16), 
            SetForegroundColor(Color::White),
            Print(format!(" {} ", progress_bar)),
            ResetColor
        ).ok();

        // 7. Codecs
        let v_c = state.video_codec.as_deref().unwrap_or("unknown");
        let a_c = state.audio_codec.as_deref().unwrap_or("unknown");
        let codec_str = format!(" Video: {} | Audio: {} ", v_c, a_c);
        let cd_y = bar_y + 1;
        let cd_x = (cols as usize).saturating_sub(codec_str.len()) / 2;
        execute!(stdout, 
            MoveTo(cd_x as u16, cd_y as u16), 
            SetForegroundColor(Color::Grey), 
            Print(codec_str), 
            ResetColor
        ).ok();

        // 8. Volume
        let vol_str = if state.is_muted {
            " (Muted) ".to_string()
        } else {
            match state.volume_level {
                Some(v) => format!(" Vol: {:.0}% ", v * 100.0),
                None => " Vol: --% ".to_string(),
            }
        };
        let v_y = cd_y + 1;
        let v_x = (cols as usize).saturating_sub(vol_str.len()) / 2;
        execute!(stdout, 
            MoveTo(v_x as u16, v_y as u16), 
            SetForegroundColor(Color::White),
            Print(vol_str),
            ResetColor
        ).ok();

        // Footer
        let footer = " [Space] Toggle  [Arrow] Seek/Vol  [M] Mute  [Q] Quit ";
        let f_y = rows.saturating_sub(2);
        let f_x = (cols as usize).saturating_sub(footer.len()) / 2;
        execute!(stdout, 
            MoveTo(f_x as u16, f_y), 
            SetForegroundColor(Color::DarkGrey), 
            Print(footer), 
            ResetColor
        ).ok();
        
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
    
    let bar_body = "█".repeat(filled_len);
    let empty_body = "░".repeat(empty_len);

    format!("{}{}", bar_body, empty_body)
}

fn render_cube_frame(frame: usize, width: usize, height: usize) -> Vec<String> {
    let background_char = ' ';
    
    // Buffer
    let mut buffer = vec![vec![background_char; width]; height];
    let mut z_buffer = vec![vec![0.0f32; width]; height];
    
    let cube_width = (height as f32) * 0.55; 
    
    // Speed: 1 rotation (2PI) in 10s. Tick 0.15s.
    let angle_step = 0.0942;
    let a = (frame as f32) * angle_step; // Rotate X
    let b = (frame as f32) * angle_step; // Rotate Y
    let c = (frame as f32) * 0.02;       // Rotate Z (slower)

    let k1 = (height as f32) * 1.5; 
    let k2 = 60.0; // Camera distance
    
    let increment_speed = 0.45; // Surface sampling step

    let (sin_a, cos_a) = a.sin_cos();
    let (sin_b, cos_b) = b.sin_cos();
    let (sin_c, cos_c) = c.sin_cos();

    // Rotation helper
    let rotate = |x: f32, y: f32, z: f32| -> (f32, f32, f32) {
         let y1 = y * cos_a - z * sin_a;
         let z1 = y * sin_a + z * cos_a;
         let x1 = x;
         let x2 = x1 * cos_b + z1 * sin_b;
         let z2 = -x1 * sin_b + z1 * cos_b;
         let y2 = y1;
         let x3 = x2 * cos_c - y2 * sin_c;
         let y3 = x2 * sin_c + y2 * cos_c;
         (x3, y3, z2)
    };
    
    // Face Chars
    let ascii_chars = b".,-~:;=!*#$@";
    let normals = [
        (0.0, 0.0, -1.0), (0.0, 0.0, 1.0),
        (0.0, -1.0, 0.0), (0.0, 1.0, 0.0),
        (-1.0, 0.0, 0.0), (1.0, 0.0, 0.0),
    ];
    let mut face_chars = [' '; 6];
    
    let lx = 0.0f32; let ly = 1.0f32; let lz = -1.0f32;
    let l_len = (lx*lx + ly*ly + lz*lz).sqrt();
    let (lx, ly, lz) = (lx/l_len, ly/l_len, lz/l_len);

    for (i, &(nx, ny, nz)) in normals.iter().enumerate() {
        let (rx, ry, rz) = rotate(nx, ny, nz);
        let l = rx * lx + ry * ly + rz * lz;
        let luminance = ((l + 1.0) / 2.0).clamp(0.0, 1.0);
        let idx = (luminance * (ascii_chars.len() - 1) as f32) as usize;
        face_chars[i] = ascii_chars[idx] as char;
    }

    let mut x = -cube_width;
    while x <= cube_width {
        let mut y = -cube_width;
        while y <= cube_width {
             let points = [
                 (x, y, -cube_width), (x, y, cube_width),
                 (x, -cube_width, y), (x, cube_width, y),
                 (-cube_width, x, y), (cube_width, x, y),
             ];
             for (i, &(px, py, pz)) in points.iter().enumerate() {
                 let (x3, y3, z3) = rotate(px, py, pz);
                 let ooz = 1.0 / (k2 + z3);
                 let xp = (width as f32 / 2.0 + k1 * ooz * x3 * 2.0) as i32;
                 let yp = (height as f32 / 2.0 - k1 * ooz * y3) as i32;
                 if xp >= 0 && xp < width as i32 && yp >= 0 && yp < height as i32 {
                     let idx_x = xp as usize;
                     let idx_y = yp as usize;
                     if ooz > z_buffer[idx_y][idx_x] {
                         z_buffer[idx_y][idx_x] = ooz;
                         buffer[idx_y][idx_x] = face_chars[i];
                     }
                 }
             }
             y += increment_speed;
        }
        x += increment_speed;
    }
    buffer.into_iter().map(|row| row.into_iter().collect()).collect()
}

fn get_animation_frames(frame: usize, w: usize, h: usize) -> Vec<String> {
    render_cube_frame(frame, w, h)
}

fn get_static_frame(w: usize, h: usize) -> Vec<String> {
    render_cube_frame(0, w, h)
}

impl Drop for TuiController {
    fn drop(&mut self) {
        let _ = execute!(stdout(), Show, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}
