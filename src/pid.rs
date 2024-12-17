use directories::UserDirs;
use std::{fs::{self, File}, io::Write, path::PathBuf};

/// Saves the process ID to a PID file
pub fn save_pid(pid: u32) {
    let pid_file = get_pid_file();
    let mut file = File::create(&pid_file).expect("Failed to create PID file.");
    writeln!(file, "{}", pid).expect("Failed to write PID to file.");
}

/// Gets the PID file location
pub fn get_pid_file() -> PathBuf {
    let home_dir = UserDirs::new()
        .expect("Failed to get user directories")
        .home_dir()
        .to_path_buf();
    home_dir.join(".romo_pid")
}

/// Removes the PID file
pub fn cleanup_pid_file(pid_file: PathBuf) {
    if pid_file.exists() {
        fs::remove_file(pid_file).expect("Failed to remove PID file.");
    }
}

