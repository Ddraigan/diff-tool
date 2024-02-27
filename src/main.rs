use anyhow::Result;
use clap::Parser;
use diff_tool::{
    cli::Arguments,
    git::{get_diff, get_raw_diff},
    model::{Model, RunningState},
    tui, view,
};

fn main() -> Result<()> {
    env_logger::init();
    let args = Arguments::parse();
    let path = args.path();

    let diff_string = get_raw_diff(path, args.change_dir());
    let diff = get_diff(&diff_string);

    let mut terminal = tui::init_terminal()?;
    let mut model = Model::new(diff);

    // Will exit when RunningState is not 'Done'
    while *model.running_state() != RunningState::Done {
        // Render ui
        terminal.draw(|rect| view::view(&mut model, rect))?;
    }

    tui::restore_terminal()?;

    Ok(())
}
