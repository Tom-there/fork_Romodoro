mod file_utils;
mod install;
mod process;
mod timer;

use clap::Parser;
use install::ensure_global_install;
use process::{cancel_timer, show_status};
use timer::run_timer;

#[derive(Parser)]
struct Cli {
    // Time in minutes
    #[arg(value_parser = parse_time)]
    time: Option<f32>,

    // Cancel timer flag
    #[arg(long)]
    cancel: bool,

    // View Timer Status
    #[arg(long)]
    status: bool,
}

fn main() {
    let installed = ensure_global_install();
    if !installed {
        return;
    }

    let args = Cli::parse();

    if args.cancel {
        cancel_timer();
        return;
    }

    if args.status {
        show_status();
        return;
    }

    let time = match args.time {
        Some(t) => t,
        None => {
            eprintln!("Error: You must specify a time in minutes. Example: 'romo 25'");
            std::process::exit(1);
        }
    };

    if std::env::var("POMODORO_CHILD").is_err() {
        std::process::Command::new(std::env::current_exe().unwrap())
            .args(&[&time.to_string()])
            .env("POMODORO_CHILD", "true")
            .spawn()
            .expect("Failed to start the timer in the background.");

        return;
    }

    run_timer(time);
}

fn parse_time(s: &str) -> Result<f32, String> {
    let formatted = if s.starts_with('.') {
        format!("0{}", s)
    } else {
        s.to_string()
    };
    formatted
        .parse::<f32>()
        .map_err(|_| "Invalid time format. Example: '0.25' or '25'".to_string())
}
