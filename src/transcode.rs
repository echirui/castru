use crate::error::CastError;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::{Child, ChildStdout, Command};

#[derive(Debug, Clone)]
pub struct MediaProbeResult {
    pub video_codec: Option<String>,
    pub video_profile: Option<String>,
    pub pix_fmt: Option<String>,
    pub audio_codec: Option<String>,
    pub duration: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct TranscodeConfig {
    pub input_path: PathBuf,
    pub start_time: f64,
    pub target_video_codec: String,
    pub target_audio_codec: String,
}

#[derive(Deserialize)]
struct FFProbeOutput {
    streams: Vec<FFProbeStream>,
    format: Option<FFProbeFormat>,
}

#[derive(Deserialize)]
struct FFProbeStream {
    codec_type: String,
    codec_name: String,
    #[serde(default)]
    profile: Option<String>,
    #[serde(default)]
    pix_fmt: Option<String>,
}

#[derive(Deserialize)]
struct FFProbeFormat {
    duration: Option<String>,
}

pub struct TranscodingPipeline {
    pub process: Child,
    pub stdout: ChildStdout,
}

pub async fn probe_media(path: &Path) -> Result<MediaProbeResult, CastError> {
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("quiet")
        .arg("-print_format")
        .arg("json")
        .arg("-show_format")
        .arg("-show_streams")
        .arg(path)
        .output()
        .await
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                CastError::Probe("ffprobe not found. Please install ffmpeg.".to_string())
            } else {
                CastError::Probe(format!("Failed to execute ffprobe: {}", e))
            }
        })?;

    if !output.status.success() {
        return Err(CastError::Probe(
            "ffprobe returned non-zero exit code".to_string(),
        ));
    }

    let parsed: FFProbeOutput = serde_json::from_slice(&output.stdout)
        .map_err(|e| CastError::Probe(format!("Failed to parse ffprobe output: {}", e)))?;

    let mut video_codec = None;
    let mut video_profile = None;
    let mut pix_fmt = None;
    let mut audio_codec = None;

    for stream in parsed.streams {
        if stream.codec_type == "video" && video_codec.is_none() {
            video_codec = Some(stream.codec_name);
            video_profile = stream.profile;
            pix_fmt = stream.pix_fmt;
        } else if stream.codec_type == "audio" && audio_codec.is_none() {
            audio_codec = Some(stream.codec_name);
        }
    }

    let duration = parsed
        .format
        .and_then(|f| f.duration)
        .and_then(|d| d.parse::<f64>().ok());

    Ok(MediaProbeResult {
        video_codec,
        video_profile,
        pix_fmt,
        audio_codec,
        duration,
    })
}

pub fn needs_transcoding(probe: &MediaProbeResult) -> bool {
    // If it has video, check if h264
    if let Some(ref v) = probe.video_codec {
        if v != "h264" {
            return true;
        }

        // Chromecast usually requires 8-bit h264
        if let Some(ref fmt) = probe.pix_fmt {
            if fmt.contains("10le") || fmt.contains("12le") || fmt.contains("10be") {
                return true;
            }
        }
        if let Some(ref prof) = probe.video_profile {
            if prof.contains("High 10")
                || prof.contains("High 4:2:2")
                || prof.contains("High 4:4:4")
            {
                return true;
            }
        }
    }
    // If it has audio, check if aac
    if let Some(ref a) = probe.audio_codec {
        if a != "aac" && a != "mp3" {
            return true;
        } // mp3 also usually supported, but safe to transcode if not aac
    }

    // If we have neither, let's assume no (or fail elsewhere)
    false
}

pub fn spawn_ffmpeg(config: &TranscodeConfig) -> Result<TranscodingPipeline, CastError> {
    let mut cmd = Command::new("ffmpeg");

    // Seek
    if config.start_time > 0.0 {
        cmd.arg("-ss").arg(config.start_time.to_string());
    }

    cmd.arg("-i")
        .arg(&config.input_path)
        .arg("-c:v")
        .arg(&config.target_video_codec)
        .arg("-pix_fmt")
        .arg("yuv420p") // Ensure 8-bit output
        // Preset for speed
        .arg("-preset")
        .arg("ultrafast")
        .arg("-c:a")
        .arg(&config.target_audio_codec)
        // Output format: mp4 fragmented for piping
        .arg("-f")
        .arg("mp4")
        .arg("-movflags")
        .arg("frag_keyframe+empty_moov+default_base_moof")
        // Pipe to stdout
        .arg("pipe:1")
        .stdout(Stdio::piped())
        .stderr(Stdio::null()); // Silence stderr for now, or maybe pipe to log

    let mut process = cmd
        .spawn()
        .map_err(|e| CastError::Transcoding(format!("Failed to spawn ffmpeg: {}", e)))?;

    let stdout = process
        .stdout
        .take()
        .ok_or_else(|| CastError::Transcoding("Failed to capture ffmpeg stdout".to_string()))?;

    Ok(TranscodingPipeline { process, stdout })
}
