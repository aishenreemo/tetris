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
}

pub struct Tetris {
    pub layout: LayoutManager,
    pub settings: Settings,
}

impl Default for Tetris {
    fn default() -> Self {
        let settings = Settings::new();
        Self {
            layout: LayoutManager::new(&settings),
            settings,
        }
    }
}

pub struct LayoutManager(pub Box<dyn Draw>);

impl LayoutManager {
    pub fn new(settings: &Settings) -> Self {
        Self(Box::new(TetrisDisplay::init(settings)))
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
