use std::fmt::Display;

use crate::{
    tasks::TaskRun,
    utils::grid::{Direction, Grid, Point},
};
use anyhow::Result;

#[derive(Debug)]
enum Place {
    Empty,
    RollOfPaper,
    Removed,
}

impl From<char> for Place {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '@' => Self::RollOfPaper,
            _ => panic!("Invalid input"),
        }
    }
}

impl Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Place::Empty => '.',
                Place::RollOfPaper => '@',
                Place::Removed => 'X',
            }
        )
    }
}

pub struct Day04;

impl Day04 {
    fn count_adjacent_rolls(grid: &Grid<Place>, point: &Point) -> usize {
        Direction::Up
            .iter()
            .filter_map(move |direction| {
                point
                    .adjacent(direction, grid)
                    .map(|point| matches!(*grid.get_at(&point), Place::RollOfPaper))
            })
            .filter(|a| *a)
            .count()
    }
}

impl TaskRun for Day04 {
    fn normal(input: &str) -> Result<impl Display> {
        let grid = &Grid::<Place>::from(input);
        Ok(grid
            .items_with_points()
            .filter(|(_, place)| matches!(place, Place::RollOfPaper))
            .filter(|(point, _)| Self::count_adjacent_rolls(grid, point) < 4)
            .count())
    }

    fn bonus(input: &str) -> Result<impl Display> {
        let mut grid = Grid::<Place>::from(input);
        let mut removed = 0;
        let mut rolls: Vec<_> = grid
            .points()
            .filter(|point| matches!(grid.get_at(point), Place::RollOfPaper))
            .collect();
        loop {
            let old_len = rolls.len();
            rolls.retain(|point| {
                if Self::count_adjacent_rolls(&grid, point) < 4 {
                    removed += 1;
                    *grid.get_at_mut(point) = Place::Removed;
                    false
                } else {
                    true
                }
            });
            if rolls.len() == old_len {
                break;
            }
        }
        Ok(removed)
    }
}
