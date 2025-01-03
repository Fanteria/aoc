use anyhow::Result;
use std::fmt::Display;

use crate::tasks::TaskRun;
use crate::utils::Parser;
use ahash::AHashMap as HashMap;

pub struct Day01;

impl Day01 {
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

impl TaskRun for Day01 {
    fn normal(input: &str) -> Result<impl Display> {
        let (mut left, mut right) = Self::read_lines(input);
        left.sort();
        right.sort();
        Ok(left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum::<usize>())
    }

    fn bonus(input: &str) -> Result<impl Display> {
        let (left, right) = Self::read_lines(input);
        let mut map: HashMap<usize, usize> = HashMap::new();
        right.into_iter().for_each(|item| {
            *map.entry(item).or_insert(0) += 1;
        });
        Ok(left
            .into_iter()
            .filter_map(|num| Some(map.get(&num)? * num))
            .sum::<usize>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn read_lines() {
        let (left, right) = Day01::read_lines(EXAMPLE);
        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }
}
