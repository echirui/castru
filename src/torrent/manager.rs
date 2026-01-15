use super::{TorrentConfig, TorrentError};
use super::stream::GrowingFile;
use librqbit::{Session, AddTorrentOptions, AddTorrent, ManagedTorrent, AddTorrentResponse};
use std::path::PathBuf;
use std::sync::Arc;
use bstr::ByteSlice;

pub struct TorrentManager {
    session: Arc<Session>,
    output_dir: PathBuf,
}

impl TorrentManager {
    pub async fn new(config: TorrentConfig) -> Result<Self, TorrentError> {
        let output_dir = config.download_dir.unwrap_or_else(|| std::env::temp_dir().join("castru_torrent"));
        tokio::fs::create_dir_all(&output_dir).await.map_err(TorrentError::Io)?;

        let session = Session::new(output_dir.clone()).await.map_err(|e| TorrentError::Engine(e.to_string()))?;
        Ok(Self { session, output_dir })
    }

    pub async fn start_magnet(&self, uri: &str) -> Result<(GrowingFile, PathBuf), TorrentError> {
        let response = self.session.add_torrent(AddTorrent::from_url(uri), Some(AddTorrentOptions::default()))
            .await
            .map_err(|e| TorrentError::Engine(e.to_string()))?;
        
        let handle = match response {
            AddTorrentResponse::Added(_, h) => h,
            AddTorrentResponse::AlreadyManaged(_, h) => h,
            _ => return Err(TorrentError::Engine("Unexpected AddTorrentResponse".into())),
        };

        self.setup_stream(handle).await
    }

    pub async fn start_torrent_file(&self, path: &str) -> Result<(GrowingFile, PathBuf), TorrentError> {
        let content = tokio::fs::read(path).await.map_err(TorrentError::Io)?;
        let response = self.session.add_torrent(AddTorrent::from_bytes(content), Some(AddTorrentOptions::default()))
            .await
            .map_err(|e| TorrentError::Engine(e.to_string()))?;

        let handle = match response {
            AddTorrentResponse::Added(_, h) => h,
            AddTorrentResponse::AlreadyManaged(_, h) => h,
            _ => return Err(TorrentError::Engine("Unexpected AddTorrentResponse".into())),
        };

        self.setup_stream(handle).await
    }

    async fn setup_stream(&self, handle: Arc<ManagedTorrent>) -> Result<(GrowingFile, PathBuf), TorrentError> {
        handle.wait_until_initialized().await.map_err(|e| TorrentError::Engine(e.to_string()))?;

        let metadata_guard = handle.metadata.load();
        let metadata = metadata_guard.as_ref().ok_or(TorrentError::Engine("Metadata not loaded".into()))?;
        let info = &metadata.info;

        // Check for single file
        if let Some(len) = info.length {
            let name = info.name.as_ref().map(|b| b.to_str_lossy()).unwrap_or("unknown".into());
            let full_path = self.output_dir.join(name.as_ref());
            let growing = GrowingFile::open(full_path.clone(), len).await.map_err(TorrentError::Io)?;
            return Ok((growing, full_path));
        }

        // Multi-file
        let files = info.files.as_ref().ok_or(TorrentError::NoVideoFound)?;
        
        let mut largest_idx = None;
        let mut max_size: u64 = 0;
        
        for (idx, file) in files.iter().enumerate() {
            let path_str = file.path.iter().map(|c| c.to_str_lossy()).collect::<Vec<_>>().join("/");
             if is_video_file(&path_str) {
                let size = file.length;
                if size > max_size {
                    max_size = size;
                    largest_idx = Some(idx);
                }
            }
        }
        
        let file_idx = largest_idx.ok_or(TorrentError::NoVideoFound)?;
        let target_file = &files[file_idx];
        
        let rel_path = target_file.path.iter().map(|c| c.to_str_lossy().into_owned()).collect::<PathBuf>();
        let base_name = info.name.as_ref().map(|b| b.to_str_lossy()).unwrap_or("".into());
        let full_path = self.output_dir.join(base_name.as_ref()).join(rel_path);

        let growing = GrowingFile::open(full_path.clone(), max_size).await.map_err(TorrentError::Io)?;
        
        Ok((growing, full_path))
    }
}

fn is_video_file(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.ends_with(".mp4") || lower.ends_with(".mkv") || lower.ends_with(".avi") || lower.ends_with(".mov")
}
