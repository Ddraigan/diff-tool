use clap::Parser;
use std::path::{Path, PathBuf};

use crate::services::config::get_config_dir;

#[derive(Parser, Debug)]
#[command(author = "Ddraigan", version = version(), about = "A side by side git diff view")]
pub struct Args {
    #[clap(short = 'C', long)]
    /// Giving a full path allows diff-tool to diff outside of the git repo
    change_dir: bool,
    /// File to diff with
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

    // pub fn tick_rate(&self) -> u64 {
    //     self.tick_rate
    // }
}

pub fn version() -> clap::builder::Str {
    let author = clap::crate_authors!();

    let commit_hash = env!("DIFF_TOOL_GIT_INFO");

    // let current_exe_path = PathBuf::from(clap::crate_name!()).display().to_string();
    let config_dir_path = get_config_dir().unwrap().display().to_string();
    // let data_dir_path = get_data_dir().unwrap().display().to_string();

    let val = format!(
        "\
    {commit_hash}

    Authors: {author}

    Config directory: {config_dir_path}"
    );

    val.into()
}
