extern crate sdl2;
use sdl2::render::WindowCanvas;

pub fn init() -> Tetris {
    Tetris::new()
}

pub struct Tetris {
    cfg: Settings,
}

impl Tetris {
    fn new() -> Self {
        Self {
            cfg: Settings::new(),
        }
    }

    pub fn size(&self) -> Size {
        self.cfg.window_size
    }

    pub fn update(
        &mut self,
        canvas: &WindowCanvas,
    ) -> Result<(), String> {
        self.cfg.update(canvas)
    }
}

struct Settings {
    window_size: Size,
}

impl Settings {
    fn new() -> Self {
        Self {
            window_size: Size::window_default(),
        }
    }

    pub fn update(
        &mut self,
        canvas: &WindowCanvas,
    ) -> Result<(), String> {
        self.window_size = canvas.output_size()?.into();
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Size {
    width: u32,
    height: u32,
}

impl Size {
    fn window_default() -> Self {
        Self {
            width: 400,
            height: 800,
        }
    }
}

impl From<(u32, u32)> for Size {
    fn from(s: (u32, u32)) -> Self {
        Self {
            width: s.0,
            height: s.1,
        }
    }
}

impl From<Size> for (u32, u32) {
    fn from(s: Size) -> Self {
        (s.width, s.height)
    }
}
