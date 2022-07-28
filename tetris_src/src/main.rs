extern crate tetris_common;
use tetris_common::controller::TetrisController as Controller;
use tetris_common::Tetris;
use tetris_common::WINDOW_NAME;
use tetris_common::GAME_FPS;

mod listener;
mod renderer;
mod updater;

use std::time::Duration;
use std::thread;

pub type R = Result<(), String>;

fn main() -> R {
	// initialize stuff
	let sdl_context = sdl2::init()?;
	let video_subsystem = sdl_context.video()?;

	let game = Tetris::default();

	let window = video_subsystem
		.window(WINDOW_NAME, game.cfg.window_size.0, game.cfg.window_size.1)
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
		renderer::render(&game, &mut canvas)?;

		// sleep for (1s / FPS)
		thread::sleep(Duration::from_nanos(1_000_000_000 / GAME_FPS));
	}
}
