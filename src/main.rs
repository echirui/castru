use castru::config::Config;
use castru::app::{CastNowCore, scan_devices, connect_only, launch_app};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Panic hook for TUI cleanup using crossterm
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        // Try to restore terminal
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::cursor::Show
        );
        default_hook(info);
    }));

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "scan" => {
            scan_devices().await?;
        }
        "cast" => {
            if args.len() < 3 {
                println!("Usage: castru cast [OPTIONS] <FILE_OR_URL> [FILE_OR_URL...]");
                println!("Options:");
                println!("  --ip <IP>      Connect to specific IP");
                println!("  --name <NAME>  Connect to device with specific Friendly Name");
                return Ok(());
            }

            let cast_args = &args[2..];
            // Since we pass slice of Strings, and Config::parse expects &[String]
            let config = Config::parse(cast_args);
            let app = CastNowCore::new(config);
            app.run().await?;
        }
        "launch" => {
            if args.len() < 4 {
                println!("Usage: castru launch <IP> <APP_ID>");
                return Ok(());
            }
            let ip = &args[2];
            let app_id = &args[3];
            launch_app(ip, app_id).await?;
        }
        "connect" => {
            if args.len() < 3 {
                println!("Usage: castru connect <IP>");
                return Ok(());
            }
            let ip = &args[2];
            connect_only(ip).await?;
        }
        _ => {
            print_usage();
        }
    }

    Ok(())
}

fn print_usage() {
    println!("Usage:");
    println!("  castru scan");
    println!("  castru cast [OPTIONS] <FILE_OR_URL> [FILE_OR_URL...]");
    println!("  castru connect <IP>");
    println!("  castru launch <IP> <APP_ID>");
    println!();
    println!("Options for 'cast':");
    println!("  --ip <IP>      Connect to specific IP");
    println!("  --name <NAME>  Connect to device with specific Friendly Name");
    println!("  --log <FILE>   Output logs to specific file");
    println!("  --myip <IP>    Specify local interface IP to bind to");
    println!("  --port <PORT>  Specify internal server port");
    println!("  --subtitles <FILE>  Load sidecar subtitle file");
    println!("  --volume <0.0-1.0>  Set initial volume");
    println!("  --loop         Loop the playlist");
    println!("  --quiet        Suppress non-critical output");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_usage() {
        // Just verify it runs
        print_usage();
    }
}
