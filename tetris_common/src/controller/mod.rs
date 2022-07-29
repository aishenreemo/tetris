extern crate sdl2;
use sdl2::keyboard::Keycode;

use crate::command::TetrisCommand;

use std::collections::VecDeque;
use std::collections::HashMap;

pub struct TetrisController {
    pub commands: VecDeque<TetrisCommand>,
    pub onhold: HashMap<Keycode, u32>,
}

impl Default for TetrisController {
    fn default() -> Self {
        TetrisController {
            commands: [].into(),
            onhold: [].into(),
        }
    }
}

impl TetrisController {
    pub fn key_hold(&mut self, keycode: Keycode, timestamp: u32) {
        self.onhold.entry(keycode).or_insert_with(|| timestamp);
    }

    pub fn key_release(&mut self, keycode: Keycode) {
        self.onhold.remove(&keycode);
    }
}
