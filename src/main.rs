use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use diff_tool::{app::app::App, git::git::get_raw_diff, start_tui};

fn main() -> Result<()> {
    get_raw_diff();
    let app = Rc::new(RefCell::new(App::new()));
    start_tui(app)?;
    Ok(())
}
