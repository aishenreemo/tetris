extern crate sdl2;
use sdl2::render::WindowCanvas;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;

mod core;
use crate::core as tetris_core;
use tetris_core::Tetris;

mod ttf;
use ttf::FontManager;

pub mod message;
use message::Command;
use message::Messenger;

use std::time::Duration;
use std::thread;
use std::error;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().expect("Could not initialize ttf.");

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("tetris", 500, 700)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem.");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas.");

    let mut game = tetris_core::init();
    let mut messenger = message::init();
    let mut fonts = ttf::init(&ttf_context);

    // traditional game loop
    let mut event_pump = sdl_context.event_pump()?;

    loop {
        // process input
        for event in event_pump.poll_iter() {
            // listen to input events
            listen(&mut messenger, event);
        }

        // update game data/info
        if let Err(err) = update(&mut messenger, &canvas, &mut game) {
            eprintln!("Encountered error while updating game:\n{err:?}");
        }

        // render display based on the info
        if let Err(err) = render(&mut canvas, &mut fonts, &game) {
            eprintln!("Encountered error while rendering canvas:\n{err:?}");
        }

        // 30 fps
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}

fn listen(
    messenger: &mut Messenger,
    event: Event,
) {
    match event {
        Event::Quit { timestamp: _ } => messenger.send(Command::Quit),
        Event::KeyDown {
            keycode,
            timestamp: _,
            window_id: _,
            scancode: _,
            keymod: _,
            repeat: _,
        } => messenger.key_hold(keycode.unwrap()),
        Event::KeyUp {
            keycode,
            timestamp: _,
            window_id: _,
            scancode: _,
            keymod: _,
            repeat: _,
        } => messenger.key_release(keycode.unwrap()),
        _ => (),
    }

    let mut commands = vec![];
    for (key, timestamp) in messenger.on_hold.iter() {
        match key {
            // if you're holding escape for 1 seconds
            Keycode::Escape if message::has_elapsed(timestamp, 1000) => {
                commands.push(Command::Quit)
            },
            _ => (),
        }
    }

    commands.into_iter().for_each(|cmd| messenger.send(cmd))
}

fn update(
    messenger: &mut Messenger,
    canvas: &WindowCanvas,
    game: &mut Tetris,
) -> Result<(), String> {
    while let Some(cmd) = messenger.receive() {
        match cmd {
            Command::Quit => std::process::exit(0),
        }
    }

    game.update(canvas)
}

fn render(
    canvas: &mut WindowCanvas,
    _font_mgr: &mut FontManager,
    game: &Tetris,
) -> Result<(), Box<dyn error::Error>> {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    canvas.set_draw_color(Color::GREY);
    let (window_width, window_height): (u32, u32) = game.size().into();

    let pwidth = |percentage: f32| window_width as f32 * percentage;
    let pheight = |percentage: f32| window_height as f32 * percentage;

    let game_width = pwidth(0.5);
    let game_height = pheight(0.7);
    let game_center = (pwidth(0.3) as i32, pheight(0.4) as i32);
    let game_size = (game_width as u32, game_height as u32);
    let game_rect = Rect::from_center(game_center, game_size.0, game_size.1);

    let columns = 10;
    let rows = 20;

    let cell_width = game_width / columns as f32;
    let cell_height = game_height / rows as f32;
    let cell_center = (cell_width / 2.0, cell_height / 2.0);

    canvas.draw_rect(game_rect)?;

    for i in 0..(columns * rows) {
        let column = i % columns;
        let row = i / columns;

        let x = game_rect.x() + (column as f32 * cell_width) as i32 + cell_center.0 as i32;
        let y = game_rect.y() + (row as f32 * cell_height) as i32 + cell_center.1 as i32;

        let width = (cell_width * 0.9) as u32;
        let height = (cell_height * 0.9) as u32;

        let cell_rect = Rect::from_center((x, y), width, height);

        canvas.draw_rect(cell_rect)?;
    }

    canvas.present();

    Ok(())
}
