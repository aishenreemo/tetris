extern crate tetris_common;
use tetris_common::controller::TetrisController as Controller;
use tetris_common::command::TetrisCommand as Command;
use tetris_common::Tetris;
use tetris_common::R;

extern crate sdl2;
use sdl2::render::WindowCanvas;

pub fn update(game: &mut Tetris, controller: &mut Controller, canvas: &WindowCanvas) -> R {
	while let Some(cmd) = controller.commands.pop_front() {
		match cmd {
			Command::Quit { timestamp } => quit(timestamp),
			Command::UpdateScale => game.update_scale(canvas)?,
		}
	}

	Ok(())
}

fn quit(timestamp: u32) {
	println!("Exited the game after {}s", timestamp / 1000);
	std::process::exit(0);
}
