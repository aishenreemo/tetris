extern crate sdl2;
use sdl2::render::WindowCanvas;
use sdl2::keyboard::Keycode;
use sdl2::event::WindowEvent;
use sdl2::event::Event;

mod tetris;
use tetris::Tetris;

pub mod message;
use message::has_elapsed;
use message::Messenger;
use message::Command;

pub mod display;

use std::time::Duration;
use std::thread;

type R = Result<(), String>;

fn main() -> R {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut game = Tetris::default();
    let (width, height) = game.settings.window_size;

    let window = video_subsystem
        .window("tetris", width, height)
        .position_centered()
        .build()
        .expect("Couldn't initialize window.");

    let mut messenger = Messenger::default();
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Couldn't initialize canvas.");

    let mut event_pump = sdl_context.event_pump()?;

    // traditional game loop
    loop {
        for event in event_pump.poll_iter() {
            listen(&mut messenger, event);
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
        Event::KeyDown { keycode, .. } => messenger.key_hold(keycode.unwrap()),
        Event::KeyUp { keycode, .. } => messenger.key_release(keycode.unwrap()),
        // if you resize the window
        Event::Window {
            win_event: WindowEvent::Resized(_, _) | WindowEvent::SizeChanged(_, _),
            ..
        } => messenger.commands.push(Command::Resize),
        _ => (),
    }

    // if you hold a key(e.g Escape key) more than the given milliseconds
    for (key, timestamp) in messenger.onhold.iter() {
        match key {
            Keycode::Escape if has_elapsed(timestamp, 500) => {
                messenger.commands.push(Command::Quit)
            },
            _ => (),
        }
    }
}

fn update(messenger: &mut Messenger, game: &mut Tetris, canvas: &WindowCanvas) -> R {
    while let Some(cmd) = messenger.commands.pop() {
        match cmd {
            Command::Quit => std::process::exit(0),
            Command::Resize => game.update_scale(canvas)?,
        }
    }

    game.update();

    Ok(())
}

fn render(canvas: &mut WindowCanvas, game: &Tetris) -> R {
    game.layout.borrow().draw(game, canvas)
}
