use crate::tasks::TaskRun;
use crate::utils::grid::{Direction, Grid, Point};
use ahash::AHashMap as HashMap;
use anyhow::{Context, Result};
use rayon::prelude::*;
use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Copy)]
enum PointState {
    Barrier,
    Empty,
    Visited,
}

impl From<char> for PointState {
    fn from(value: char) -> Self {
        match value {
            '#' => PointState::Barrier,
            '.' => PointState::Empty,
            '^' => PointState::Visited,
            _ => panic!("Unknown tile"),
        }
    }
}

impl Display for PointState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PointState::Barrier => '#',
                PointState::Empty => '.',
                PointState::Visited => 'X',
            }
        )
    }
}

pub struct Day06;

impl Day06 {
    fn guard_travel(mut grid: Grid<PointState>) -> Result<Grid<PointState>> {
        let mut direction = Direction::Up;
        let mut point = grid
            .find(&PointState::Visited)
            .context("Start position missing")?;

        while let Some(next_point) = point.adjacent(direction, &grid) {
            match grid.get_at(&next_point) {
                PointState::Barrier => direction = direction.clockwise(2),
                PointState::Empty => {
                    point = next_point;
                    *grid.get_at_mut(&point) = PointState::Visited;
                }
                PointState::Visited => point = next_point,
            };
        }
        Ok(grid)
    }

    fn is_cycle(grid: &Grid<PointState>, start: Point) -> bool {
        let mut point = start;
        let mut direction = Direction::Up;
        let mut hitted_barriers: HashMap<Point, [bool; 4]> = HashMap::new();

        while let Some(next_point) = point.adjacent(direction, grid) {
            match grid.get_at(&next_point) {
                PointState::Barrier => {
                    let directions = hitted_barriers.entry(next_point).or_default();
                    match direction {
                        Direction::Up if directions[0] => return true,
                        Direction::Up => directions[0] = true,
                        Direction::Right if directions[1] => return true,
                        Direction::Right => directions[1] = true,
                        Direction::Down if directions[2] => return true,
                        Direction::Down => directions[2] = true,
                        Direction::Left if directions[3] => return true,
                        Direction::Left => directions[3] = true,
                        _ => unreachable!(),
                    };
                    direction = direction.clockwise(2)
                }
                _ => point = next_point,
            };
        }
        false
    }
}

impl TaskRun for Day06 {
    fn normal(input: &str) -> Result<impl Display> {
        let grid = Grid::<PointState>::from(input);
        Ok(Self::guard_travel(grid)?
            .items()
            .filter(|item| **item == PointState::Visited)
            .count())
    }

    fn bonus(input: &str) -> Result<impl Display> {
        let grid = Grid::<PointState>::from(input);
        let start = grid
            .find(&PointState::Visited)
            .context("Start position missing")?;
        let mut grid = Self::guard_travel(grid)?;
        *grid.get_at_mut(&start) = PointState::Empty;

        Ok(grid
            .items_with_points()
            .filter(|(_, item)| **item == PointState::Visited)
            .par_bridge()
            .filter(|(point, _)| {
                let mut grid = grid.clone();
                *grid.get_at_mut(point) = PointState::Barrier;
                Self::is_cycle(&grid, start)
            })
            .count())
    }
}
