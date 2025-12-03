use std::fmt::Display;

use crate::tasks::TaskRun;
use anyhow::Result;

pub struct Day03;

impl Day03 {
    fn find_batteries<const N: usize>(batteries: &[u64]) -> u64 {
        let mut index = 0_usize;
        (0..N)
            .map(|i| {
                let (new_index, value) = batteries[index..batteries.len() - N + 1 + i]
                    .iter()
                    .enumerate()
                    .rev() // Reverse because max by key get last max value
                    .max_by_key(|(_, v)| *v)
                    .unwrap();
                index += new_index + 1;
                *value
            })
            .fold(0_u64, |acc, num| 10 * acc + num)
    }

    fn process_line(line: &str) -> Vec<u64> {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<_>>()
    }
}

impl TaskRun for Day03 {
    fn normal(input: &str) -> Result<impl Display> {
        Ok(input
            .lines()
            .map(|line| Self::find_batteries::<2>(&Self::process_line(line)))
            .sum::<u64>())
    }

    fn bonus(input: &str) -> Result<impl Display> {
        Ok(input
            .lines()
            .map(|line| Self::find_batteries::<12>(&Self::process_line(line)))
            .sum::<u64>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_batteries_test() {
        assert_eq!(Day03::find_batteries::<2>(&[1, 2, 3, 4, 2]), 42);
        assert_eq!(Day03::find_batteries::<2>(&[1, 2, 3, 4]), 34);
        assert_eq!(Day03::find_batteries::<3>(&[1, 2, 3, 4, 2]), 342);
        assert_eq!(Day03::find_batteries::<3>(&[1, 2, 3, 4]), 234);
    }
}
