use std::{thread, time::Duration, io::Write, process::Command};
use clap::Parser;
use native_dialog::MessageDialog;

#[derive(Parser)]
struct Cli {
    /// Time in minutes
    time: f32,

    /// Run the timer in the background
    #[arg(long)]
    background: bool,
}

fn main() {
    let args = Cli::parse();

    // If --background is specified, run the timer logic directly
    if args.background {
        run_timer(args.time);
        return;
    }

    // Run as a detached process
    println!("Starting Pomodoro timer for {} minutes in the background...", args.time);

    Command::new(std::env::current_exe().unwrap())
        .args(&["--background", &args.time.to_string()])
        .spawn()
        .expect("Failed to start the timer in the background.");

    println!("Timer is running in the background. You can continue working in this terminal.");
}

/// Timer Logic
fn run_timer(time: f32) {
    let duration_seconds = (time * 60.0).round() as i32;

    println!("Pomodoro Timer started for {:.1} minutes.", time);
    for remaining in (0..duration_seconds).rev() {

        std::io::stdout().flush().unwrap();

        thread::sleep(Duration::from_secs(1));
    }

    println!("\nTime's up! Take a break!");
    MessageDialog::new()
        .set_title("Pomodoro Timer")
        .set_text("Time's up! Take a break!")
        .show_alert()
        .unwrap();
}

