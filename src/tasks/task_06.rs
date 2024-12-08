use std::{fmt::Display, str::FromStr};

use crate::utils::grid::{Direction, Grid, Point};
use rayon::prelude::*;

use super::TaskRun;

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

const INITIAL_POINT_HISTORY: PointHistory = PointHistory::Visited([true, false, false, false]);
#[derive(Clone, Copy, PartialEq, Eq)]
enum PointHistory {
    Barrier,
    Empty,
    // [Up, Down, Left, Right]
    Visited([bool; 4]),
}

impl From<char> for PointHistory {
    fn from(value: char) -> Self {
        match value {
            '#' => PointHistory::Barrier,
            '.' => PointHistory::Empty,
            '^' => INITIAL_POINT_HISTORY,
            _ => panic!("Unknown tile"),
        }
    }
}

impl Display for PointHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PointHistory::Barrier => '#',
                PointHistory::Empty => '.',
                PointHistory::Visited([true, false, false, false]) => '^',
                PointHistory::Visited([false, true, false, false]) => 'v',
                PointHistory::Visited([false, false, true, false]) => '<',
                PointHistory::Visited([false, false, false, true]) => '>',
                PointHistory::Visited([true, true, false, false]) => '|',
                PointHistory::Visited([false, false, true, true]) => '-',
                PointHistory::Visited([true, true, true, true]) => '+',
                _ => '?',
            }
        )
    }
}

pub struct Task06;

impl Task06 {
    fn guard_travel(mut grid: Grid<PointState>) -> Grid<PointState> {
        let mut direction = Direction::Up;
        let mut point = grid.find(&PointState::Visited).unwrap();

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
        grid
    }

    fn is_cycle(mut grid: Grid<PointHistory>, start: Point) -> bool {
        let mut point = start;
        let mut direction = Direction::Up;

        while let Some(next_point) = point.adjacent(direction, &grid) {
            match grid.get_at_mut(&next_point) {
                PointHistory::Barrier => direction = direction.clockwise(2),
                PointHistory::Empty => {
                    point = next_point;
                    *grid.get_at_mut(&point) = PointHistory::Visited(match direction {
                        Direction::Up => [true, false, false, false],
                        Direction::Down => [false, true, false, false],
                        Direction::Left => [false, false, true, false],
                        Direction::Right => [false, false, false, true],
                        _ => unreachable!(),
                    })
                }
                PointHistory::Visited(history) => {
                    point = next_point;
                    match direction {
                        Direction::Up if history[0] => return true,
                        Direction::Up => history[0] = true,
                        Direction::Down if history[1] => return true,
                        Direction::Down => history[1] = true,
                        Direction::Left if history[2] => return true,
                        Direction::Left => history[2] = true,
                        Direction::Right if history[3] => return true,
                        Direction::Right => history[3] = true,
                        _ => unreachable!(),
                    };
                }
            };
        }
        false
    }
}

impl TaskRun for Task06 {
    fn normal(input: &str) -> usize {
        let grid = Grid::<PointState>::from_str(input).unwrap();
        Self::guard_travel(grid)
            .items()
            .filter(|item| **item == PointState::Visited)
            .count()
    }

    fn bonus(input: &str) -> usize {
        let original_grid = Grid::<PointHistory>::from_str(input).unwrap();
        let start = original_grid.find(&INITIAL_POINT_HISTORY).unwrap();

        let mut grid = Self::guard_travel(Grid::<PointState>::from_str(input).unwrap());
        *grid.get_at_mut(&start) = PointState::Empty;

        grid.items_with_points()
            .filter(|(_, item)| **item == PointState::Visited)
            .par_bridge()
            .filter(|(point, _)| {
                let mut grid = original_grid.clone();
                *grid.get_at_mut(point) = PointHistory::Barrier;
                Self::is_cycle(grid, start.clone())
            })
            .count()
    }
}

