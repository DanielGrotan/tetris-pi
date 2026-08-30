use std::{thread, time::Duration};

mod input;

fn main() {
    let rx = input::spawn();

    loop {
        loop {
            match rx.try_recv() {
                Ok(input) => println!("{input:?}"),
                Err(crossfire::TryRecvError::Empty) => break,
                Err(crossfire::TryRecvError::Disconnected) => return,
            }
        }

        thread::sleep(Duration::from_millis(16));
    }
}
