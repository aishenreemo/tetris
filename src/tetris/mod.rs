extern crate sdl2;
use sdl2::render::WindowCanvas;

extern crate rand;
use rand::rngs::ThreadRng;
use rand::Rng;

pub mod mino;
use mino::ShapeVariant;
use mino::Shape;
use mino::Mino;

pub mod cfg;
use cfg::Settings;

use crate::display;
use display::TetrisDisplay;
use display::Draw;

use std::time::SystemTime;
use std::cell::RefCell;

pub struct Tetris {
    pub minos: [[Option<Mino>; 10]; 20],
    pub layout: Box<RefCell<dyn Draw>>,
    pub focused_shape: Option<Shape>,
    pub last_update: SystemTime,
    pub rng: ThreadRng,
    pub cfg: Settings,
}

impl Default for Tetris {
    fn default() -> Self {
        let cfg = Settings::default();

        Self {
            focused_shape: None,
            last_update: SystemTime::now(),
            layout: Box::new(RefCell::new(TetrisDisplay::init(&cfg))),
            minos: [[None; 10]; 20],
            rng: rand::thread_rng(),
            cfg,
        }
    }
}

impl Tetris {
    pub fn update_scale(&mut self, canvas: &WindowCanvas) -> crate::R {
        self.cfg.window_size = canvas.output_size()?;
        self.layout.borrow_mut().update(self);

        Ok(())
    }

    pub fn update(&mut self) {
        // reset timer
        self.last_update = SystemTime::now();

        // generate a shape if there is none
        if self.focused_shape.is_none() {
            self.generate();
            return; // skip for next iteration
        }

        // is locked
        let is_locked = |column: usize, row: usize| -> bool {
            self.minos[row][column].map(|m| m.locked).unwrap_or(false)
        };

        // collision checking
        let is_colliding_down = |&[column, row]: &[usize; 2]| -> bool {
            row + 1 >= 20 || !is_locked(column, row) && is_locked(column, row + 1)
        };

        if self
            .focused_shape
            .as_ref()
            .unwrap()
            .mino_pos
            .iter()
            .any(is_colliding_down)
        {
            self.lock();
        } else {
            self.advance();
        }
    }

    fn generate(&mut self) {
        let mino_types = [
            ShapeVariant::O,
            ShapeVariant::I,
            ShapeVariant::Z,
            ShapeVariant::S,
            ShapeVariant::J,
            ShapeVariant::L,
        ];

        // pick a random shape
        let variant = mino_types[self.rng.gen_range(0..6)];

        // initial positions
        let mino_pos = match variant {
            ShapeVariant::S => [[4, 0], [4, 1], [5, 1], [5, 2]],
            ShapeVariant::Z => [[5, 0], [5, 1], [4, 1], [4, 2]],
            ShapeVariant::O => [[4, 0], [5, 0], [4, 1], [5, 1]],
            ShapeVariant::I => [[3, 0], [4, 0], [5, 0], [6, 0]],
            ShapeVariant::L => [[4, 1], [4, 0], [5, 0], [6, 0]],
            ShapeVariant::J => [[4, 0], [5, 0], [6, 0], [6, 1]],
        };

        // fill each position
        for &[column, row] in mino_pos.iter() {
            self.minos[row][column] = Some(Mino { locked: false });
        }

        // focus the formed shape
        self.focused_shape = Some(Shape { variant, mino_pos });
    }

    fn lock(&mut self) {
        // pin focused_shape to this position
        // consumes self.focused_shape
        let m = self.focused_shape.take().unwrap();

        for &[column, row] in m.mino_pos.iter() {
            let mino = &mut self.minos[row][column];

            mino.as_mut().unwrap().locked = true;
        }
    }

    fn advance(&mut self) {
        // move focused_shape 1 block down
        let m = self.focused_shape.as_mut().unwrap();

        for &[column, row] in m.mino_pos.iter() {
            self.minos[row][column] = None;
        }

        for pos in m.mino_pos.iter_mut() {
            pos[1] += 1;
            self.minos[pos[1]][pos[0]] = Some(Mino { locked: false });
        }
    }
}
