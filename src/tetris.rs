extern crate sdl2;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

use crate::display;
use display::TetrisDisplay;

pub trait Draw {
    fn draw(&self, canvas: &mut WindowCanvas) -> crate::R {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        Ok(())
    }

    fn update(&mut self, settings: &Settings);
}

pub struct Tetris {
    pub layout: Box<dyn Draw>,
    pub settings: Settings,
}

impl Tetris {
    pub fn update_scale(&mut self, canvas: &WindowCanvas) -> crate::R {
        self.settings.window_size = canvas.output_size()?;
        self.layout.update(&self.settings);

        Ok(())
    }
}

impl Default for Tetris {
    fn default() -> Self {
        let settings = Settings::new();
        Self {
            layout: Box::new(TetrisDisplay::init(&settings)),
            settings,
        }
    }
}

pub struct Settings {
    pub window_size: (u32, u32),
}

impl Settings {
    pub fn new() -> Self {
        Self {
            window_size: (500, 700),
        }
    }
}
