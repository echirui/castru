use super::{TorrentConfig, TorrentError};
use bstr::ByteSlice;
use librqbit::{
    AddTorrent, AddTorrentOptions, AddTorrentResponse, ManagedTorrent, Session, SessionOptions,
};
use std::path::PathBuf;
use std::sync::Arc;

pub struct TorrentManager {
    session: Arc<Session>,
    output_dir: PathBuf,
}

impl TorrentManager {
    pub async fn new(config: TorrentConfig) -> Result<Self, TorrentError> {
        let output_dir = config
            .download_dir
            .unwrap_or_else(|| {
                let mut path = std::env::temp_dir();
                path.push(format!("castru_torrent_{}", uuid::Uuid::new_v4()));
                path
            });
        tokio::fs::create_dir_all(&output_dir)
            .await
            .map_err(TorrentError::Io)?;

        let session = Session::new_with_opts(
            output_dir.clone(),
            SessionOptions {
                disable_dht_persistence: true,
                ..Default::default()
            },
        )
            .await
            .map_err(|e| TorrentError::Engine(e.to_string()))?;
        Ok(Self {
            session,
            output_dir,
        })
    }

    pub async fn start_magnet(&self, uri: &str) -> Result<super::TorrentStreamInfo, TorrentError> {
        let response = self
            .session
            .add_torrent(
                AddTorrent::from_url(uri),
                Some(AddTorrentOptions {
                    overwrite: true,
                    ..Default::default()
                }),
            )
            .await
            .map_err(|e| TorrentError::Engine(e.to_string()))?;

        let handle = match response {
            AddTorrentResponse::Added(_, h) => h,
            AddTorrentResponse::AlreadyManaged(_, h) => h,
            _ => return Err(TorrentError::Engine("Unexpected AddTorrentResponse".into())),
        };


        self.get_info(handle).await
    }

    pub async fn start_torrent_file(
        &self,
        path: &str,
    ) -> Result<super::TorrentStreamInfo, TorrentError> {
        let content = tokio::fs::read(path).await.map_err(TorrentError::Io)?;
        let response = self
            .session
            .add_torrent(
                AddTorrent::from_bytes(content),
                Some(AddTorrentOptions {
                    overwrite: true,
                    ..Default::default()
                }),
            )
            .await
            .map_err(|e| TorrentError::Engine(e.to_string()))?;

        let handle = match response {
            AddTorrentResponse::Added(_, h) => h,
            AddTorrentResponse::AlreadyManaged(_, h) => h,
            _ => return Err(TorrentError::Engine("Unexpected AddTorrentResponse".into())),
        };


        self.get_info(handle).await
    }

    async fn get_info(
        &self,
        handle: Arc<ManagedTorrent>,
    ) -> Result<super::TorrentStreamInfo, TorrentError> {
        let init_future = handle.wait_until_initialized();
        let timeout_duration = std::time::Duration::from_secs(30); // 30s timeout for metadata

        tokio::time::timeout(timeout_duration, init_future)
            .await
            .map_err(|_| TorrentError::Engine("Timeout waiting for torrent metadata".into()))?
            .map_err(|e| TorrentError::Engine(e.to_string()))?;

        let metadata_guard = handle.metadata.load();
        let metadata = metadata_guard
            .as_ref()
            .ok_or(TorrentError::Engine("Metadata not loaded".into()))?;
        let info = &metadata.info;

        if let Some(len) = info.length {
            let name = info
                .name
                .as_ref()
                .map(|b| b.to_str_lossy())
                .unwrap_or("unknown".into());
            let full_path = self.output_dir.join(name.as_ref());
            return Ok(super::TorrentStreamInfo {
                handle,
                path: full_path,
                total_size: len,
                file_offset: 0,
                piece_length: info.piece_length as u64,
                file_idx: 0,
            });
        }

        let files = info.files.as_ref().ok_or(TorrentError::NoVideoFound)?;

        let mut largest_idx = None;
        let mut max_size: u64 = 0;

        for (idx, file) in files.iter().enumerate() {
            let path_str = file
                .path
                .iter()
                .map(|c| c.to_str_lossy())
                .collect::<Vec<_>>()
                .join("/");
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

        let rel_path = target_file
            .path
            .iter()
            .map(|c| c.to_str_lossy().into_owned())
            .collect::<PathBuf>();
        let base_name = info
            .name
            .as_ref()
            .map(|b| b.to_str_lossy())
            .unwrap_or("".into());
        let full_path = self.output_dir.join(base_name.as_ref()).join(rel_path);

        let mut file_offset = 0;
        for i in 0..file_idx {
            file_offset += files[i].length;
        }

        Ok(super::TorrentStreamInfo {
            handle,
            path: full_path,
            total_size: max_size,
            file_offset,
            piece_length: info.piece_length as u64,
            file_idx,
        })
    }

    pub fn cleanup(&self) {
        if self.output_dir.exists() {
             let _ = std::fs::remove_dir_all(&self.output_dir);
        }
    }
}

impl Drop for TorrentManager {
    fn drop(&mut self) {
        if self.output_dir.exists() {
             let _ = std::fs::remove_dir_all(&self.output_dir);
        }
    }
}

fn is_video_file(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.ends_with(".mp4")
        || lower.ends_with(".mkv")
        || lower.ends_with(".avi")
        || lower.ends_with(".mov")
}
