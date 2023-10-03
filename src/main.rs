use std::{cell::RefCell, rc::Rc};

use diff_tool::{app::app::App, start_tui};
use eyre::Result;

fn main() -> Result<()> {
    let app = Rc::new(RefCell::new(App::new()));
    start_tui(app)?;
    Ok(())
}
