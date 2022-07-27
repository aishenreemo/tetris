use crate::message;
use message::Rotation;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub struct Mino {
    pub locked: bool,
}

pub struct Shape {
    pub variant: ShapeVariant,
    pub mino_pos: [[usize; 2]; 4],
}
// ShapeVariant::S => [[4, 0], [4, 1], [5, 1], [5, 2]],
// ShapeVariant::Z => [[5, 0], [5, 1], [4, 1], [4, 2]],
// ShapeVariant::O => [[4, 0], [5, 0], [4, 1], [5, 1]],
// ShapeVariant::I => [[3, 0], [4, 0], [5, 0], [6, 0]],
// ShapeVariant::L => [[4, 1], [4, 0], [5, 0], [6, 0]],
// ShapeVariant::J => [[4, 0], [5, 0], [6, 0], [6, 1]],

impl Shape {
    pub fn rotate(&self, rot: Rotation) -> [[i32; 2]; 4] {
        let origin = self.variant.find_origin(rot);

        let mut mino_pos = [[0; 2]; 4];
        for (i, &[x, y]) in self.mino_pos.iter().enumerate() {
            mino_pos[i] = [x as i32, y as i32];
        }

        // calculate relative position
        let mut relative_pos = [[0; 2]; 4];

        let o = self.mino_pos[origin];

        let [orig_x, orig_y] = o;
        let [orig_x, orig_y] = [orig_x as i32, orig_y as i32];

        let [x_offset, y_offset] = [0 - orig_x, 0 - orig_y];

        for (i, &[x, y]) in mino_pos.iter().enumerate() {
            relative_pos[i] = [x + x_offset, y + y_offset];
        }

        for pos in relative_pos.iter_mut() {
            *pos = [pos[1], pos[0]];

            match rot {
                Rotation::Clockwise => pos[1] = -pos[1],
                Rotation::CounterClockwise => pos[0] = -pos[0],
            }
        }

        // transform relative into absolute
        let mut absolute_pos = [[0; 2]; 4];
        for (i, &[rel_x, rel_y]) in relative_pos.iter().enumerate() {
            absolute_pos[i] = [orig_x + rel_x, orig_y + rel_y]
        }

        absolute_pos
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ShapeVariant {
    /// o
    /// o
    /// o
    /// o
    I,

    /// oo
    /// oo
    O,

    /// o
    /// oo
    ///  o
    S,

    ///  o
    /// oo
    /// o
    Z,

    ///  o
    ///  o
    /// oo
    J,

    /// o
    /// o
    /// oo
    L,
}

impl ShapeVariant {
    fn find_origin(&self, rot: Rotation) -> usize {
        match self {
            // o o o o
            //   ^
            ShapeVariant::I if rot == Rotation::Clockwise => 1,
            // o o o o
            //     ^
            ShapeVariant::I => 2,
            // > o o
            //   o o
            ShapeVariant::O => 0,
            // o
            // o o <
            //   o
            ShapeVariant::S => 2,
            //     o
            // > o o
            //   o
            ShapeVariant::Z => 2,
            //     o
            // o o o
            //   ^
            ShapeVariant::L => 2,
            // o
            // o o o
            //   ^
            ShapeVariant::J => 1,
        }
    }
}
