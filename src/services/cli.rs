use clap::Parser;
use std::path::{Path, PathBuf};

use crate::services::config::{get_config_dir, get_data_dir};

#[derive(Parser, Debug)]
#[command(author = "Ddraigan", version = version(), about = "A side by side git diff view")]
pub struct Args {
    #[arg(short = 'C', long)]
    /// Giving a full path allows diff-tool to diff outside of the git repo
    change_dir: bool,
    /// File to diff with
    #[arg(value_name = "FILE")]
    path: PathBuf,
    // #[clap(short, long, default_value_t = 250)]
    // tick_rate: u64,
    // TODO: Implement tick rate arg
}

impl Args {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn change_dir(&self) -> bool {
        self.change_dir
    }
}

pub fn version() -> clap::builder::Str {
    let author = clap::crate_authors!();

    let commit_hash = option_env!("DIFF_TOOL_GIT_INFO").unwrap_or("unknown");

    let config_dir_path = get_config_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "Unavailable".into());

    let data_dir_path = get_data_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "Unavailable".into());

    format!(
        "\n
    Commit: {commit_hash}

    Authors: {author}

    Config directory: {config_dir_path}
    Data directory: {data_dir_path}\n"
    )
    .into()
}
