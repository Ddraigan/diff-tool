use std::process::{Command, Stdio};

#[derive(Default, Debug)]
pub struct Diff {
    diff_one: Vec<DiffLine>,
    diff_two: Vec<DiffLine>,
}

#[derive(Debug)]
struct DiffLine {
    content: String,
    kind: DiffKind,
}

#[derive(Debug)]
enum DiffKind {
    Addition,
    Removal,
    Neutral,
    Blank,
}

impl DiffKind {
    fn value(&self) -> &str {
        match self {
            DiffKind::Addition => "+",
            DiffKind::Removal => "-",
            DiffKind::Neutral => "",
            DiffKind::Blank => "",
        }
    }
}

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

pub fn get_diff(diff_string: &str) -> Diff {
    let lines = diff_string.split("\n");

    let diff = Diff::default();

    let mut start = false;
    let mut additions = 0;
    let mut removals = 0;

    for line in lines {
        if line.starts_with("@@") && line.ends_with("@@") {
            start = true;
            continue;
        }

        if !start {
            continue;
        }

        if line.starts_with("+") {
            remove_first_char(line);
        }
    }
}

fn remove_first_char(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next();
    chars.as_str()
}
