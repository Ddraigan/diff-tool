use std::cell::RefCell;

use ratatui::widgets::TableState;

#[derive(Debug)]
pub struct DiffState {
    old_diff: RefCell<TableState>,
    current_diff: RefCell<TableState>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

impl Default for DiffState {
    fn default() -> Self {
        Self {
            old_diff: RefCell::from(TableState::default().with_selected(0)),
            current_diff: RefCell::from(TableState::default().with_selected(0)),
        }
    }
}

impl DiffState {
    pub fn old_diff(&self) -> &RefCell<TableState> {
        &self.old_diff
    }

    pub fn current_diff(&self) -> &RefCell<TableState> {
        &self.current_diff
    }

    pub fn reset_row_state(&self) {
        self.old_diff.borrow_mut().select(Some(0));
        self.current_diff.borrow_mut().select(Some(0));
    }
}
