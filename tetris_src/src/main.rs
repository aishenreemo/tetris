extern crate tetris_common;
use tetris_common::controller::TetrisController as Controller;

mod listener;
mod renderer;
mod updater;

use std::time::Duration;
use std::thread;

pub type R = Result<(), String>;

const WINDOW_NAME: &str = "tetris";
const WINDOW_DEFAULT_WIDTH: u32 = 800;
const WINDOW_DEFAULT_HEIGHT: u32 = 800;

const GAME_FPS: u64 = 30;

fn main() -> R {
    // initialize stuff
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(WINDOW_NAME, WINDOW_DEFAULT_WIDTH, WINDOW_DEFAULT_HEIGHT)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    // create canvas
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // controller
    let mut controller = Controller::default();

    // event listener
    let mut event_pump = sdl_context.event_pump()?;

    // traditional game loop
    loop {
        // handle events
        listener::listen(&mut event_pump, &mut controller);

        // update game struct
        updater::update(&mut controller)?;

        // render game
        renderer::render(&mut canvas)?;

        // sleep for (1s / FPS)
        thread::sleep(Duration::from_nanos(1_000_000_000 / GAME_FPS));
    }
}
