//! HTTP Server for streaming local content to Cast devices.

use crate::error::CastError;
use crate::torrent::stream::GrowingFile;
use bytes::Bytes;
use librqbit::ManagedTorrent;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt, AsyncWriteExt, SeekFrom};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

// Constants for buffering
const DEFAULT_CHUNK_SIZE: usize = 1024 * 1024; // 1M
const DEFAULT_BUFFER_CAPACITY: usize = 16; // 16 chunks

#[derive(Debug, Clone, Copy)]
pub struct StreamConfig {
    pub chunk_size: usize,
    pub buffer_capacity: usize,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            chunk_size: DEFAULT_CHUNK_SIZE,
            buffer_capacity: DEFAULT_BUFFER_CAPACITY,
        }
    }
}

#[derive(Clone)]
pub enum StreamSource {
    Static(PathBuf),
    Growing {
        path: PathBuf,
        total_size: u64,
        handle: Arc<ManagedTorrent>,
        file_offset: u64,
        piece_length: u64,
    },
}

impl StreamSource {
    pub async fn open(&self) -> std::io::Result<Box<dyn AsyncReadSeek + Unpin + Send>> {
        match self {
            StreamSource::Static(p) => {
                let f = File::open(p).await?;
                Ok(Box::new(f))
            }
            StreamSource::Growing {
                path,
                total_size,
                handle,
                file_offset,
                piece_length,
            } => {
                let f = GrowingFile::open(
                    path.clone(),
                    *total_size,
                    handle.clone(),
                    *file_offset,
                    *piece_length,
                )
                .await?;
                Ok(Box::new(f))
            }
        }
    }

    pub fn get_path(&self) -> PathBuf {
        match self {
            StreamSource::Static(p) => p.clone(),
            StreamSource::Growing { path, .. } => path.clone(),
        }
    }

    pub fn total_size(&self) -> Option<u64> {
        match self {
            StreamSource::Static(_) => None,
            StreamSource::Growing { total_size, .. } => Some(*total_size),
        }
    }
}

// Trait alias for convenience
pub trait AsyncReadSeek: AsyncRead + AsyncSeek {}
impl<T: AsyncRead + AsyncSeek> AsyncReadSeek for T {}

/// Simple HTTP Server to stream a specific file.
pub struct StreamServer {
    source: Arc<Mutex<Option<StreamSource>>>,
    transcode_rx: Arc<tokio::sync::Mutex<Option<tokio::process::ChildStdout>>>,
    transcode_process: Arc<tokio::sync::Mutex<Option<tokio::process::Child>>>,
    port: u16,
}

impl Default for StreamServer {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamServer {
    pub fn new() -> Self {
        Self {
            source: Arc::new(Mutex::new(None)),
            transcode_rx: Arc::new(tokio::sync::Mutex::new(None)),
            transcode_process: Arc::new(tokio::sync::Mutex::new(None)),
            port: 0,
        }
    }

    /// Starts the HTTP server on an available port.
    /// Returns the local IP address and port where the content is served.
    pub async fn start(&mut self, local_ip: &str) -> Result<String, CastError> {
        let listener = TcpListener::bind(format!("{}:0", local_ip))
            .await
            .map_err(CastError::Io)?;

        let addr = listener.local_addr().map_err(CastError::Io)?;
        self.port = addr.port();
        let source_clone = self.source.clone();
        let transcode_rx_clone = self.transcode_rx.clone();

        println!("Streaming server listening on {}", addr);

        tokio::spawn(async move {
            loop {
                if let Ok((socket, _)) = listener.accept().await {
                    let src = source_clone.clone();
                    let trx = transcode_rx_clone.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(socket, src, trx).await {
                            eprintln!("Connection handling error: {}", e);
                        }
                    });
                }
            }
        });

