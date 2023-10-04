use std::process::{Command, Stdio};

/// Performs 'git diff <filename>' and returns the result as a string
pub fn get_raw_diff(filename: &str) -> String {
    // Process git diff <filename> command and save the stdout response
    let output = Command::new("git")
        .args(["diff", filename])
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute process");

    // Convert stdout response to a string and return
    String::from_utf8(output.stdout).expect("UTF8 data to convert to string")
}
