use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use super::Settings;
use super::Tetris;
use super::R;

pub trait Layout {
	fn render(&self, game: &Tetris, canvas: &mut WindowCanvas) -> R;
}

pub struct TetrisLayout {
	main: Rect,
}

impl TetrisLayout {
	pub fn new(settings: &Settings) -> Self {
		let (window_width, window_height) = settings.window_size;

		let pwidth = |percentage: f32| window_width as f32 * percentage;
		let pheight = |percentage: f32| window_height as f32 * percentage;

		let game_width = pwidth(0.5);
		let game_height = pheight(0.7);
		let game_center = (pwidth(0.3) as i32, pheight(0.4) as i32);
		let game_size = (game_width as u32, game_height as u32);
		let main = Rect::from_center(game_center, game_size.0, game_size.1);

		Self { main }
	}
}

impl Layout for TetrisLayout {
	fn render(&self, _game: &Tetris, canvas: &mut WindowCanvas) -> R {
		canvas.set_draw_color(Color::BLACK);
		canvas.clear();

		canvas.set_draw_color(Color::GREY);
		canvas.draw_rect(self.main)?;

		canvas.present();

		Ok(())
	}
}
