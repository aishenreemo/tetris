#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Mino {
    pub locked: bool,
}

pub struct Shape {
    pub variant: ShapeVariant,
    pub mino_pos: [[usize; 2]; 4],
}

#[derive(Copy, Clone)]
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
