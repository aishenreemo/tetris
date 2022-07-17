extern crate sdl2;
use sdl2::keyboard::Keycode;

use std::collections::HashMap;
use std::time::SystemTime;
use std::time::Duration;

pub fn init() -> Messenger {
    Messenger::new()
}

pub fn has_elapsed(
    timestamp: &SystemTime,
    ms: u64,
) -> bool {
    timestamp.elapsed().expect("Unexpected time error.") > Duration::from_millis(ms)
}

pub enum Command {
    Quit,
}

pub struct Messenger {
    pub on_hold: HashMap<Keycode, SystemTime>,
    commands: Vec<Command>,
}

impl Messenger {
    fn new() -> Self {
        Self {
            commands: [].into(),
            on_hold: [].into(),
        }
    }

    pub fn send(
        &mut self,
        cmd: Command,
    ) {
        self.commands.push(cmd);
    }

    pub fn receive(&mut self) -> Option<Command> {
        self.commands.pop()
    }

    pub fn key_hold(
        &mut self,
        keycode: Keycode,
    ) {
        self.on_hold.entry(keycode).or_insert_with(SystemTime::now);
    }

    pub fn key_release(
        &mut self,
        keycode: Keycode,
    ) {
        self.on_hold.remove(&keycode);
    }
}
