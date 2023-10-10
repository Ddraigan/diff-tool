use anyhow::Result;
use clap::Parser;
use diff_tool::{
    app::App,
    cli::Arguments,
    git::{get_diff, get_raw_diff},
    start_tui,
};

fn main() -> Result<()> {
    env_logger::init();
    let args = Arguments::parse();
    let filename = args.filename();

    let diff = get_raw_diff(filename);

    let app = App::new(get_diff(&diff));

    start_tui(app)?;

    Ok(())
}