        Ok(format!("http://{}:{}", local_ip, self.port))
    }

    /// Sets the source to be streamed.
    pub async fn set_source(&self, source: StreamSource) {
        {
            let mut src = self.source.lock().unwrap();
            *src = Some(source);
        }
        // Clear transcode
        self.clear_transcode().await;
    }

    /// Sets the file to be streamed (Legacy helper).
    pub async fn set_file(&self, path: PathBuf) {
        self.set_source(StreamSource::Static(path)).await;
    }

    async fn clear_transcode(&self) {
        let mut trx = self.transcode_rx.lock().await;
        *trx = None;
        let mut proc = self.transcode_process.lock().await;
        if let Some(mut child) = proc.take() {
            let _ = child.start_kill();
        }
    }

    pub async fn set_transcode_output(&self, pipeline: crate::transcode::TranscodingPipeline) {
        // Cleanup old
        self.clear_transcode().await;

        {
            let mut proc = self.transcode_process.lock().await;
            *proc = Some(pipeline.process);
        }

        let mut trx = self.transcode_rx.lock().await;
        *trx = Some(pipeline.stdout);
    }
}

async fn stream_file_buffered<R>(
    socket: &mut TcpStream,
    reader: R,
    config: StreamConfig,
    remaining: u64,
) -> std::io::Result<()>
where
    R: AsyncRead + Unpin + Send + 'static,
{
    let (tx, mut rx) = mpsc::channel(config.buffer_capacity);

    // Spawn producer
    tokio::spawn(producer_task(reader, tx, config.chunk_size, remaining));

    // Consumer loop
    while let Some(res) = rx.recv().await {
        match res {
            Ok(chunk) => {
                socket.write_all(&chunk).await?;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

async fn handle_connection(
    mut socket: TcpStream,
    source_arc: Arc<Mutex<Option<StreamSource>>>,
    transcode_rx: Arc<tokio::sync::Mutex<Option<tokio::process::ChildStdout>>>,
) -> std::io::Result<()> {
    let mut buf = [0; 1024];
    let n = socket.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buf[..n]);

    // Check transcode first
    {
        let mut trx = transcode_rx.lock().await;
        if let Some(stdout) = trx.as_mut() {
            let status_line = "HTTP/1.1 200 OK";
            let header = format!(
                "{} \r\n\
                Content-Type: video/mp4\r\n\
                Connection: keep-alive\r\n\
                Transfer-Encoding: chunked\r\n\
                \r\n",
                status_line
            );
            socket.write_all(header.as_bytes()).await?;

            let mut pipe_buf = [0u8; 65536];
            loop {
                let n = stdout.read(&mut pipe_buf).await?;
                if n == 0 {
                    socket.write_all(b"0\r\n\r\n").await?;
                    break;
                }
                let size_str = format!("{:X}\r\n", n);
                socket.write_all(size_str.as_bytes()).await?;
                socket.write_all(&pipe_buf[..n]).await?;
                socket.write_all(b"\r\n").await?;
            }
            return Ok(());
        }
    }

    // Extract Range header
    let range_header = request
        .lines()
        .find(|line| line.starts_with("Range: bytes="))
        .map(|line| line.trim_start_matches("Range: bytes=").trim());

    // Get source
    let source = {
        let s = source_arc.lock().unwrap();
        match s.as_ref() {
            Some(src) => src.clone(),
            None => return Ok(()), // 404
        }
    };

    let path = source.get_path();
    let mime_type = get_mime_type(&path);

    // Open stream
    let mut stream = source.open().await?;

    // Determine size
    let file_size = if let Some(s) = source.total_size() {
        s
    } else {
        tokio::fs::metadata(&path).await?.len()
    };

    let (start, end) = parse_range(range_header, file_size);
    let length = end - start + 1;

    stream.seek(SeekFrom::Start(start)).await?;

    let status_line = if range_header.is_some() {
        "HTTP/1.1 206 Partial Content"
    } else {
        "HTTP/1.1 200 OK"
    };

    let header = format!(
        "{} \r\n\
        Content-Type: {}\r\n\
        Content-Length: {}\r\n\
        Content-Range: bytes {}-{}/{}\r\n\
        Connection: keep-alive\r\n\
        Accept-Ranges: bytes\r\n\
        \r\n",
        status_line, mime_type, length, start, end, file_size
    );

    socket.write_all(header.as_bytes()).await?;

    let config = StreamConfig::default();
    stream_file_buffered(&mut socket, stream, config, length).await?;

    Ok(())
}

async fn producer_task<R>(
    mut reader: R,
    tx: mpsc::Sender<Result<Bytes, std::io::Error>>,
    chunk_size: usize,
    mut remaining: u64,
) -> std::io::Result<()>
where
    R: AsyncRead + Unpin,
{
    let mut buffer = vec![0u8; chunk_size];

    while remaining > 0 {
        let to_read = std::cmp::min(chunk_size as u64, remaining);
        match reader.read_exact(&mut buffer[..to_read as usize]).await {
            Ok(_) => {
                let chunk = Bytes::copy_from_slice(&buffer[..to_read as usize]);
                if tx.send(Ok(chunk)).await.is_err() {
                    break;
                }
                remaining -= to_read;
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    break;
                }
                let _ = tx.send(Err(e)).await;
                break;
            }
        }
    }
    Ok(())
}

fn parse_range(range: Option<&str>, file_size: u64) -> (u64, u64) {
    if let Some(r) = range {
        if let Some((start_str, end_str)) = r.split_once('-') {
            if start_str.is_empty() {
                if let Ok(suffix) = end_str.parse::<u64>() {
                    return (file_size.saturating_sub(suffix), file_size - 1);
                }
            }

            let start = start_str.parse::<u64>().unwrap_or(0);
            let end = if end_str.is_empty() {
                file_size - 1
            } else {
                end_str.parse::<u64>().unwrap_or(file_size - 1)
            };
            return (start, std::cmp::min(end, file_size - 1));
        }
    }
    (0, file_size - 1)
}

pub fn get_mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("mp4") | Some("m4v") => "video/mp4",
        Some("webm") => "video/webm",
        Some("mkv") => "video/x-matroska",
        Some("avi") => "video/x-msvideo",
        Some("mp3") => "audio/mpeg",
        Some("aac") => "audio/aac",
        Some("wav") => "audio/wav",
        Some("ogg") => "audio/ogg",
        Some("flac") => "audio/flac",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_mime_type_detection() {
        assert_eq!(get_mime_type(Path::new("video.mp4")), "video/mp4");
        assert_eq!(get_mime_type(Path::new("song.mp3")), "audio/mpeg");
        assert_eq!(get_mime_type(Path::new("image.jpg")), "image/jpeg");
        assert_eq!(
            get_mime_type(Path::new("unknown.xyz")),
            "application/octet-stream"
        );
    }

    #[test]
    fn test_range_parsing() {
        let size = 1000;
        assert_eq!(parse_range(Some("0-499"), size), (0, 499));
        assert_eq!(parse_range(Some("500-"), size), (500, 999));
        assert_eq!(parse_range(Some("-500"), size), (500, 999));
        assert_eq!(parse_range(None, size), (0, 999));
    }

    #[tokio::test]
    async fn test_producer_task_basic() {
        let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        let cursor = std::io::Cursor::new(data.clone());
        let (tx, mut rx) = mpsc::channel(10);

        let chunk_size = 3;
        let total_len = data.len() as u64;

        tokio::spawn(async move {
            let _ = producer_task(cursor, tx, chunk_size, total_len).await;
        });

        let mut received_data = Vec::new();
        while let Some(res) = rx.recv().await {
            let chunk = res.expect("Should not fail");
            received_data.extend_from_slice(&chunk);
        }

        assert_eq!(received_data, data);
    }

    #[tokio::test]
    async fn test_producer_task_partial_read() {
        let data = vec![1u8, 2, 3, 4, 5];
        let cursor = std::io::Cursor::new(data.clone());
        let (tx, mut rx) = mpsc::channel(10);

        let chunk_size = 2;
        let total_len = data.len() as u64;

        tokio::spawn(async move {
            let _ = producer_task(cursor, tx, chunk_size, total_len).await;
        });

        let mut received_data = Vec::new();
        while let Some(res) = rx.recv().await {
            let chunk = res.expect("Should not fail");
            received_data.extend_from_slice(&chunk);
        }

        assert_eq!(received_data, data);
    }
}
