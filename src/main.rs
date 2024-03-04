use anyhow::Result;
use clap::Parser;
use diff_tool::{
    model::{Model, RunningState},
    services::{cli::Arguments, git::get_raw_diff, terminal},
    update::{event::handle_event, message::update},
    view,
};

fn main() -> Result<()> {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
    terminal::install_panic_hook();

    let args = Arguments::parse();
    let path = args.path();

    let mut terminal = terminal::init_terminal()?;

    let mut model = Model::default();
    let diff_string = get_raw_diff(path, args.change_dir());
    model.set_diff(&diff_string);

    // Will exit when RunningState is 'Done'
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
