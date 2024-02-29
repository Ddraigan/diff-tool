use anyhow::Result;
use clap::Parser;
use diff_tool::{
    input::event::handle_event,
    model::{Model, RunningState},
    services::{
        git::get_raw_diff,
        terminal::{self},
    },
    update::update,
    utils::cli::Arguments,
    view,
};

fn main() -> Result<()> {
    env_logger::init();
    terminal::install_panic_hook();

    let args = Arguments::parse();
    let path = args.path();

    let mut terminal = terminal::init_terminal()?;

    let diff_string = get_raw_diff(path, args.change_dir());
    let mut model = Model::new(&diff_string);

    // Will exit when RunningState is not 'Done'
    while *model.running_state() != RunningState::Done {
        // Render ui
        terminal.draw(|rect| view::view(&mut model, rect))?;

        let mut current_msg = handle_event(&model)?;

        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    terminal::restore_terminal()?;

    Ok(())
}
