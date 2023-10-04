use std::{
    env,
    process::{Command, Stdio},
};

pub fn get_raw_diff() -> String {
    let args: Vec<String> = env::args().collect();

    // Needs better handling
    let filename = if args.len() > 1 { &args[1] } else { "" };

    // Process git diff <filename> command and save the stdout response
    let output = Command::new("git")
        .args(["diff", filename])
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    // Convert stdout response to a string
    let result = String::from_utf8(output.stdout).unwrap();
    println!("{result}");
    result
}
