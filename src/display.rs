use sensehat_screen::{PixelColor, PixelFrame, Screen};

use crate::game::{Cell, Game, piece::Kind};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

pub struct Display {
    screen: Screen,
}

impl Display {
    pub fn new() -> Self {
        Self {
            screen: Screen::open("/dev/fb0").expect("failed to open Sense HAT display"),
        }
    }

    pub fn render(&mut self, game: &Game) {
        let mut rows = [[PixelColor::BLACK; WIDTH]; HEIGHT];

        for (y, row) in game.board().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Cell::Filled(kind) = cell {
                    rows[y][x] = color(*kind);
                }
            }
        }

        let active_color = color(game.piece().kind).dim(0.5);

        for (x, y) in game.piece().position() {
            if (0..WIDTH as i32).contains(&x) && (0..HEIGHT as i32).contains(&y) {
                rows[y as usize][x as usize] = active_color;
            }
        }

        if game.is_game_over() {
            for row in &mut rows {
                for pixel in row {
                    *pixel = pixel.dim(0.25);
                }
            }

            for x in 0..8 {
                rows[0][x] = PixelColor::RED;
                rows[7][x] = PixelColor::RED;
            }

            for y in 0..8 {
                rows[y][0] = PixelColor::RED;
                rows[y][7] = PixelColor::RED;
            }
        }

        let frame = PixelFrame::from_rows(&rows);
        self.screen.write_frame(&frame.frame_line());
    }
}

fn color(kind: Kind) -> PixelColor {
    match kind {
        Kind::I => PixelColor::CYAN,
        Kind::O => PixelColor::YELLOW,
        Kind::T => PixelColor::MAGENTA,
        Kind::S => PixelColor::GREEN,
        Kind::Z => PixelColor::RED,
        Kind::J => PixelColor::BLUE,
        Kind::L => PixelColor::new(255, 165, 0),
    }
}
