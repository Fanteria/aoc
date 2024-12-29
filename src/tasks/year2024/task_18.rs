use crate::{
    tasks::TaskRun,
    utils::grid::Direction,
    utils::{
        grid::{Grid, Point},
        Parser,
    },
};
use anyhow::{Context, Result};
use std::{collections::VecDeque, fmt::Display};

pub struct Task18;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum Tail {
    FallByte,
    #[default]
    Empty,
    Visited,
}

impl Display for Tail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FallByte => write!(f, "#"),
            Self::Empty => write!(f, "."),
            Self::Visited => write!(f, "O"),
        }
    }
}

fn find_path(mut grid: Grid<Tail>, start: Point) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((pos, count)) = queue.pop_front() {
        if *grid.get_at(&pos) == Tail::Visited {
            continue;
        }
        if pos.coordinaes() == (0, 0) {
            return Some(count);
        }
        *grid.get_at_mut(&pos) = Tail::Visited;
        queue.extend(
            Direction::Up
                .iter()
                .step_by(2)
                .filter_map(|direction| pos.adjacent(direction, &grid))
                .filter(|new_pos| *grid.get_at(new_pos) == Tail::Empty)
                .map(|new_pos| (new_pos, count + 1)),
        );
    }
    None
}

impl TaskRun for Task18 {
    fn normal(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        // const GRID_SIZE: usize = 6;
        // const FALL_BYTE_SIZE: usize = 12;
        const GRID_SIZE: usize = 70;
        const FALL_BYTE_SIZE: usize = 1024;
        let mut grid = Grid::<Tail>::from_dimensions(GRID_SIZE + 1, GRID_SIZE + 1);
        Parser::iter_array_sep::<usize, 2>(input, ",")
            .take(FALL_BYTE_SIZE)
            .for_each(|coord| {
                *grid.get_mut(coord[1], coord[0]).unwrap() = Tail::FallByte;
            });
        let start = Point::new(GRID_SIZE, GRID_SIZE, &grid).unwrap();
        find_path(grid, start).context("Path not found")
    }

    fn bonus(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        // const GRID_SIZE: usize = 6;
        // const FALL_BYTE_SIZE: usize = 12;
        const GRID_SIZE: usize = 70;
        const FALL_BYTE_SIZE: usize = 1024;
        let mut grid = Grid::<Tail>::from_dimensions(GRID_SIZE + 1, GRID_SIZE + 1);
        let start = Point::new(GRID_SIZE, GRID_SIZE, &grid).unwrap();
        let fall_bytes = Parser::iter_array_sep::<usize, 2>(input, ",").collect::<Vec<_>>();
        fall_bytes
            .iter()
            .take(FALL_BYTE_SIZE)
            .for_each(|coord| *grid.get_mut(coord[1], coord[0]).unwrap() = Tail::FallByte);

        let mut max_index = fall_bytes.len() - 1;
        let mut min_index = FALL_BYTE_SIZE;

        loop {
            let mut new_grid = grid.clone();
            let index_shift = (max_index - min_index) / 2;
            fall_bytes
                .iter()
                .take(min_index + index_shift)
                .skip(FALL_BYTE_SIZE - 1)
                .for_each(|coord| *new_grid.get_mut(coord[1], coord[0]).unwrap() = Tail::FallByte);
            match find_path(new_grid, start.clone()) {
                Some(_) => min_index += index_shift,
                None => max_index -= index_shift,
            }
            if min_index + 1 >= max_index {
                break;
            }
        }
        Ok(fall_bytes[min_index][0] * 100 + fall_bytes[min_index][1])
    }
}
