use librqbit::ManagedTorrent;
use std::io::{self, SeekFrom};
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncSeek, ReadBuf};

pub struct GrowingFile {
    file: Option<File>,
    path: PathBuf,
    position: u64,
    total_size: u64,
    handle: Arc<ManagedTorrent>,
    file_offset: u64,
    piece_length: u64,
}

impl GrowingFile {
    pub async fn open(
        path: PathBuf,
        total_size: u64,
        handle: Arc<ManagedTorrent>,
        file_offset: u64,
        piece_length: u64,
    ) -> io::Result<Self> {
        let file = File::open(&path).await?;
        Ok(Self {
            file: Some(file),
            path,
            position: 0,
            total_size,
            handle,
            file_offset,
            piece_length,
        })
    }

    pub fn total_size(&self) -> u64 {
        self.total_size
    }
}

impl AsyncRead for GrowingFile {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {

        let file = match self.file.as_mut() {
            Some(f) => f,
            None => return Poll::Ready(Err(io::Error::other("File not open"))),
        };

        let file_pin = Pin::new(file);
        let filled_before = buf.filled().len();

        match file_pin.poll_read(cx, buf) {
            Poll::Ready(Ok(())) => {
                let filled_after = buf.filled().len();
                let bytes_read = filled_after - filled_before;

                if bytes_read > 0 {
                    self.position += bytes_read as u64;
                    Poll::Ready(Ok(()))
                } else if self.position >= self.total_size {
                    Poll::Ready(Ok(()))
                } else {
                    // EOF from file but we expect more data
                    let waker = cx.waker().clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                        waker.wake();
                    });
                    Poll::Pending
                }
            }
            other => other,
        }
    }
}

impl AsyncSeek for GrowingFile {
    fn start_seek(mut self: Pin<&mut Self>, position: SeekFrom) -> io::Result<()> {
        if let Some(file) = self.file.as_mut() {
            Pin::new(file).start_seek(position)
        } else {
            Err(io::Error::other("File not open"))
        }
    }

    fn poll_complete(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<u64>> {
        if let Some(file) = self.file.as_mut() {
            match Pin::new(file).poll_complete(cx) {
                Poll::Ready(Ok(pos)) => {
                    self.position = pos;
                    Poll::Ready(Ok(pos))
                }
                other => other,
            }
        } else {
            Poll::Ready(Err(io::Error::other("File not open")))
        }
    }
}
