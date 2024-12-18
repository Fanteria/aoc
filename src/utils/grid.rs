mod direction;
mod path;
mod point;

pub use direction::Direction;
pub use path::Path;
pub use point::Point;

use point::new_point;
use std::{fmt::Display, str::FromStr};

#[derive(Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from_dimensions(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            height,
            width,
            data: vec![Default::default(); height * width],
        }
    }

    pub fn new(data: Vec<T>, width: usize) -> Self {
        Self {
            height: data.len() / width,
            width,
            data,
        }
    }

    pub fn consume_data(self) -> Vec<T> {
        self.data
    }

    fn point_from_index(&self, index: usize) -> Point {
        new_point(index / self.width, index % self.width)
    }

    pub fn items(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.data.iter()
    }

    pub fn items_with_points(&self) -> impl DoubleEndedIterator<Item = (Point, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(index, item)| (self.point_from_index(index), item))
    }

    pub fn lines(&self) -> impl DoubleEndedIterator<Item = &[T]> {
        (0..self.height).map(|row| {
            let index = row * self.width;
            &self.data[index..index + self.width]
        })
    }

    #[allow(dead_code)]
    pub fn columns(&self) -> impl DoubleEndedIterator<Item = Vec<&T>> {
        (0..self.width).map(|column| self.iter_column(column).unwrap().collect())
    }

    #[allow(dead_code)]
    pub fn iter_column(&self, index: usize) -> Option<impl DoubleEndedIterator<Item = &T>> {
        if index < self.width {
            Some((0..self.height).map(move |row| self.get(row, index).unwrap()))
        } else {
            None
        }
    }

    pub fn find(&self, to_find: &T) -> Option<Point>
    where
        T: Eq,
    {
        self.data
            .iter()
            .position(|item| *item == *to_find)
            .map(|index| self.point_from_index(index))
    }

    #[allow(dead_code)]
    pub fn get_width(&self) -> usize {
        self.width
    }

    #[allow(dead_code)]
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.height && y < self.width {
            Some(&self.data[x * self.width + y])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.height && y < self.width {
            Some(&mut self.data[x * self.width + y])
        } else {
            None
        }
    }

    pub fn get_at(&self, point: &Point) -> &T {
        let (x, y) = point.coordinaes();
        self.get(x, y).unwrap()
    }

    pub fn get_at_mut(&mut self, point: &Point) -> &mut T {
        let (x, y) = point.coordinaes();
        self.get_mut(x, y).unwrap()
    }

    pub fn from_str_by(s: &str, f: impl Fn(char) -> T) -> Self {
        let mut height = 0;
        let data = s
            .trim()
            .lines()
            .flat_map(|line| {
                height += 1;
                line.chars().map(&f).collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self {
            width: data.len() / height,
            height,
            data,
        }
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.lines().try_for_each(|line| {
            line.iter()
                .try_for_each(|item| write!(f, "{} ", item))
                .and_then(|_| writeln!(f))
        })
    }
}

impl<T> FromStr for Grid<T>
where
    T: From<char>,
{
    // TODO use error
    type Err = i32;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_str_by(s, |c| c.into()))
    }
}
