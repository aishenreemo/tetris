extern crate sdl2;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::tetris;
use tetris::cfg::Settings;
use tetris::Tetris;

use crate::R;

const COLUMNS: usize = 10;
const ROWS: usize = 20;

pub trait Draw {
    fn draw(&self, game: &Tetris, canvas: &mut WindowCanvas) -> R;
    fn update(&mut self, game: &Tetris);
}

pub struct TetrisDisplay {
    main: Rect,
    cells: [[Rect; COLUMNS]; ROWS],
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

        let cell_width = game_width / COLUMNS as f32;
        let cell_height = game_height / ROWS as f32;
        let cell_center = (cell_width / 2.0, cell_height / 2.0);

        let mut cells = [[Rect::new(0, 0, 1, 1); COLUMNS]; ROWS];

        for i in 0..(COLUMNS * ROWS) {
            let column = i % COLUMNS;
            let row = i / COLUMNS;

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
    fn draw(&self, game: &Tetris, canvas: &mut WindowCanvas) -> R {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.set_draw_color(Color::GREY);
        canvas.draw_rect(self.main)?;

        for i in 0..COLUMNS * ROWS {
            let column = i % COLUMNS;
            let row = i / COLUMNS;

            let cell = self.cells[row][column];

            if game.minos[row][column].is_none() {
                canvas.set_draw_color(Color::GRAY);
                canvas.draw_rect(cell)?;
            } else {
                canvas.set_draw_color(Color::BLUE);
                canvas.fill_rect(cell)?;
            }
        }

        canvas.present();
        Ok(())
    }

    fn update(&mut self, game: &Tetris) {
        let (window_width, window_height) = game.cfg.window_size;

        let pwidth = |percentage: f32| window_width as f32 * percentage;
        let pheight = |percentage: f32| window_height as f32 * percentage;

        self.main.set_width(pwidth(0.5) as u32);
        self.main.set_height(pwidth(0.7) as u32);
        self.main
            .center_on((pwidth(0.3) as i32, pheight(0.4) as i32));

        let cell_width = self.main.width() as f32 / 10.0;
        let cell_height = self.main.height() as f32 / 20.0;
        let cell_center = (cell_width / 2.0, cell_height / 2.0);

        for i in 0..(COLUMNS * ROWS) {
            let column = i % COLUMNS;
            let row = i / COLUMNS;

            let cell = &mut self.cells[row][column];

            cell.set_width((cell_width * 0.9) as u32);
            cell.set_height((cell_height * 0.9) as u32);

            cell.center_on((
                self.main.x() + (column as f32 * cell_width) as i32 + cell_center.0 as i32,
                self.main.y() + (row as f32 * cell_height) as i32 + cell_center.1 as i32,
            ));
        }
    }
}
