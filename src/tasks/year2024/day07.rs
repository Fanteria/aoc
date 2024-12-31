use crate::tasks::TaskRun;
use anyhow::{anyhow, Result};
use rayon::prelude::*;
use std::{fmt::Display, num::ParseIntError};

pub struct Day07;

impl Day07 {
    fn read(input: &str) -> Result<Vec<(usize, Vec<usize>)>> {
        fn read_line(line: &str) -> Result<(usize, Vec<usize>)> {
            let (left, right) = line
                .split_once(':')
                .ok_or_else(|| anyhow!("Cannot split against ':'"))?;
            Ok((
                left.parse()?,
                right
                    .split_whitespace()
                    .map(|num| num.parse())
                    .collect::<std::result::Result<Vec<_>, ParseIntError>>()?,
            ))
        }
        input.lines().map(read_line).collect()
    }
}

impl TaskRun for Day07 {
    fn normal(input: &str) -> Result<impl Display> {
        fn evaluate(numbers: &[usize], act: usize, index: usize, expected: usize) -> bool {
            if index == numbers.len() {
                act == expected
            } else if evaluate(numbers, act + numbers[index], index + 1, expected) {
                true
            } else {
                evaluate(numbers, act * numbers[index], index + 1, expected)
            }
        }

        Ok(Self::read(input)?
            .par_iter()
            .filter_map(|(result, numbers)| {
                if evaluate(numbers.as_slice(), numbers[0], 1, *result) {
                    Some(result)
                } else {
                    None
                }
            })
            .sum::<usize>())
    }

    fn bonus(input: &str) -> Result<impl Display> {
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

        Ok(Self::read(input)?
            .par_iter()
            .filter_map(|(result, numbers)| {
                if evaluate(numbers.as_slice(), numbers[0], 1, *result) {
                    Some(result)
                } else {
                    None
                }
            })
            .sum::<usize>())
    }
}
