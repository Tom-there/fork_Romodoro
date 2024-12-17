use clap::Parser;
use directories::UserDirs;
use native_dialog::MessageDialog;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::{self, Command},
    thread,
    time::Duration,
};

#[derive(Parser)]
struct Cli {
    /// Time in minutes
    time: Option<f32>,
    // Cancel timer flag
    #[arg(long)]
    cancel: bool,
}

fn main() {
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

    if env::var("POMODORO_CHILD").is_err() {
        Command::new(std::env::current_exe().unwrap())
            .args(&[&time.to_string()])
            .env("POMODORO_CHILD", "true")
            .spawn()
            .expect("Failed to start the timer in the background.");

        return;
    }

    run_timer(time);
}

/// Timer Logic
fn run_timer(time: f32) {
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

fn cancel_timer() {
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

fn save_pid(pid: u32) {
    let pid_file = get_pid_file();
    let mut file = File::create(&pid_file).expect("Failed to create PID file.");
    writeln!(file, "{}", pid).expect("Failed to write PID to file.");
}

fn get_pid_file() -> PathBuf {
    let home_dir = UserDirs::new()
        .expect("Failed to get user directories")
        .home_dir()
        .to_path_buf();
    home_dir.join(".romo_pid")
}

fn cleanup_pid_file(pid_file: PathBuf) {
    if pid_file.exists() {
        fs::remove_file(pid_file).expect("Failed to remove PID file.");
    }
}

fn process_running(pid: u32) -> bool {
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
