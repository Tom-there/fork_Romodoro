use clap::Parser;
use native_dialog::MessageDialog;
use std::{env, io::Write, process::Command, thread, time::Duration};

#[derive(Parser)]
struct Cli {
    /// Time in minutes
    time: f32,

}

fn main() {
    let args = Cli::parse();

    if env::var("POMODORO_CHILD").is_err() {
         Command::new(std::env::current_exe().unwrap())
        .args(&[&args.time.to_string()])
        .env("POMODORO_CHILD", "true")
        .spawn()
        .expect("Failed to start the timer in the background.");
 
        return;
    }

    run_timer(args.time);
}


/// Timer Logic
fn run_timer(time: f32) {
    let duration_seconds = (time * 60.0).round() as i32;

    println!("Pomodoro Timer started for {:.1} minutes.", time);
    for _ in (0..duration_seconds).rev() {
        std::io::stdout().flush().unwrap();

        thread::sleep(Duration::from_secs(1));
    }

    MessageDialog::new()
        .set_title("Pomodoro Timer")
        .set_text("Time's up! Take a break!")
        .show_alert()
        .unwrap();
}
