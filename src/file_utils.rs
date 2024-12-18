use directories::UserDirs;
use std::{fs, fs::File, io::Write, path::PathBuf};

/// Generic function to save data to a file
pub fn save_to_file<T: ToString>(file: &PathBuf, data: T) {
    let mut file = File::create(file).expect("Failed to create file.");
    writeln!(file, "{}", data.to_string()).expect("Failed to write to file.");
}

/// Generic function to get a file path in the home directory
pub fn get_file_path(file_name: &str) -> PathBuf {
    let home_dir = UserDirs::new()
        .expect("Failed to get user directories")
        .home_dir()
        .to_path_buf();
    home_dir.join(file_name)
}

/// Generic function to clean up (remove) a file
pub fn cleanup_file(file: &PathBuf) {
    if file.exists() {
        fs::remove_file(file).expect("Failed to remove file.");
    }
}

