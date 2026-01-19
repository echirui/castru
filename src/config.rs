
#[derive(Debug, Clone, Default)]
pub struct Config {
    pub target_ip: Option<String>,
    pub target_name: Option<String>,
    pub log_file: Option<String>,
    pub inputs: Vec<String>,
    pub myip: Option<String>,
    pub port: Option<u16>,
    pub subtitles: Option<String>,
    pub volume: Option<f32>,
    pub loop_playlist: bool,
    pub quiet: bool,
}

impl Config {
    pub fn parse(args: &[String]) -> Self {
        let mut target_ip = None;
        let mut target_name = None;
        let mut log_file = None;
        let mut inputs = Vec::new();
        let mut myip = None;
        let mut port = None;
        let mut subtitles = None;
        let mut volume = None;
        let mut loop_playlist = false;
        let mut quiet = false;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--ip" => {
                    if i + 1 < args.len() {
                        target_ip = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "--name" => {
                    if i + 1 < args.len() {
                        target_name = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "--log" => {
                    if i + 1 < args.len() {
                        log_file = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "--myip" => {
                    if i + 1 < args.len() {
                        myip = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "--port" => {
                    if i + 1 < args.len() {
                        if let Ok(p) = args[i + 1].parse::<u16>() {
                            port = Some(p);
                        }
                        i += 1;
                    }
                }
                "--subtitles" => {
                    if i + 1 < args.len() {
                        subtitles = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "--volume" => {
                    if i + 1 < args.len() {
                        if let Ok(v) = args[i + 1].parse::<f32>() {
                            volume = Some(v);
                        }
                        i += 1;
                    }
                }
                "--loop" => {
                    loop_playlist = true;
                }
                "--quiet" => {
                    quiet = true;
                }
                val => {
                    inputs.push(val.to_string());
                }
            }
            i += 1;
        }

        Self {
            target_ip,
            target_name,
            log_file,
            inputs,
            myip,
            port,
            subtitles,
            volume,
            loop_playlist,
            quiet,
        }
    }
}
