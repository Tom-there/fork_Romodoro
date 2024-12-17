use native_dialog::MessageDialog;
use std::{process, thread, time::Duration, io::Write};

use crate::pid::{cleanup_pid_file, get_pid_file, save_pid};

/// Runs the Pomodoro timer
pub fn run_timer(time: f32) {
    let pid_file = get_pid_file();
    save_pid(process::id());

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

    cleanup_pid_file(pid_file);
}

