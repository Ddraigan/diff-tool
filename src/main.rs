use anyhow::Result;
use clap::Parser;
use diff_tool::{
    app::app::App,
    cli::cli::Arguments,
    git::git::{get_diff, get_raw_diff},
    start_tui,
};
use std::{cell::RefCell, rc::Rc};

fn main() -> Result<()> {
    env_logger::init();
    let args = Arguments::parse();
    let filename = args.filename();

    let diff = get_raw_diff(filename);

    let mut app = App::new();
    app.set_diff(get_diff(&diff));

    let app = Rc::new(RefCell::new(app));

    start_tui(app)?;

    Ok(())
}
