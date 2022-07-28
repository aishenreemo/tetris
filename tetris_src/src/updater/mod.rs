extern crate tetris_common;
use tetris_common::controller::TetrisController as Controller;
use tetris_common::command::TetrisCommand as Command;

use super::R;

pub fn update(controller: &mut Controller) -> R {
	while let Some(cmd) = controller.commands.pop_front() {
		match cmd {
			Command::Quit { timestamp } => quit(timestamp),
		}
	}

	Ok(())
}

fn quit(timestamp: u32) {
	println!("Exited the game after {timestamp}");
	std::process::exit(0);
}
