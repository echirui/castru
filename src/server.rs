
//! HTTP Server for streaming local content to Cast devices.

use std::path::{Path, PathBuf};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncSeekExt, SeekFrom};
use tokio::fs::File;
use std::sync::{Arc, Mutex};
use crate::error::CastError;

/// Simple HTTP Server to stream a specific file.
pub struct StreamServer {
    file_path: Arc<Mutex<Option<PathBuf>>>,
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
            file_path: Arc::new(Mutex::new(None)),
            port: 0,
        }
    }

    /// Starts the HTTP server on an available port.
    /// Returns the local IP address and port where the content is served.
    pub async fn start(&mut self, local_ip: &str) -> Result<String, CastError> {
        let listener = TcpListener::bind(format!("{}:0", local_ip)).await
            .map_err(CastError::Io)?;
        
        let addr = listener.local_addr().map_err(CastError::Io)?;
        self.port = addr.port();
        let file_path_clone = self.file_path.clone();

        println!("Streaming server listening on {}", addr);

        tokio::spawn(async move {
            loop {
                if let Ok((socket, _)) = listener.accept().await {
                    let fp = file_path_clone.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(socket, fp).await {
                            eprintln!("Connection handling error: {}", e);
                        }
                    });
                }
            }
        });

        Ok(format!("http://{}:{}", local_ip, self.port))
    }

    /// Sets the file to be streamed.
    pub fn set_file(&self, path: PathBuf) {
        let mut fp = self.file_path.lock().unwrap();
        *fp = Some(path);
    }
}

async fn handle_connection(mut socket: TcpStream, file_path: Arc<Mutex<Option<PathBuf>>>) -> std::io::Result<()> {
    let mut buf = [0; 1024];
    let n = socket.read(&mut buf).await?;
    if n == 0 { return Ok(()); }

    let request = String::from_utf8_lossy(&buf[..n]);
    
    // Extract Range header
    let range_header = request.lines()
        .find(|line| line.starts_with("Range: bytes="))
        .map(|line| line.trim_start_matches("Range: bytes=").trim());

    // Get file path
    let path = {
        let fp = file_path.lock().unwrap();
        match fp.as_ref() {
            Some(p) => p.clone(),
            None => return Ok(()), // 404
        }
    };

    let mut file = File::open(&path).await?;
    let metadata = file.metadata().await?;
    let file_size = metadata.len();
    let mime_type = get_mime_type(&path);

    let (start, end) = parse_range(range_header, file_size);
    let length = end - start + 1;

    file.seek(SeekFrom::Start(start)).await?;

    let status_line = if range_header.is_some() { "HTTP/1.1 206 Partial Content" } else { "HTTP/1.1 200 OK" };
    
    let header = format!(
        "{}\r\n\
        Content-Type: {}\r\n\
        Content-Length: {}\r\n\
        Content-Range: bytes {}-{}/{}\r\n\
        Connection: keep-alive\r\n\
        Accept-Ranges: bytes\r\n\
        \r\n",
        status_line, mime_type, length, start, end, file_size
    );

    socket.write_all(header.as_bytes()).await?;

    // Streaming loop
    let mut buffer = vec![0; 64 * 1024]; // 64KB Chunk
    let mut remaining = length;

    while remaining > 0 {
        let to_read = std::cmp::min(buffer.len() as u64, remaining);
        let n = file.read(&mut buffer[..to_read as usize]).await?;
        if n == 0 { break; }
        
        socket.write_all(&buffer[..n]).await?;
        remaining -= n as u64;
    }

    Ok(())
}

fn parse_range(range: Option<&str>, file_size: u64) -> (u64, u64) {
    if let Some(r) = range {
        if let Some((start_str, end_str)) = r.split_once('-') {
            // Suffix range: -500 (Last 500 bytes)
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

/// Helper to guess MIME type from file extension.
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
        assert_eq!(get_mime_type(Path::new("unknown.xyz")), "application/octet-stream");
    }

    #[test]
    fn test_range_parsing() {
        let size = 1000;
        // Test assumes input string is already stripped of "Range: bytes=" prefix
        assert_eq!(parse_range(Some("0-499"), size), (0, 499));
        assert_eq!(parse_range(Some("500-"), size), (500, 999));
        assert_eq!(parse_range(Some("-500"), size), (500, 999)); // Last 500 bytes
        assert_eq!(parse_range(None, size), (0, 999));
    }
}
