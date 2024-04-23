use anyhow::Result;
use clap::Parser;
use diff_tool::{
    app::{state::RunningState, App},
    services::{
        cli::Args,
        git::get_raw_diff,
        logger::{init_logging, initialize_logging, VecWriter},
        terminal,
    },
    view,
};
use std::sync::{Arc, Mutex};

fn main() -> Result<()> {
    let args = Args::parse();

    // Set up logging that can be sent to the application console
    let logs = Arc::new(Mutex::new(Vec::new()));
    let writer = VecWriter::new(logs.clone());
    // init_logging(writer, log::LevelFilter::Trace);
    initialize_logging();

    let mut app = App::new(logs);

    let diff_string = get_raw_diff(args.path(), args.change_dir());
    app.set_diff(&diff_string);

    if app.diff().is_none() {
        // Exit programme gracefully when no diff is found
        println!("No diff found, exiting");
        return Ok(());
    }

    terminal::install_panic_hook();
    let mut terminal = terminal::init_terminal()?;

    let mut previous_log_length = app.console().len();
    // Will exit when RunningState is 'Done'
    while *app.running_state() != RunningState::Done {
        // Update console on new log
        let current_log_length = app.console().len();
        if previous_log_length != current_log_length {
            previous_log_length = current_log_length;
            app.handle_console();
        }

        // Render ui
        terminal.draw(|rect| view::view(&mut app, rect))?;

        let mut current_msg = app.handle_event()?;

        while let Some(msg) = current_msg {
            current_msg = app.update(msg);
        }
    }

    terminal::restore_terminal()?;

    Ok(())
}
