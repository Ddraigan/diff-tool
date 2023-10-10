use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author = "Ddraigan", version, about = "A side by side git diff view")]
pub struct Arguments {
    /// File to diff with
    filename: String,
}

impl Arguments {
    pub fn filename(&self) -> &String {
        &self.filename
    }
}
