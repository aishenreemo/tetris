extern crate sdl2;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use super::Settings;
use super::Tetris;
use super::R;

pub trait Layout {
	fn render(&self, game: &Tetris, canvas: &mut WindowCanvas) -> R;
	fn update_scale(&mut self, game: &Tetris);
}

pub struct TetrisLayout {
	main: Rect,
}

impl TetrisLayout {
	pub fn new(settings: &Settings) -> Self {
		let (window_width, window_height) = settings.window_size;

		let pwidth = |percentage: f32| window_width as f32 * percentage;
		let pheight = |percentage: f32| window_height as f32 * percentage;

		let game_width = clamp(pwidth(0.5), 275.0, 400.0);
		let game_height = clamp(pheight(0.7), 400.0, 500.0);
		let game_center = (pwidth(0.375) as i32, pheight(0.5) as i32);
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

	fn update_scale(&mut self, game: &Tetris) {
		let (window_width, window_height) = game.cfg.window_size;
		let pwidth = |percentage: f32| window_width as f32 * percentage;
		let pheight = |percentage: f32| window_height as f32 * percentage;

		self.main.set_width(clamp(pwidth(0.5), 200.0, 300.0) as u32);
		self.main.set_height(clamp(pheight(0.7), 400.0, 575.0) as u32);
		self.main.center_on((pwidth(0.3) as i32, pheight(0.5) as i32));
	}
}

fn clamp<T: PartialOrd + std::fmt::Debug>(input: T, min: T, max: T) -> T {
	match (input, min, max) {
		(_, b, c) if b >= c => panic!("min must be less than max."),
		(a, b, _) if a < b => b,
		(a, _, c) if a > c => c,
		(a, _, _) => a,
	}
}
