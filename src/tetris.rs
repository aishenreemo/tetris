extern crate sdl2;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

use crate::display;
use display::TetrisDisplay;

use std::time::SystemTime;
use std::time::Duration;
use std::cell::RefCell;

pub trait Draw {
    fn draw(&self, _game: &Tetris, canvas: &mut WindowCanvas) -> crate::R {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        Ok(())
    }

    fn update(&mut self, game: &Tetris);
}

pub struct Tetris {
    pub layout: Box<RefCell<dyn Draw>>,
    pub last_update: SystemTime,
    pub settings: Settings,
    pub minos: Vec<Mino>,
}

pub struct Mino {
    pub position: [i32; 2],
    pub mino_type: MinoType,
}

impl From<([i32; 2], MinoType)> for Mino {
    fn from(e: ([i32; 2], MinoType)) -> Self {
        Self {
            position: e.0,
            mino_type: e.1,
        }
    }
}

#[allow(dead_code)]
pub enum MinoType {
    O,
    S,
    Z,
    I,
    J,
    L,
}

impl Default for MinoType {
    fn default() -> Self {
        Self::O
    }
}

impl Tetris {
    pub fn update(&mut self) {
        if self.last_update.elapsed().expect("Unexpected time error.") >= self.settings.speed {
            for mino in self.minos.iter_mut() {
                mino.position[1] += 1;
            }

            self.last_update = SystemTime::now();
        }
    }

    pub fn update_scale(&mut self, canvas: &WindowCanvas) -> crate::R {
        self.settings.window_size = canvas.output_size()?;
        self.layout.borrow_mut().update(self);

        Ok(())
    }

    pub fn is_occupied(&self, position: [i32; 2]) -> bool {
        self.minos.iter().any(|m| m.position == position)
    }
}

impl Default for Tetris {
    fn default() -> Self {
        let settings = Settings::default();
        Self {
            layout: Box::new(RefCell::new(TetrisDisplay::init(&settings))),
            last_update: SystemTime::now(),
            settings,
            minos: [
                ([4, 0], MinoType::O),
                ([5, 0], MinoType::O),
                ([4, 1], MinoType::O),
                ([5, 1], MinoType::O),
            ]
            .into_iter()
            .map(|v| v.into())
            .collect(),
        }
    }
}

pub struct Settings {
    pub window_size: (u32, u32),
    pub speed: Duration,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window_size: (500, 700),
            speed: Duration::from_millis(250),
        }
    }
}
