use std::{
    thread,
    time::{Duration, Instant},
};

use crate::game::Game;

mod game;
mod input;

fn main() {
    let rx = input::spawn();

    let mut game = Game::new();
    let mut last_updated = Instant::now();

    loop {
        loop {
            match rx.try_recv() {
                Ok(input) => game.handle_input(input),
                Err(crossfire::TryRecvError::Empty) => break,
                Err(crossfire::TryRecvError::Disconnected) => return,
            }
        }

        let now = Instant::now();
        let dt = now - last_updated;

        game.update(dt);
        last_updated = now;

        thread::sleep(Duration::from_millis(16));
    }
}
