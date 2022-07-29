extern crate tetris_common;
use tetris_common::Tetris;
use tetris_common::R;

extern crate sdl2;
use sdl2::render::WindowCanvas;

pub fn render(game: &Tetris, canvas: &mut WindowCanvas) -> R {
	game.layout.borrow().render(game, canvas)
}
