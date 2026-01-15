use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

pub mod manager;
pub mod stream;

pub use manager::TorrentManager;
pub use stream::GrowingFile;

#[derive(Debug, Clone)]
pub struct TorrentConfig {
    pub download_dir: Option<PathBuf>,
    pub keep_files: bool,
    pub listen_port: Option<u16>,
}

impl Default for TorrentConfig {
    fn default() -> Self {
        Self {
            download_dir: None,
            keep_files: false,
            listen_port: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TorrentState {
    Resolving,
    Buffering,
    Playing,
    Finished,
}

#[derive(Debug)]
pub struct TorrentSession {
    pub session_id: Uuid,
    pub save_path: PathBuf,
    pub target_file_index: usize,
    pub state: TorrentState,
}

#[derive(Error, Debug)]
pub enum TorrentError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Magnet link error: {0}")]
    Magnet(String),
    #[error("Torrent engine error: {0}")]
    Engine(String),
    #[error("No video file found in torrent")]
    NoVideoFound,
}