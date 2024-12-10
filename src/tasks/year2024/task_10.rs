use std::collections::HashSet;

use crate::{
    tasks::TaskRun,
    utils::grid::{Direction, Grid, Point},
};

pub struct Task10;

impl Task10 {
    fn find_paths(grid: &Grid<u32>, start: Point) -> Vec<Point> {
        let mut stack = vec![start];
        let mut ret = Vec::new();

        while let Some(point) = stack.pop() {
            let num = grid.get_at(&point);
            (0..8)
                .step_by(2)
                .filter_map(|i| point.adjacent(Direction::Up.clockwise(i), grid))
                .filter(|point| *grid.get_at(point) == num + 1)
                .for_each(|p| {
                    if *grid.get_at(&p) == 9 {
                        ret.push(p)
                    } else {
                        stack.push(p)
                    }
                });
        }

        ret
    }
}

impl TaskRun for Task10 {
    fn normal(input: &str) -> usize {
        let grid = Grid::<u32>::from_str_by(input, |c| c.to_digit(10).unwrap_or(99));
        grid.items_with_points()
            .filter(|(_, item)| **item == 0)
            .map(|(point, _)| Self::find_paths(&grid, point).iter().collect::<HashSet<_>>().len())
            .sum()
    }

    fn bonus(input: &str) -> usize {
        let grid = Grid::<u32>::from_str_by(input, |c| c.to_digit(10).unwrap_or(99));
        grid.items_with_points()
            .filter(|(_, item)| **item == 0)
            .map(|(point, _)| Self::find_paths(&grid, point).len())
            .sum()
    }
}

