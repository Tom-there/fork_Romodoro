use std::process::Command;

/// Checks if a process is running based on its PID
pub fn process_running(pid: u32) -> bool {
    if cfg!(target_os = "windows") {
        Command::new("tasklist")
            .args(&["/FI", &format!("PID eq {}", pid)])
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).contains(&pid.to_string()))
            .unwrap_or(false)
    } else {
        Command::new("ps")
            .arg("-p")
            .arg(pid.to_string())
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

/// Cancels a running timer process
pub fn cancel_timer() {
    use crate::pid::{cleanup_pid_file, get_pid_file};
    use std::fs;

    let pid_file = get_pid_file();

    if let Ok(pid_str) = fs::read_to_string(&pid_file) {
        if let Ok(pid) = pid_str.trim().parse::<u32>() {
            if process_running(pid) {
                if cfg!(target_os = "windows") {
                    Command::new("taskkill")
                        .args(&["/PID", &pid.to_string(), "/F"])
                        .status()
                        .expect("Failed to stop the timer process.");
                } else {
                    Command::new("kill")
                        .arg(pid.to_string())
                        .status()
                        .expect("Failed to stop the timer process.");
                }
            }
            cleanup_pid_file(pid_file);
            println!("Timer stopped.");
        } else {
            eprintln!("Invalid PID found in the PID file.");
        }
    } else {
        eprintln!("No running timer found.");
    }
}

