use anyhow::Result;
use clap::Parser;
use diff_tool::{
    cli::Arguments,
    git::{get_raw_diff, parse_diff},
    model::{Model, RunningState},
    tui,
    update::{event::handle_event, update},
    view::{self, body::parse_diff_rows},
};

fn main() -> Result<()> {
    env_logger::init();
    let args = Arguments::parse();
    let path = args.path();

    let mut terminal = tui::init_terminal()?;

    let diff_string = get_raw_diff(path, args.change_dir());
    let mut model = Model::new(&diff_string);

    let old_diff_rows = parse_diff_rows(model.old_diff());
    let current_diff_rows = parse_diff_rows(model.current_diff());

    // Will exit when RunningState is not 'Done'
    while *model.running_state() != RunningState::Done {
        // Render ui
        terminal.draw(|rect| view::view(&mut model, rect, &old_diff_rows, &current_diff_rows))?;

        let mut current_msg = handle_event(&model)?;

        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    tui::restore_terminal()?;

    Ok(())
}
