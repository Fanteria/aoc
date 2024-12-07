use crate::utils::grid::{Direction, Grid, Point};
use rayon::prelude::*;

use super::TaskRun;

pub struct Task07;

impl Task07 {
    fn read<'a>(input: &'a str) -> impl Iterator<Item = (usize, Vec<usize>)> + 'a {
        input.lines().map(|line| {
            let (left, right) = line.split_once(':').unwrap();
            (
                left.parse().unwrap(),
                right
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
    }
}

impl TaskRun for Task07 {
    fn normal(input: &str) -> usize {
        fn evaluate(numbers: &[usize], act: usize, index: usize, expected: usize) -> bool {
            if index == numbers.len() {
                act == expected
            } else if evaluate(numbers, act + numbers[index], index + 1, expected) {
                true
            } else {
                evaluate(numbers, act * numbers[index], index + 1, expected)
            }
        }

        Self::read(input)
            .par_bridge()
            .filter_map(|(result, numbers)| {
                if evaluate(numbers.as_slice(), numbers[0], 1, result) {
                    Some(result)
                } else {
                    None
                }
            })
            .sum()
    }

    fn bonus(input: &str) -> usize {
        fn evaluate(numbers: &[usize], act: usize, index: usize, expected: usize) -> bool {
            if index == numbers.len() {
                act == expected
            } else if evaluate(numbers, act + numbers[index], index + 1, expected) {
                true
            } else if evaluate(numbers, act * numbers[index], index + 1, expected) {
                true
            } else {
                evaluate(
                    numbers,
                    (act * (10 as usize).pow(numbers[index].checked_ilog10().unwrap_or(0) + 1))
                        + numbers[index],
                    index + 1,
                    expected,
                )
            }
        }

        Self::read(input)
            .par_bridge()
            .filter_map(|(result, numbers)| {
                if evaluate(numbers.as_slice(), numbers[0], 1, result) {
                    Some(result)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::{Task, TaskType};
    use test::Bencher;

    #[test]
    fn normal() {
        let t = Task::new(7, TaskType::Normal);
        assert_eq!(
            t.run(t.get_example_in_path()),
            t.get_output(t.get_example_out_path())
        );
        assert_eq!(t.run(t.get_in_path()), t.get_output(t.get_out_path()));
    }

    #[test]
    fn bonus() {
        let t = Task::new(7, TaskType::Bonus);
        assert_eq!(
            t.run(t.get_example_in_path()),
            t.get_output(t.get_example_out_path())
        );
        assert_eq!(t.run(t.get_in_path()), t.get_output(t.get_out_path()));
    }

    #[bench]
    fn normal_bench(b: &mut Bencher) {
        let t = Task::new(7, TaskType::Normal);
        let input = Task::get_input(t.get_in_path());
        b.iter(|| Task07::normal(&input))
    }

    #[bench]
    fn bonus_bench(b: &mut Bencher) {
        let t = Task::new(7, TaskType::Bonus);
        let input = Task::get_input(t.get_in_path());
        b.iter(|| Task07::bonus(&input))
    }
}
