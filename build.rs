use chrono::{Local, Timelike};
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    // Get current system time as integers
    let now = Local::now();
    let current_hr = now.hour();
    let current_mn = now.minute();
    let current_sec = now.second();

    // Path to the generated file
    let dest_path = format!("{}/time.rs", out_dir);
    let mut f = File::create(dest_path).expect("Could not create file");

    // Write integer constants
    writeln!(f, "pub const CURRENT_HR: u8 = {};", current_hr).unwrap();
    writeln!(f, "pub const CURRENT_MN: u8 = {};", current_mn).unwrap();
    writeln!(f, "pub const CURRENT_SEC: u8 = {};", current_sec).unwrap();

    println!(
        "Generated new compile-time time: {:02}:{:02}:{:02}",
        current_hr, current_mn, current_sec
    );
}
