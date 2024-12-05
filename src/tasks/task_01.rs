use crate::utils::Parser;

use super::TaskRun;
use std::collections::HashMap;

pub struct Task01;

impl Task01 {
    fn read_lines(input: &str) -> (Vec<usize>, Vec<usize>) {
        let mut left: Vec<usize> = Vec::new();
        let mut right: Vec<usize> = Vec::new();
        Parser::iter_array::<usize, 2>(input).for_each(|line| {
            left.push(line[0]);
            right.push(line[1]);
        });
        (left, right)
    }
}

impl TaskRun for Task01 {
    fn normal(input: &str) -> usize {
        let (mut left, mut right) = Self::read_lines(input);
        left.sort();
        right.sort();
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum()
    }

    fn bonus(input: &str) -> usize {
        let (left, right) = Self::read_lines(input);
        let mut map: HashMap<usize, usize> = HashMap::new();
        right.into_iter().for_each(|item| {
            *map.entry(item).or_insert(0) += 1;
        });
        left.into_iter()
            .filter_map(|num| Some(map.get(&num)? * num))
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::{Task, TaskType};
    use test::Bencher;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn read_lines() {
        let (left, right) = Task01::read_lines(EXAMPLE);
        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn normal() {
        let t = Task::new(1, TaskType::Normal);
        assert_eq!(
            t.run(t.get_example_in_path()),
            t.get_output(t.get_example_out_path())
        );
        assert_eq!(t.run(t.get_in_path()), t.get_output(t.get_out_path()));
    }

    #[test]
    fn bonus() {
        let t = Task::new(1, TaskType::Bonus);
        assert_eq!(
            t.run(t.get_example_in_path()),
            t.get_output(t.get_example_out_path())
        );
        assert_eq!(t.run(t.get_in_path()), t.get_output(t.get_out_path()));
    }

    #[bench]
    fn normal_bench(b: &mut Bencher) {
        let t = Task::new(1, TaskType::Normal);
        let input = Task::get_input(t.get_in_path());
        b.iter(|| Task01::normal(&input))
    }

    #[bench]
    fn bonus_bench(b: &mut Bencher) {
        let t = Task::new(1, TaskType::Bonus);
        let input = Task::get_input(t.get_in_path());
        b.iter(|| Task01::bonus(&input))
    }
}
