use std::process::{Command, Stdio};

#[derive(Default, Debug)]
pub struct Diff {
    diff_one: Vec<DiffLine>,
    diff_two: Vec<DiffLine>,
}

impl Diff {
    pub fn diff_one(&self) -> &Vec<DiffLine> {
        &self.diff_one
    }

    pub fn diff_two(&self) -> &Vec<DiffLine> {
        &self.diff_two
    }
}

#[derive(Debug)]
pub struct DiffLine {
    content: String,
    kind: DiffKind,
}

impl DiffLine {
    fn new(content: &str, kind: DiffKind) -> Self {
        let content = content.to_string();
        Self { content, kind }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

#[derive(Debug)]
enum DiffKind {
    Addition,
    Removal,
    Neutral,
    Blank,
}

impl DiffKind {
    fn value(&self) -> char {
        match self {
            DiffKind::Addition => '+',
            DiffKind::Removal => '-',
            DiffKind::Neutral => ' ',
            DiffKind::Blank => ' ',
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

    let mut diff = Diff::default();

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

        let (prefix, content) = remove_first_char(line);

        match prefix {
            '+' => {
                diff.diff_two
                    .push(DiffLine::new(content, DiffKind::Addition));
                if removals > 0 {
                    removals -= 1
                } else {
                    additions += 1
                }
            }

            '-' => {
                diff.diff_one
                    .push(DiffLine::new(content, DiffKind::Removal));
                removals += 1
            }
            _ => {
                for _ in 0..removals {
                    diff.diff_two.push(DiffLine::new("", DiffKind::Blank))
                }

                removals = 0;

                for _ in 0..additions {
                    diff.diff_one.push(DiffLine::new("", DiffKind::Blank))
                }

                additions = 0;

                diff.diff_one
                    .push(DiffLine::new(content, DiffKind::Neutral));
                diff.diff_two
                    .push(DiffLine::new(content, DiffKind::Neutral))
            }
        }
    }

    for _ in 0..removals {
        diff.diff_two.push(DiffLine::new("", DiffKind::Blank))
    }

    for _ in 0..additions {
        diff.diff_one.push(DiffLine::new("", DiffKind::Blank))
    }

    println!("{:?}", diff);
    diff
}

fn remove_first_char(string: &str) -> (char, &str) {
    let mut chars = string.chars();
    (chars.next().unwrap_or(' '), chars.as_str())
}
