extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::EventPump;

extern crate tetris_common;
use tetris_common::controller::TetrisController as Controller;
use tetris_common::command::TetrisCommand as Command;

pub fn listen(event_pump: &mut EventPump, controller: &mut Controller) {
    for event in event_pump.poll_iter() {
        handle_event(event, controller)
    }
}

fn handle_event(event: Event, controller: &mut Controller) {
    match event {
        // if you force close the window
        Event::Quit { timestamp } => {
            controller.commands.push_back(Command::Quit { timestamp });
        },

        // if you hold/press a key
        Event::KeyDown {
            timestamp,
            keycode: Some(keycode),
            ..
        } => {
            handle_keyhold(controller, keycode, timestamp);
        },

        _ => (),
    }
}

#[allow(clippy::single_match)]
fn handle_keyhold(controller: &mut Controller, keycode: Keycode, timestamp: u32) {
    match keycode {
        Keycode::Escape => {
            controller.commands.push_back(Command::Quit { timestamp });
        },
        _ => (),
    }
}
