use crate::tasks::TaskRun;
use std::fmt::Display;
use crate::utils::grid::{Grid, Path, Point};
use ahash::{AHashMap as HashMap, AHashSet as HashSet};
use itertools::Itertools;
use std::str::FromStr;

pub struct Task08;

impl Task08 {
    fn read(input: &str) -> (Grid<char>, HashMap<char, HashSet<Point>>) {
        let grid = Grid::<char>::from_str(input).unwrap();
        let mut anthennas: HashMap<char, HashSet<Point>> = HashMap::new();
        grid.items_with_points()
            .filter(|(_, c)| **c != '.')
            .for_each(|(point, c)| {
                anthennas.entry(*c).or_default().insert(point);
            });

        (grid, anthennas)
    }
}

impl TaskRun for Task08 {
    fn normal(input: &str) -> impl Display {
        let (grid, anthennas) = Self::read(input);
        anthennas
            .into_iter()
            .flat_map(|(_, points)| {
                points.into_iter().permutations(2).filter_map(|pair| {
                    let path = Path::from_points(&pair[0], &pair[1]);
                    pair[1].move_along_path(&path, &grid)
                })
            })
            .collect::<HashSet<_>>()
            .len()
    }

    fn bonus(input: &str) -> impl Display {
        let (grid, anthennas) = Self::read(input);
        anthennas
            .into_iter()
            .flat_map(|(_, points)| {
                points.into_iter().permutations(2).map(|mut antinodes| {
                    let path = Path::from_points(&antinodes[0], &antinodes[1]);
                    while let Some(new) = antinodes.last().unwrap().move_along_path(&path, &grid) {
                        antinodes.push(new);
                    }
                    antinodes
                })
            })
            .flatten()
            .collect::<HashSet<_>>()
            .len()
    }
}
