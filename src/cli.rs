use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author = "Ddraigan", version, about = "A side by side git diff view")]
pub struct Arguments {
    #[clap(short = 'C', long, action)]
    /// Giving a full path allows diff-tool to diff outside of the git repo
    change_dir: bool,
    /// File to diff with
    path: String,
}

impl Arguments {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn change_dir(&self) -> bool {
        self.change_dir
    }
}
