use native_dialog::MessageDialog;
use std::{fs, io::Write, path::Path, process, thread, time::Duration};

use crate::pid::{cleanup_pid_file, get_pid_file, save_pid};

/// Runs the Pomodoro timer
pub fn run_timer(time: f32) {
    if is_timer_running() {
        println!("A timer is already running. Please cancel or wait until it completes.");
        return;
    }

    let pid_file = get_pid_file();
    save_pid(process::id());

    let duration_seconds = (time * 60.0).round() as i32;

    println!("Pomodoro Timer started for {:.1} minutes.", time);
    for _ in (1..duration_seconds).rev() {
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

pub fn is_timer_running() -> bool {
    let pid_file = get_pid_file();

    if !Path::new(&pid_file).exists() {
        return false;
    }

    if let Ok(pid) = fs::read_to_string(&pid_file) {
        if let Ok(pid_num) = pid.trim().parse::<u32>() {
            if let Ok(_) = process::Command::new("kill")
                .arg("-0")
                .arg(pid_num.to_string())
                .output()
            {
                return true;
            }
        }
    }

    cleanup_pid_file(pid_file);
    false
}
