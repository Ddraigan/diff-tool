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
    let path = args.path();

    let diff = get_raw_diff(path, args.change_dir());

    let app = App::new(get_diff(&diff));

    start_tui(app)?;

    Ok(())
}
