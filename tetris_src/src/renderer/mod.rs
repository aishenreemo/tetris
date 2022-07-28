extern crate sdl2;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

use super::R;

pub fn render(canvas: &mut WindowCanvas) -> R {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    Ok(())
}
