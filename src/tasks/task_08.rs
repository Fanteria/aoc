use super::TaskRun;
use crate::utils::grid::{Grid, Path, Point};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

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
    fn normal(input: &str) -> usize {
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

    fn bonus(input: &str) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::{Task, TaskType};
    use test::Bencher;

    #[test]
    fn normal_examples() {
        let t = Task::new(8, TaskType::Normal);
        assert_eq!(
            t.run(t.get_custom_example_in_path("2")),
            t.get_output(t.get_custom_example_out_path("2"))
        );
        assert_eq!(
            t.run(t.get_custom_example_in_path("3")),
            t.get_output(t.get_custom_example_out_path("3"))
        );
    }

    #[test]
    fn normal() {
        let t = Task::new(8, TaskType::Normal);
        assert_eq!(
            t.run(t.get_example_in_path()),
            t.get_output(t.get_example_out_path())
        );
        assert_eq!(t.run(t.get_in_path()), t.get_output(t.get_out_path()));
    }

    #[test]
    fn bonus_examples() {
        let t = Task::new(8, TaskType::Bonus);
        assert_eq!(
            t.run(t.get_custom_example_in_path("4")),
            t.get_output(t.get_custom_example_out_path("4"))
        );
    }

    #[test]
    fn bonus() {
        let t = Task::new(8, TaskType::Bonus);
        assert_eq!(
            t.run(t.get_example_in_path()),
            t.get_output(t.get_example_out_path())
        );
        assert_eq!(t.run(t.get_in_path()), t.get_output(t.get_out_path()));
    }

    #[bench]
    fn normal_bench(b: &mut Bencher) {
        let t = Task::new(8, TaskType::Normal);
        let input = Task::get_input(t.get_in_path());
        b.iter(|| Task08::normal(&input))
    }

    #[bench]
    fn bonus_bench(b: &mut Bencher) {
        let t = Task::new(8, TaskType::Bonus);
        let input = Task::get_input(t.get_in_path());
        b.iter(|| Task08::bonus(&input))
    }
}
