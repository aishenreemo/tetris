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

use crate::message;
use message::MinoDirection;
use message::Rotation;

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

        let rows_to_clear: Vec<usize> = (0..20)
            .filter(|&r| (0..10).all(|c| is_locked(c, r)))
            .collect();

        if !rows_to_clear.is_empty() {
            rows_to_clear.into_iter().for_each(|i| self.clear(i));
        } else if self
            .focused_shape
            .as_ref() // Option<Shape> -> Option<&Shape>
            .unwrap() // Option<&Shape> -> &Shape
            .mino_pos // [[usize; 2]; 4]
            .iter()
            .any(is_colliding_down)
        {
            self.lock();
        } else {
            self.advance();
        }
    }

    pub fn request_turn(&mut self, dir: MinoDirection, stop: &mut bool) {
        if self.focused_shape.is_none() {
            return; // skip
        }

        let offset = match dir {
            MinoDirection::Left => -1,
            MinoDirection::Right => 1,
        };

        let is_locked = |c: i32, r: i32| -> bool {
            let (c, r) = (c as usize, r as usize);
            self.minos[r][c].map(|m| m.locked).unwrap_or(false)
        };

        let is_colliding = |[c, r]: [i32; 2]| -> bool {
            !(0..10).contains(&(c + offset)) || !is_locked(c, r) && is_locked(c + offset, r)
        };

        if !self
            .focused_shape
            .as_ref()
            .unwrap()
            .mino_pos
            .iter()
            .map(|&p| [p[0] as i32, p[1] as i32]) // [usize; 2] -> [i32; 2]
            .any(is_colliding)
        {
            // prevent from updating right after this iteration
            *stop = true;
            // turn the focused tetromino
            self.turn(offset);
        }
    }

    pub fn request_rotate(&mut self, rot: Rotation, stop: &mut bool) {
        if self.focused_shape.is_none() {
            return; // skip
        }

        if self.focused_shape.as_ref().unwrap().variant == ShapeVariant::O {
            return;
        }

        let new_pos = self.focused_shape.as_ref().unwrap().rotate(rot);

        // is locked
        let is_locked = |c: i32, r: i32| -> bool {
            let (c, r) = (c as usize, r as usize);
            self.minos[r][c].map(|m| m.locked).unwrap_or(false)
        };

        // collision checking
        let is_colliding = |(i, [c, r]): (usize, [i32; 2])| -> bool {
            let [nc, nr] = new_pos[i];

            (!(0..10).contains(&nc) || !(0..20).contains(&nr))
                || (!is_locked(c, r) && is_locked(nc, nr))
        };

        if !self
            .focused_shape
            .as_ref()
            .unwrap()
            .mino_pos
            .iter()
            .map(|&p| [p[0] as i32, p[1] as i32]) // [usize; 2] -> [i32; 2]
            .enumerate()
            .any(is_colliding)
        {
            *stop = true;
            self.rotate(new_pos);
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

    fn turn(&mut self, offset: i32) {
        let m = self.focused_shape.as_mut().unwrap();

        for &[column, row] in m.mino_pos.iter() {
            self.minos[row][column] = None;
        }

        for pos in m.mino_pos.iter_mut() {
            pos[0] = (pos[0] as i32 + offset) as usize;
            self.minos[pos[1]][pos[0]] = Some(Mino { locked: false });
        }
    }

    fn rotate(&mut self, new_pos: [[i32; 2]; 4]) {
        let m = self.focused_shape.as_mut().unwrap();

        for &[column, row] in m.mino_pos.iter() {
            self.minos[row][column] = None;
        }

        for (i, pos) in m.mino_pos.iter_mut().enumerate() {
            pos[0] = new_pos[i][0] as usize;
            pos[1] = new_pos[i][1] as usize;
            self.minos[pos[1]][pos[0]] = Some(Mino { locked: false });
        }
    }

    fn clear(&mut self, row_index: usize) {
        for mino in self.minos[row_index].iter_mut() {
            *mino = None;
        }

        for row in (0..row_index).into_iter().rev() {
            self.minos[row + 1] = self.minos[row];
        }

        for mino in self.minos[0].iter_mut() {
            *mino = None;
        }
    }
}
