use crate::tasks::TaskRun;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::fmt::Display;

pub struct Day09;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
    fn area(&self, other: &Point) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

impl TaskRun for Day09 {
    fn normal(input: &str) -> Result<impl Display> {
        input
            .lines()
            .filter_map(|line| line.split_once(','))
            .map(|(x, y)| Point::new(x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
            .tuple_combinations()
            .map(|(a, b)| a.area(&b))
            .max()
            .context("Max value is expected")
    }

    fn bonus(input: &str) -> Result<impl Display> {
        let points = input
            .lines()
            .filter_map(|line| line.split_once(','))
            .map(|(x, y)| Point::new(x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
            .collect::<Vec<_>>();
        let (a, b) = points
            .iter()
            .tuple_combinations()
            .sorted_by_key(|(a, b)| a.area(b))
            .rev()
            // Check if any line intersects with rectangles interior
            .find(|(a, b)| {
                let min = Point::new(a.x.min(b.x), a.y.min(b.y));
                let max = Point::new(a.x.max(b.x), a.y.max(b.y));
                !points.iter().circular_tuple_windows().any(|(c1, c2)| {
                    // Vertical line
                    c1.x == c2.x
                        && min.x < c1.x
                        && max.x > c1.x
                        && !(min.y >= c1.y.max(c2.y) || max.y <= c1.y.min(c2.y))
                        // Horizontal line
                        || c1.y == c2.y
                            && min.y < c1.y
                            && max.y > c1.y
                            && !(min.x >= c1.x.max(c2.x) || max.x <= c1.x.min(c2.x))
                })
            })
            .context("Does not find any suitable rectangle")?;
        Ok(a.area(b))
    }
}
