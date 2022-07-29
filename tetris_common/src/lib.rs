extern crate sdl2;
use sdl2::render::WindowCanvas;

pub mod controller;
pub mod command;

pub mod layout;
use layout::TetrisLayout;
use layout::Layout;

use std::cell::RefCell;
use std::time::Duration;

pub type R = Result<(), String>;

pub const COLUMNS: usize = 10;
pub const ROWS: usize = 20;
pub const WINDOW_NAME: &str = "tetris";
pub const WINDOW_DEFAULT_WIDTH: u32 = 400;
pub const WINDOW_DEFAULT_HEIGHT: u32 = 600;
pub const DEFAULT_SPEED: u64 = 200;
pub const GAME_FPS: u64 = 30;

pub struct Tetris {
    pub layout: Box<RefCell<dyn Layout>>,
    pub cfg: Settings,
}

impl Default for Tetris {
    fn default() -> Self {
        let cfg = Settings::default();

        Tetris {
            layout: Box::new(RefCell::new(TetrisLayout::new(&cfg))),
            cfg,
        }
    }
}

impl Tetris {
    pub fn update_scale(&mut self, canvas: &WindowCanvas) -> R {
        self.cfg.window_size = canvas.output_size()?;
        self.layout.borrow_mut().update_scale(self);

        Ok(())
    }
}

pub struct Settings {
    pub window_size: (u32, u32),
    pub speed: Duration,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window_size: (WINDOW_DEFAULT_WIDTH, WINDOW_DEFAULT_HEIGHT),
            speed: Duration::from_millis(DEFAULT_SPEED),
        }
    }
}
