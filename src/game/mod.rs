use std::time::Duration;

use piece::Piece;
use rand::seq::SliceRandom;

use crate::{game::piece::Kind, input::Input};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

pub mod piece;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Filled(Kind),
}

pub struct Game {
    board: [[Cell; WIDTH]; HEIGHT],
    piece: Piece,
    bag: Vec<Kind>,
    gravity_timer: Duration,
    gravity_interval: Duration,
    game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            board: [[Cell::Empty; WIDTH]; HEIGHT],
            piece: Piece::new(Kind::T, 4, 0),
            bag: Vec::new(),
            gravity_timer: Duration::ZERO,
            gravity_interval: Duration::from_secs(1),
            game_over: false,
        };

        game.spawn_piece();

        game
    }

    pub fn update(&mut self, dt: Duration) {
        if self.game_over {
            return;
        }

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
        if self.game_over {
            return;
        }

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
                self.move_piece(1, 0);
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

    pub fn is_game_over(&self) -> bool {
        self.game_over
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
        let kind = self.next_kind();
        let piece = Piece::new(kind, 4, 0);

        if !self.can_place(&piece) {
            self.game_over = true;
            return;
        }

        self.piece = piece;
    }

    fn next_kind(&mut self) -> Kind {
        if self.bag.is_empty() {
            self.bag = vec![
                Kind::I,
                Kind::O,
                Kind::T,
                Kind::S,
                Kind::Z,
                Kind::J,
                Kind::L,
            ];

            let mut rng = rand::rng();
            self.bag.shuffle(&mut rng);
        }

        self.bag.pop().unwrap()
    }
}
