use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser, Default, Debug)]
#[command(author = "Ddraigan", version, about = "A side by side git diff view")]
pub struct Args {
    #[clap(short = 'C', long)]
    /// Giving a full path allows diff-tool to diff outside of the git repo
    change_dir: bool,
    /// File to diff with
    path: PathBuf,
    // #[clap(short, long, default_value_t = 250)]
    // tick_rate: u64,
    // TODO: Add tick rate arg
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
