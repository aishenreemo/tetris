extern crate sdl2;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::tetris;
use tetris::Draw;
use tetris::Settings;

use crate::R;

pub struct TetrisDisplay {
    main: Rect,
    cells: [[Rect; 10]; 20],
}

impl TetrisDisplay {
    pub fn init(settings: &Settings) -> Self {
        let (window_width, window_height) = settings.window_size;

        let pwidth = |percentage: f32| window_width as f32 * percentage;
        let pheight = |percentage: f32| window_height as f32 * percentage;

        let game_width = pwidth(0.5);
        let game_height = pheight(0.7);
        let game_center = (pwidth(0.3) as i32, pheight(0.4) as i32);
        let game_size = (game_width as u32, game_height as u32);
        let main = Rect::from_center(game_center, game_size.0, game_size.1);

        let columns = 10;
        let rows = 20;

        let mut cells = [[Rect::new(0, 0, 1, 1); 10]; 20];
        let cell_width = game_width / columns as f32;
        let cell_height = game_height / rows as f32;
        let cell_center = (cell_width / 2.0, cell_height / 2.0);

        for i in 0..(columns * rows) {
            let column = i % columns;
            let row = i / columns;

            let cell = &mut cells[row][column];

            let x = main.x() + (column as f32 * cell_width) as i32 + cell_center.0 as i32;
            let y = main.y() + (row as f32 * cell_height) as i32 + cell_center.1 as i32;

            cell.set_width((cell_width * 0.9) as u32);
            cell.set_height((cell_height * 0.9) as u32);
            cell.center_on((x, y));
        }

        Self { main, cells }
    }
}

impl Draw for TetrisDisplay {
    fn draw(&self, canvas: &mut WindowCanvas) -> R {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.set_draw_color(Color::GREY);
        canvas.draw_rect(self.main)?;

        for cell_row in self.cells.into_iter() {
            for cell in cell_row.into_iter() {
                canvas.draw_rect(cell)?;
            }
        }

        canvas.present();
        Ok(())
    }
}
