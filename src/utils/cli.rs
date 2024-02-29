use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author = "Ddraigan", version, about = "A side by side git diff view")]
pub struct Arguments {
    #[clap(short = 'C', long)]
    /// Giving a full path allows diff-tool to diff outside of the git repo
    change_dir: bool,
    /// File to diff with
    path: String,
    #[clap(short, long, default_value_t = 250)]
    tick_rate: u64,
}

impl Arguments {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn change_dir(&self) -> bool {
        self.change_dir
    }

    pub fn tick_rate(&self) -> u64 {
        self.tick_rate
    }
}
