use std::time::Duration;

use piece::Piece;

use crate::{game::piece::Kind, input::Input};

mod piece;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Filled(Kind),
}

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

pub struct Game {
    board: [[Cell; 8]; 8],
    piece: Piece,
    gravity_timer: Duration,
    gravity_interval: Duration,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [[Cell::Empty; WIDTH]; HEIGHT],
            piece: Piece::new(Kind::T, 4, 0),
            gravity_timer: Duration::ZERO,
            gravity_interval: Duration::from_millis(500),
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.gravity_timer += dt;

        while self.gravity_timer >= self.gravity_interval {
            self.gravity_timer -= self.gravity_interval;

            if !self.move_piece(0, 1) {
                self.lock_piece();
                self.clear_lines();
                self.spawn_piece();
            }
        }
    }

    pub fn handle_input(&mut self, input: Input) {
        match input {
            Input::Up => {
                self.rotate_piece();
            }
            Input::Down => {
                self.move_piece(0, 1);
            }
            Input::Left => {
                self.move_piece(-1, 0);
            }
            Input::Right => {
                self.move_piece(0, 1);
            }
            Input::Press => {
                self.hard_drop();
            }
        }
    }

    pub fn board(&self) -> &[[Cell; WIDTH]; HEIGHT] {
        &self.board
    }

    pub fn piece(&self) -> &Piece {
        &self.piece
    }

    fn move_piece(&mut self, dx: i32, dy: i32) -> bool {
        let mut piece = self.piece;
        piece.x += dx;
        piece.y += dy;

        if self.can_place(&piece) {
            self.piece = piece;
            true
        } else {
            false
        }
    }

    fn rotate_piece(&mut self) {
        let mut piece = self.piece;
        piece.rotate();

        if self.can_place(&piece) {
            self.piece = piece
        }
    }

    fn hard_drop(&mut self) {
        while self.move_piece(0, 1) {}

        self.lock_piece();
        self.clear_lines();
        self.spawn_piece();
    }

    fn can_place(&self, piece: &Piece) -> bool {
        for (x, y) in piece.position() {
            if x < 0 || x >= WIDTH as i32 || y < 0 || y >= HEIGHT as i32 {
                return false;
            }

            if self.board[y as usize][x as usize] != Cell::Empty {
                return false;
            }
        }

        true
    }

    fn lock_piece(&mut self) {
        for (x, y) in self.piece.position() {
            self.board[y as usize][x as usize] = Cell::Filled(self.piece.kind);
        }
    }

    fn clear_lines(&mut self) {
        let mut write_row = HEIGHT;

        for read_row in (0..HEIGHT).rev() {
            let full = self.board[read_row].iter().all(|cell| *cell != Cell::Empty);

            if !full {
                write_row -= 1;
                self.board[write_row] = self.board[read_row]
            }
        }

        for row in 0..write_row {
            self.board[row] = [Cell::Empty; WIDTH];
        }
    }

    fn spawn_piece(&mut self) {
        self.piece = Piece::new(Kind::T, 4, 0);
    }
}
