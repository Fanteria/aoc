use std::ops::Sub;

use super::{Direction, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Path {
    pub x: i64,
    pub y: i64,
}

impl Path {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const fn from_points(from: &Point, to: &Point) -> Self {
        let (to_x, to_y) = to.coordinaes();
        let (from_x, from_y) = from.coordinaes();
        Self {
            x: to_x as i64 - from_x as i64,
            y: to_y as i64 - from_y as i64,
        }
    }

    pub const fn to_direction(&self) -> Option<Direction> {
        Some(match (self.x, self.y) {
            (x, y) if x < 0 && y == 0 => Direction::Up,
            (x, y) if x > 0 && y == 0 => Direction::Down,
            (x, y) if x == 0 && y > 0 => Direction::Right,
            (x, y) if x == 0 && y < 0 => Direction::Left,
            (x, y) if x < 0 && y > 0 => Direction::UpRight,
            (x, y) if x > 0 && y > 0 => Direction::DownRight,
            (x, y) if x > 0 && y < 0 => Direction::DownLeft,
            (x, y) if x < 0 && y < 0 => Direction::UpLeft,
            _ => return None,
        })
    }
}

impl From<Direction> for Path {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Self { x: -1, y: 0 },
            Direction::UpRight => Self { x: -1, y: 1 },
            Direction::Right => Self { x: 0, y: 1 },
            Direction::DownRight => Self { x: 1, y: 1 },
            Direction::Down => Self { x: 1, y: 0 },
            Direction::DownLeft => Self { x: 1, y: -1 },
            Direction::Left => Self { x: 0, y: -1 },
            Direction::UpLeft => Self { x: -1, y: -1 },
        }
    }
}

impl Sub for Path {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
