
pub fn srt_to_vtt(srt_content: &str) -> String {
    let mut vtt = String::from("WEBVTT\n\n");
    for line in srt_content.lines() {
        // Simple timestamp conversion: 00:00:00,000 --> 00:00:00.000
        // We act on lines containing the arrow
        if line.contains("-->") {
            vtt.push_str(&line.replace(',', "."));
        } else {
            vtt.push_str(line);
        }
        vtt.push('\n');
    }
    vtt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srt_to_vtt_basic() {
        let srt = "1\n00:00:20,000 --> 00:00:24,400\nHello World\n\n2\n00:00:24,600 --> 00:00:27,800\nFoo Bar";
        let vtt = srt_to_vtt(srt);
        
        assert!(vtt.starts_with("WEBVTT"));
        assert!(vtt.contains("00:00:20.000 --> 00:00:24.400"));
        assert!(vtt.contains("Hello World"));
    }
}
