use sensehat_screen::{PixelColor, PixelFrame, Screen};

use crate::game::{Cell, Game};

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
        let mut rows = [[PixelColor::BLACK; 8]; 8];

        for (y, row) in game.board().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Cell::Filled(_) = cell {
                    rows[y][x] = PixelColor::WHITE;
                }
            }
        }

        for (x, y) in game.piece().position() {
            if (0..8).contains(&x) && (0..8).contains(&y) {
                rows[y as usize][x as usize] = PixelColor::WHITE;
            }
        }

        let frame = PixelFrame::from_rows(&rows);
        self.screen.write_frame(&frame.frame_line());
    }
}
