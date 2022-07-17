extern crate sdl2;
use sdl2::ttf::Sdl2TtfContext;

pub fn init(context: &Sdl2TtfContext) -> FontManager {
    FontManager::new(context)
}

#[allow(dead_code)]
pub struct FontManager<'ttf> {
    context: &'ttf Sdl2TtfContext,
}

impl<'ttf> FontManager<'ttf> {
    pub fn new(context: &'ttf Sdl2TtfContext) -> Self {
        Self { context }
    }
}
