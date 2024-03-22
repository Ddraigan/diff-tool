use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use clap::Parser;
use diff_tool::{
    model::{state::RunningState, App},
    services::{cli::Arguments, git::get_raw_diff, logger::VecWriter, terminal},
    view,
};

fn main() -> Result<()> {
    let logs = Arc::new(Mutex::new(Vec::new()));
    let writer = VecWriter { logs: logs.clone() };

    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(Box::new(writer)))
        .filter(None, log::LevelFilter::Trace)
        .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
        .init();
    terminal::install_panic_hook();

    let mut model = App::default(logs);

    let args = Arguments::parse();

    let mut terminal = terminal::init_terminal()?;

    let diff_string = get_raw_diff(args.path(), args.change_dir());
    model.set_diff(&diff_string);

    // Will exit when RunningState is 'Done'
    let mut previous_log_length = model.console().len();
    while *model.running_state() != RunningState::Done {
        // Update console on new log
        let current_log_length = model.console().len();
        if previous_log_length != current_log_length {
            previous_log_length = current_log_length;
            model.handle_console();
        }

        // Render ui
        terminal.draw(|rect| view::view(&mut model, rect))?;

        let mut current_msg = model.handle_event()?;

        while current_msg.is_some() {
            current_msg = model.update(current_msg.unwrap());
        }
    }

    terminal::restore_terminal()?;

    Ok(())
}
