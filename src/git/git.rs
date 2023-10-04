use crate::cli::cli::Arguments;
use clap::Parser;
use std::process::{Command, Stdio};

pub fn get_raw_diff() -> String {
    let args = Arguments::parse();
    let filename = args.filename();

    // Process git diff <filename> command and save the stdout response
    let output = Command::new("git")
        .args(["diff", filename])
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute process");

    // Convert stdout response to a string
    let result = String::from_utf8(output.stdout).expect("UTF8 data to convert to string");
    result
}
