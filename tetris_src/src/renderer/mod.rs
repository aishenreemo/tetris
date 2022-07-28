extern crate tetris_common;
use tetris_common::Tetris;

extern crate sdl2;
use sdl2::render::WindowCanvas;

use super::R;

pub fn render(game: &Tetris, canvas: &mut WindowCanvas) -> R {
	game.layout.borrow().render(game, canvas)
}
