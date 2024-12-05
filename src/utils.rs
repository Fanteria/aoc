mod parser;

pub use parser::Parser;

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub points: Vec<T>,
}

pub struct Point {
    pub x: usize,
    pub y: usize,
}

