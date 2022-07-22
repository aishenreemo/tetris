extern crate sdl2;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

extern crate rand;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::display;
use display::TetrisDisplay;

use crate::message;
use message::MinoDirection;

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
    pub rng: ThreadRng,
}

impl Tetris {
    pub fn update(&mut self) {
        // if all of other minos are locked
        if self.minos.iter().all(|v| v.locked) {
            // generate a mino
            self.minos.extend(Mino::generate(&mut self.rng));
        }

        let is_pos_valid = |m: &Mino| {
            let peek = [m.position[0], m.position[1] + 1];
            (self.is_outside(peek) || self.is_occupied_and_locked(peek)) && !m.locked
        };

        // move it or lock it
        if !self.minos.iter().any(is_pos_valid) {
            self.minos
                .iter_mut()
                .filter(|m| !m.locked)
                .for_each(|m| m.position[1] += 1)
        } else {
            self.minos.iter_mut().for_each(|m| m.locked = true);
        }

        self.last_update = SystemTime::now();
    }

    pub fn update_scale(&mut self, canvas: &WindowCanvas) -> crate::R {
        self.settings.window_size = canvas.output_size()?;
        self.layout.borrow_mut().update(self);

        Ok(())
    }

    pub fn advance(&mut self, direction: MinoDirection) {
        let offset = match direction {
            MinoDirection::Left => -1,
            MinoDirection::Right => 1,
        };

        let is_pos_valid = |m: &Mino| {
            let peek = [m.position[0] + offset, m.position[1]];
            (self.is_outside(peek) || self.is_occupied_and_locked(peek)) && !m.locked
        };

        if !self.minos.iter().any(is_pos_valid) {
            self.minos
                .iter_mut()
                .filter(|m| !m.locked)
                .for_each(|m| m.position[0] += offset)
        }
    }

    pub fn is_occupied(&self, position: [i32; 2]) -> bool {
        self.minos.iter().any(|m| m.position == position)
    }

    pub fn is_occupied_and_locked(&self, position: [i32; 2]) -> bool {
        self.minos
            .iter()
            .any(|m| m.position == position && m.locked)
    }

    pub fn is_outside(&self, position: [i32; 2]) -> bool {
        !(0..10).contains(&position[0]) || !(0..20).contains(&position[1])
    }
}

impl Default for Tetris {
    fn default() -> Self {
        let settings = Settings::default();
        let mut rng = rand::thread_rng();
        Self {
            layout: Box::new(RefCell::new(TetrisDisplay::init(&settings))),
            last_update: SystemTime::now(),
            minos: Mino::generate(&mut rng),
            settings,
            rng,
        }
    }
}

pub struct Mino {
    pub position: [i32; 2],
    pub mino_type: MinoType,
    pub locked: bool,
}

impl Mino {
    pub fn generate(rng: &mut ThreadRng) -> Vec<Mino> {
        let mino_types = [
            MinoType::O,
            MinoType::I,
            MinoType::Z,
            MinoType::S,
            MinoType::J,
            MinoType::L,
        ];

        let mino_type = mino_types[rng.gen_range(0..6)];

        let minos = match mino_types[rng.gen_range(0..6)] {
            MinoType::O => [[4, -1], [5, -1], [4, 0], [5, 0]],
            MinoType::I => [[3, -1], [4, -1], [5, -1], [6, -1]],
            MinoType::S => [[4, -1], [4, 0], [5, 0], [5, 1]],
            MinoType::Z => [[5, -1], [5, 0], [4, 0], [4, 1]],
            MinoType::J => [[4, -1], [5, -1], [6, -1], [6, 0]],
            MinoType::L => [[4, 0], [4, -1], [5, -1], [6, -1]],
        };

        minos.into_iter().map(|v| (v, mino_type).into()).collect()
    }
}

impl From<([i32; 2], MinoType)> for Mino {
    fn from(e: ([i32; 2], MinoType)) -> Self {
        Self {
            position: e.0,
            mino_type: e.1,
            locked: false,
        }
    }
}

#[derive(Clone, Copy)]
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
