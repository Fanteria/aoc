use super::{Direction, Grid, Path};

/// Can construct invalid `Point`
pub fn new_point(x: usize, y: usize) -> Point {
    Point { x, y }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    #[allow(dead_code)]
    pub fn new<T>(x: usize, y: usize, grid: &Grid<T>) -> Option<Self> {
        if grid.height > x && grid.width > y {
            Some(Self { x, y })
        } else {
            None
        }
    }

    pub fn can<T>(&self, direction: Direction, grid: &Grid<T>) -> bool {
        use Direction::*;
        match direction {
            Up => self.x != 0,
            UpRight => self.x != 0 && grid.width > self.y + 1,
            Right => grid.width > self.y + 1,
            DownRight => grid.width > self.y + 1 && grid.height > self.x + 1,
            Down => grid.height > self.x + 1,
            DownLeft => grid.height > self.x + 1 && self.y != 0,
            Left => self.y != 0,
            UpLeft => self.y != 0 && self.x != 0,
        }
    }

    pub fn adjacent<T>(&self, direction: Direction, grid: &Grid<T>) -> Option<Self> {
        if self.can(direction, grid) {
            use Direction::*;
            Some(match direction {
                Up => Self {
                    x: self.x - 1,
                    y: self.y,
                },
                UpRight => Self {
                    x: self.x - 1,
                    y: self.y + 1,
                },
                Right => Self {
                    x: self.x,
                    y: self.y + 1,
                },
                DownRight => Self {
                    x: self.x + 1,
                    y: self.y + 1,
                },
                Down => Self {
                    x: self.x + 1,
                    y: self.y,
                },
                DownLeft => Self {
                    x: self.x + 1,
                    y: self.y - 1,
                },
                Left => Self {
                    x: self.x,
                    y: self.y - 1,
                },
                UpLeft => Self {
                    x: self.x - 1,
                    y: self.y - 1,
                },
            })
        } else {
            None
        }
    }

    pub fn move_along_path<T>(&self, path: &Path, grid: &Grid<T>) -> Option<Self> {
        let x = if path.x < 0 {
            self.x.checked_sub((-path.x) as usize)?
        } else {
            self.x.checked_add(path.x as usize)?
        };
        let y = if path.y < 0 {
            self.y.checked_sub((-path.y) as usize)?
        } else {
            self.y.checked_add(path.y as usize)?
        };
        Self::new(x, y, grid)
    }

    pub const fn coordinaes(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn find<T>(grid: &Grid<T>, to_find: &T) -> Option<Self>
    where
        T: Eq,
    {
        grid.data
            .iter()
            .position(|item| *item == *to_find)
            .map(|index| Self {
                x: index / grid.width,
                y: index % grid.width,
            })
    }

    #[inline]
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
