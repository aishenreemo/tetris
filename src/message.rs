extern crate sdl2;
use sdl2::keyboard::Keycode;

use std::collections::HashMap;
use std::time::SystemTime;
use std::time::Duration;

pub fn has_elapsed(timestamp: &SystemTime, ms: u64) -> bool {
    timestamp.elapsed().expect("Unexpected time error.") > Duration::from_millis(ms)
}

pub struct Messenger {
    pub commands: Vec<Command>,
    pub onhold: HashMap<Keycode, SystemTime>,
}

impl Default for Messenger {
    fn default() -> Self {
        Self {
            commands: [].into(),
            onhold: [].into(),
        }
    }
}

impl Messenger {
    pub fn key_hold(&mut self, keycode: Keycode) {
        self.onhold.entry(keycode).or_insert_with(SystemTime::now);
    }

    pub fn key_release(&mut self, keycode: Keycode) {
        self.onhold.remove(&keycode);
    }
}

pub enum Command {
    Quit,
    Resize,
    MoveMino(MinoDirection),
}

pub enum MinoDirection {
    Left,
    Right,
}
