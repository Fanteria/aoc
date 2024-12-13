const CLOCKWISE: [Direction; 8] = [
    Direction::Up,
    Direction::UpRight,
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left,
    Direction::UpLeft,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn clockwise(&self, shift: usize) -> Self {
        CLOCKWISE[(shift + *self as usize) % CLOCKWISE.len()]
    }

    #[allow(dead_code)]
    pub fn oposite(&self) -> Self {
        CLOCKWISE[((CLOCKWISE.len() / 2) + *self as usize) % CLOCKWISE.len()]
    }

    pub fn iter(&self) -> DirectionIter {
        DirectionIter{ direction: self, shift: 0 }
    }
}

pub struct DirectionIter<'a> {
    direction: &'a Direction,
    shift: usize,
}

impl<'a> Iterator for DirectionIter<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.shift < 8 {
            self.shift += 1;
            Some(self.direction.clockwise(self.shift - 1))
        } else {
            None
        }
    }
}
