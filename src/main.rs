use std::ffi::c_float;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    time: c_float
}

fn main() {
    let args = Cli::parse();
    
    let duration_seconds = (args.time * 60.0).round() as i32;
    

    for remaining in (0..duration_seconds).rev() {
        let minutes = remaining / 60;
        let seconds = remaining % 60;

        println!("\rTime Left: {:02}:{:02}", minutes, seconds);
        std::io::stdout().flush().unwrap();

        sleep(Duration::from_secs(1));
    }

    println!("\nTime's Up! Take a break!");
}
