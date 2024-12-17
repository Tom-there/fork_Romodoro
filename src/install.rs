use std::{env, fs, path::PathBuf};

pub fn ensure_global_install() {
    let exe_path = env::current_exe().expect("Failed to determine executable path.");

    #[cfg(target_os = "windows")]
    {
        let global_dir = PathBuf::from("C:\\Program Files\\Romo");
        let global_exe_path = global_dir.join("romo.exe");

        if !global_exe_path.exists() {
            println!("Installing Romo globally...");
            fs::create_dir_all(&global_dir).expect("Failed to create global installation directory.");
            fs::copy(&exe_path, &global_exe_path).expect("Failed to copy Romo to global directory.");
            println!("Romo installed successfully!");
        }
    }

    #[cfg(target_os = "linux")]
    {
        let local_bin_dir = PathBuf::from(env::var("HOME").unwrap()).join(".local/bin");
        let global_exe_path = local_bin_dir.join("romo");

        if !global_exe_path.exists() {
            println!("Installing Romo globally...");
            fs::create_dir_all(&local_bin_dir).expect("Failed to create ~/.local/bin directory.");
            fs::copy(&exe_path, &global_exe_path).expect("Failed to copy Romo to ~/.local/bin.");
            println!("Romo installed successfully!");
        }
    }
}

