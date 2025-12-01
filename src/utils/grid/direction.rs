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
    pub const fn clockwise(&self, shift: usize) -> Self {
        CLOCKWISE[(shift + *self as usize) % CLOCKWISE.len()]
    }

    pub const fn counter_clockwise(&self, shift: usize) -> Self {
        self.clockwise(CLOCKWISE.len() - (shift % CLOCKWISE.len()))
    }

    #[allow(dead_code)]
    pub fn oposite(&self) -> Self {
        CLOCKWISE[((CLOCKWISE.len() / 2) + *self as usize) % CLOCKWISE.len()]
    }

    pub const fn iter(&self) -> DirectionIter<'_> {
        DirectionIter {
            direction: self,
            shift: 0,
        }
    }

    pub fn is_horizontal(&self) -> bool {
        *self == Self::Left || *self == Self::Right
    }

    pub fn is_vertical(&self) -> bool {
        *self == Self::Up || *self == Self::Down
    }
}

pub struct DirectionIter<'a> {
    direction: &'a Direction,
    shift: usize,
}

impl Iterator for DirectionIter<'_> {
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
