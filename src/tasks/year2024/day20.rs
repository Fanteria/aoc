use crate::{
    tasks::TaskRun,
    utils::grid::{Direction, Grid, Point},
};
use ahash::AHashMap as HashMap;
use anyhow::{Context, Result};
use rayon::iter::*;
use std::fmt::Display;

pub struct Day20;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Wall,
    Track,
    Start,
    End,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '.' => Self::Track,
            'S' => Self::Start,
            'E' => Self::End,
            _ => unreachable!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Wall => '#',
                Tile::Track => '.',
                Tile::Start => 'S',
                Tile::End => 'E',
            }
        )
    }
}

fn get_path(grid: &Grid<Tile>) -> Result<HashMap<Point, usize>> {
    let mut visited: HashMap<Point, usize> = HashMap::new();
    let mut last = grid.find(&Tile::Start).context("Cannot find start")?;
    let mut i = 0;
    visited.insert(last, i);
    loop {
        if *grid.get_at(&last) == Tile::End {
            break;
        }
        i += 1;
        last = Direction::Up
            .iter()
            .step_by(2)
            .filter_map(|direction| last.adjacent(direction, grid))
            .filter(|point| {
                (*grid.get_at(point) == Tile::Track || *grid.get_at(point) == Tile::End)
                    && !visited.contains_key(point)
            })
            .collect::<Vec<_>>()[0];
        visited.insert(last, i);
    }
    Ok(visited)
}

fn possible_cheats<'a>(
    grid: &'a Grid<Tile>,
    point: &'a Point,
    distance: i32,
) -> impl Iterator<Item = Point> + 'a {
    let (x, y) = point.coordinaes();
    (-distance..=distance).flat_map(move |i| {
        let max_y = distance - i.abs();
        (-max_y..=max_y)
            .filter_map(move |j| Point::new((x as i32 + i) as usize, (y as i32 + j) as usize, grid))
    })
}

fn count_cheats(grid: &Grid<Tile>, max_cheat_time: i32) -> Result<usize> {
    let path = get_path(grid)?;
    Ok(path
        .par_iter()
        .map(|(start, from_start)| {
            possible_cheats(grid, start, max_cheat_time)
                // TODO write this filter some more readable way...
                .filter(|point| {
                    path.get(point).is_some_and(|end_point_from_start| {
                        *end_point_from_start > *from_start
                            && end_point_from_start - *from_start - start.manhattan_distance(point)
                                >= 100
                    })
                })
                .count()
        })
        .sum())
}

impl TaskRun for Day20 {
    fn normal(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        count_cheats(&Grid::<Tile>::from(input), 2)
    }

    fn bonus(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        count_cheats(&Grid::<Tile>::from(input), 20)
    }
}
