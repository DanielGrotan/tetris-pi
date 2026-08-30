use std::thread;

use crossfire::spsc;

use joystick::{Action, Direction, Joystick};

mod joystick;

#[derive(Debug)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
    Press,
}

pub fn spawn() -> crossfire::Rx<spsc::Array<Input>> {
    let (tx, rx) = spsc::bounded_blocking(16);

    thread::spawn(move || {
        let mut joystick = Joystick::open().expect("failed to open joystick");

        loop {
            let events = match joystick.events() {
                Ok(events) => events,
                Err(error) => {
                    eprintln!("joystick error: {error}");
                    break;
                }
            };

            for event in events {
                if event.action != Action::Press {
                    continue;
                }

                let input = match event.direction {
                    Direction::Up => Input::Up,
                    Direction::Down => Input::Down,
                    Direction::Left => Input::Left,
                    Direction::Right => Input::Right,
                    Direction::Enter => Input::Press,
                };

                if tx.send(input).is_err() {
                    return;
                }
            }
        }
    });

    rx
}
