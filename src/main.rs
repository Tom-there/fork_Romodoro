mod timer;
mod pid;
mod process;
mod install;

use clap::Parser;
use process::cancel_timer;
use timer::run_timer;
use install::ensure_global_install;

#[derive(Parser)]
struct Cli {
    /// Time in minutes
    time: Option<f32>,
    /// Cancel timer flag
    #[arg(long)]
    cancel: bool,
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

