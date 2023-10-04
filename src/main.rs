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
    let args = Arguments::parse();
    let filename = args.filename();

    let diff = get_raw_diff(filename);
    println!("{diff}");

    let app = Rc::new(RefCell::new(App::new(get_diff(&diff))));
    start_tui(app)?;

    Ok(())
}
