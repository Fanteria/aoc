use crate::tasks::TaskRun;
use rayon::prelude::*;

pub struct Task07;

impl Task07 {
    fn read(input: &str) -> impl Iterator<Item = (usize, Vec<usize>)> + '_ {
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
            } else if evaluate(numbers, act + numbers[index], index + 1, expected)
                || evaluate(numbers, act * numbers[index], index + 1, expected)
            {
                true
            } else {
                evaluate(
                    numbers,
                    (act * 10_usize.pow(numbers[index].checked_ilog10().unwrap_or(0) + 1))
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
