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
    line_number: Option<usize>,
}

impl DiffLine {
    fn new(content: &str, kind: DiffKind, line_number: Option<usize>) -> Self {
        let content = content.to_string();
        Self {
            content,
            kind,
            line_number,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn kind(&self) -> &DiffKind {
        &self.kind
    }

    pub fn line_number(&self) -> &Option<usize> {
        &self.line_number
    }
}

#[derive(Debug)]
pub enum DiffKind {
    Addition,
    Removal,
    Neutral,
    Blank,
}

impl DiffKind {
    pub fn value(&self) -> &str {
        match self {
            DiffKind::Addition => "+",
            DiffKind::Removal => "-",
            DiffKind::Neutral => " ",
            DiffKind::Blank => " ",
        }
    }
}

/// Performs 'git diff <filename>' and returns the result as a string
pub fn get_raw_diff(filename: &str) -> String {
    // Process git diff <filename> command and save the stdout response
    let output = Command::new("git")
        .args(["diff", "-U1000", filename])
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
    let mut diff_one_line = 1;
    let mut diff_two_line = 1;

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
                diff.diff_two.push(DiffLine::new(
                    content,
                    DiffKind::Addition,
                    Some(diff_two_line),
                ));
                diff_two_line += 1;
                if removals > 0 {
                    removals -= 1
                } else {
                    additions += 1
                }
            }

            '-' => {
                diff.diff_one.push(DiffLine::new(
                    content,
                    DiffKind::Removal,
                    Some(diff_one_line),
                ));
                diff_one_line += 1;
                removals += 1
            }
            _ => {
                for _ in 0..removals {
                    diff.diff_two.push(DiffLine::new("", DiffKind::Blank, None))
                }

                removals = 0;

                for _ in 0..additions {
                    diff.diff_one.push(DiffLine::new("", DiffKind::Blank, None))
                }

                additions = 0;

                diff.diff_one.push(DiffLine::new(
                    content,
                    DiffKind::Neutral,
                    Some(diff_one_line),
                ));
                diff_one_line += 1;
                diff.diff_two.push(DiffLine::new(
                    content,
                    DiffKind::Neutral,
                    Some(diff_two_line),
                ));
                diff_two_line += 1
            }
        }
    }

    for _ in 0..removals {
        diff.diff_two.push(DiffLine::new("", DiffKind::Blank, None))
    }

    for _ in 0..additions {
        diff.diff_one.push(DiffLine::new("", DiffKind::Blank, None))
    }

    println!("{:?}", diff);
    diff
}

fn remove_first_char(string: &str) -> (char, &str) {
    let mut chars = string.chars();
    (chars.next().unwrap_or(' '), chars.as_str())
}
