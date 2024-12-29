use crate::{tasks::TaskRun, utils::Parser};
use std::fmt::Display;

pub struct Task02;

impl Task02 {
    fn is_cending<'a, I>(iter: I) -> bool
    where
        I: Iterator<Item = &'a usize> + Clone,
    {
        iter.clone()
            .zip(iter.skip(1))
            .all(|(prev, next)| prev > next && (prev.abs_diff(*next) <= 3))
    }

    fn is_valid(records: &[usize]) -> bool {
        Self::is_cending(records.iter()) || Self::is_cending(records.iter().rev())
    }
}

impl TaskRun for Task02 {
    fn normal(input: &str) -> impl Display {
        Parser::iter_vec::<usize>(input)
            .filter(|record| Self::is_valid(record))
            .count()
    }

    fn bonus(input: &str) -> impl Display {
        Parser::iter_vec::<usize>(input)
            .filter(|record| {
                Self::is_valid(record)
                    || ((0..record.len())
                        .any(|i| Self::is_valid(&[&record[..i], &record[i + 1..]].concat())))
            })
            .count()
    }
}
