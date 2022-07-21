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
        if self.last_update.elapsed().expect("Unexpected time error.") >= self.settings.speed {
            // if all of other minos are locked
            if self.minos.iter().all(|v| v.locked) {
                // generate a mino
                self.minos.extend(Mino::generate(&mut self.rng));
            }

            // lock it or move it
            let mut lock = false;

            for mino in self.minos.iter().filter(|v| !v.locked) {
                let mut new_pos = mino.position;

                new_pos[1] += 1;

                if Tetris::is_outside(new_pos) || self.is_occupied_and_locked(new_pos) {
                    lock = true;
                    break;
                }
            }

            if lock {
                for mino in self.minos.iter_mut() {
                    mino.locked = true;
                }
            } else {
                for mino in self.minos.iter_mut().filter(|v| !v.locked) {
                    mino.position[1] += 1;
                }
            }

            self.last_update = SystemTime::now();
        }
    }

    pub fn update_scale(&mut self, canvas: &WindowCanvas) -> crate::R {
        self.settings.window_size = canvas.output_size()?;
        self.layout.borrow_mut().update(self);

        Ok(())
    }

    pub fn advance(&mut self, direction: MinoDirection) {
        let mut mino_to_move = vec![];
        let offset = match direction {
            MinoDirection::Left => -1,
            MinoDirection::Right => 1,
        };

        for (i, mino) in self.minos.iter().enumerate().filter(|&(_, v)| !v.locked) {
            let mut new_pos = mino.position;

            new_pos[0] += offset;

            if Tetris::is_outside(new_pos) || self.is_occupied_and_locked(new_pos) {
                mino_to_move.clear();
                break;
            }

            mino_to_move.push(i);
        }

        for i in mino_to_move.into_iter() {
            self.minos[i].position[0] += offset;
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

    pub fn is_outside(position: [i32; 2]) -> bool {
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

        match mino_types[rng.gen_range(0..6)] {
            MinoType::O => [
                ([4, -1], MinoType::O),
                ([5, -1], MinoType::O),
                ([4, 0], MinoType::O),
                ([5, 0], MinoType::O),
            ]
            .into_iter()
            .map(|v| v.into())
            .collect(),

            MinoType::I => [
                ([3, -1], MinoType::I),
                ([4, -1], MinoType::I),
                ([5, -1], MinoType::I),
                ([6, -1], MinoType::I),
            ]
            .into_iter()
            .map(|v| v.into())
            .collect(),

            MinoType::S => [
                ([4, -1], MinoType::S),
                ([4, 0], MinoType::S),
                ([5, 0], MinoType::S),
                ([5, 1], MinoType::S),
            ]
            .into_iter()
            .map(|v| v.into())
            .collect(),

            MinoType::Z => [
                ([5, -1], MinoType::Z),
                ([5, 0], MinoType::Z),
                ([4, 0], MinoType::Z),
                ([4, 1], MinoType::Z),
            ]
            .into_iter()
            .map(|v| v.into())
            .collect(),

            MinoType::J => [
                ([4, -1], MinoType::J),
                ([5, -1], MinoType::J),
                ([6, -1], MinoType::J),
                ([6, 0], MinoType::J),
            ]
            .into_iter()
            .map(|v| v.into())
            .collect(),

            MinoType::L => [
                ([4, 0], MinoType::L),
                ([4, -1], MinoType::L),
                ([5, -1], MinoType::L),
                ([6, -1], MinoType::L),
            ]
            .into_iter()
            .map(|v| v.into())
            .collect(),
        }
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
