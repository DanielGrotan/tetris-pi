use std::io;

use evdev::{EventSummary, KeyCode, raw_stream::RawDevice};

const DEVICE_NAME: &str = "Raspberry Pi Sense HAT Joystick";

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Enter,
}

#[derive(PartialEq)]
pub enum Action {
    Press,
    Release,
    Hold,
}

pub struct Event {
    pub direction: Direction,
    pub action: Action,
}

pub struct Joystick {
    device: RawDevice,
}

impl Joystick {
    pub fn open() -> io::Result<Self> {
        let (_, device) = evdev::raw_stream::enumerate()
            .find(|(_, device)| device.name() == Some(DEVICE_NAME))
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "Sense HAT joystick not found")
            })?;

        Ok(Self { device })
    }

    pub fn events(&mut self) -> io::Result<Vec<Event>> {
        let events = self
            .device
            .fetch_events()?
            .filter_map(|event| {
                let EventSummary::Key(_, key, value) = event.destructure() else {
                    return None;
                };

                let direction = match key {
                    KeyCode::KEY_UP => Direction::Up,
                    KeyCode::KEY_DOWN => Direction::Down,
                    KeyCode::KEY_LEFT => Direction::Left,
                    KeyCode::KEY_RIGHT => Direction::Right,
                    KeyCode::KEY_ENTER => Direction::Enter,
                    _ => return None,
                };

                let action = match value {
                    0 => Action::Release,
                    1 => Action::Press,
                    2 => Action::Hold,
                    _ => return None,
                };

                Some(Event { direction, action })
            })
            .collect::<Vec<_>>();

        Ok(events)
    }
}
