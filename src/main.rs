extern crate sdl2;
use sdl2::render::WindowCanvas;
use sdl2::keyboard::Keycode;
use sdl2::event::WindowEvent;
use sdl2::event::Event;

mod tetris;
use tetris::Tetris;

pub mod message;
use message::MinoDirection;
use message::has_elapsed;
use message::Messenger;
use message::Rotation;
use message::Command;

pub mod display;

use std::time::SystemTime;
use std::time::Duration;
use std::thread;

type R = Result<(), String>;

fn main() -> R {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut game = Tetris::default();
    let (width, height) = game.cfg.window_size;

    let window = video_subsystem
        .window("tetris", width, height)
        .position_centered()
        .build()
        .expect("Couldn't initialize window.");

    // handles input
    let mut messenger = Messenger::default();

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Couldn't initialize canvas.");

    let mut event_pump = sdl_context.event_pump()?;

    // traditional game loop
    loop {
        // convert input to a command
        for event in event_pump.poll_iter() {
            listen(&mut messenger, event);
        }

        for (key, timestamp) in messenger.onhold.iter_mut() {
            hold_key(&mut messenger.commands, key, timestamp);
        }

        // update game data/info
        if let Err(err) = update(&mut messenger, &mut game, &canvas) {
            eprintln!("Encountered error while updating game:\n{err:?}");
        }

        // render display based on the info
        if let Err(err) = render(&mut canvas, &game) {
            eprintln!("Encountered error while rendering canvas:\n{err:?}");
        }

        // 30 fps
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    // unreachable expression
    // Ok(())
}

fn listen(messenger: &mut Messenger, event: Event) {
    match event {
        // Ctrl + C in the terminal
        Event::Quit { timestamp: _ } => messenger.commands.push(Command::Quit),
        // releasing a key
        Event::KeyUp { keycode, .. } => messenger.key_release(keycode.unwrap()),
        // holding a key
        Event::KeyDown { keycode, .. } => {
            press_key(messenger, &keycode.unwrap());
            messenger.key_hold(keycode.unwrap());
        },

        // if you resize the window
        Event::Window {
            win_event: WindowEvent::Resized(_, _) | WindowEvent::SizeChanged(_, _),
            ..
        } => messenger.commands.push(Command::Resize),

        _ => (),
    }
}

fn update(messenger: &mut Messenger, game: &mut Tetris, canvas: &WindowCanvas) -> R {
    let mut stop = false;

    while let Some(cmd) = messenger.commands.pop() {
        match cmd {
            // exit the game
            Command::Quit => std::process::exit(0),
            // update scale ui of the game
            Command::Resize => game.update_scale(canvas)?,
            // go left or right
            Command::MoveMino(d) => game.request_turn(d, &mut stop),
            // rotate clockwise or counterclockwise
            Command::Rotate(r) => game.request_rotate(r, &mut stop),
        }
    }

    if game.last_update.elapsed().expect("Unexpected time error.") >= game.cfg.speed && !stop {
        game.update();
    }

    Ok(())
}

fn render(canvas: &mut WindowCanvas, game: &Tetris) -> R {
    game.layout.borrow().draw(game, canvas)
}

fn press_key(m: &mut Messenger, keycode: &Keycode) {
    if m.onhold.contains_key(keycode) {
        return;
    }

    match keycode {
        Keycode::Left => {
            m.commands.push(Command::MoveMino(MinoDirection::Left));
        },
        Keycode::Right => {
            m.commands.push(Command::MoveMino(MinoDirection::Right));
        },
        Keycode::Q => {
            m.commands.push(Command::Rotate(Rotation::CounterClockwise));
        },
        Keycode::E => {
            m.commands.push(Command::Rotate(Rotation::Clockwise));
        },
        _ => (),
    }
}

fn hold_key(commands: &mut Vec<Command>, keycode: &Keycode, timestamp: &mut SystemTime) {
    // if you hold a key(e.g Escape key) more than the given milliseconds
    match keycode {
        Keycode::Escape if has_elapsed(timestamp, 500) => {
            commands.push(Command::Quit);
        },
        Keycode::Left if has_elapsed(timestamp, 200) => {
            commands.push(Command::MoveMino(MinoDirection::Left));
            *timestamp = SystemTime::now();
        },
        Keycode::Right if has_elapsed(timestamp, 200) => {
            commands.push(Command::MoveMino(MinoDirection::Right));
            *timestamp = SystemTime::now();
        },
        _ => (),
    }
}
