#[derive(Copy, Clone, PartialEq)]
pub enum Kind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub kind: Kind,
    pub rotation: u8,
    pub x: i32,
    pub y: i32,
}

impl Piece {
    pub fn new(kind: Kind, x: i32, y: i32) -> Self {
        Self {
            kind,
            rotation: 0,
            x,
            y,
        }
    }

    pub fn blocks(&self) -> [(i32, i32); 4] {
        let blocks = match self.kind {
            Kind::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
            Kind::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            Kind::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
            Kind::S => [(0, 0), (1, 0), (-1, 1), (0, 1)],
            Kind::Z => [(-1, 0), (0, 0), (0, 1), (1, 1)],
            Kind::J => [(-1, 0), (-1, 1), (0, 1), (1, 1)],
            Kind::L => [(1, 0), (-1, 1), (0, 1), (1, 1)],
        };

        if matches!(self.kind, Kind::O) {
            return blocks;
        }

        blocks.map(|(x, y)| match self.rotation % 4 {
            0 => (x, y),
            1 => (-y, x),
            2 => (-x, -y),
            3 => (y, -x),
            _ => unreachable!(),
        })
    }

    pub fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn position(&self) -> [(i32, i32); 4] {
        self.blocks().map(|(x, y)| (self.x + x, self.y + y))
    }
}
