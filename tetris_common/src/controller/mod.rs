use crate::command::TetrisCommand;

use std::collections::VecDeque;

pub struct TetrisController {
    pub commands: VecDeque<TetrisCommand>,
}

impl Default for TetrisController {
    fn default() -> Self {
        TetrisController {
            commands: [].into(),
        }
    }
}
