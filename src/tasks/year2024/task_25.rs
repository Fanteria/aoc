use crate::tasks::TaskRun;
use itertools::Itertools;

pub struct Task25;

impl TaskRun for Task25 {
    fn normal(input: &str) -> usize
    where
        Self: Sized,
    {
        fn count<'a>(lines: impl Iterator<Item = &'a str> + 'a) -> [u32; 5] {
            lines.fold([0; 5], |mut lock, line| {
                line.chars().enumerate().for_each(|(i, c)| match c {
                    '#' => lock[i] += 1,
                    '.' => {}
                    _ => unreachable!(),
                });
                lock
            })
        }

        let (mut keys, mut locks): (Vec<_>, Vec<_>) = input.split("\n\n").partition_map(|block| {
            if block.starts_with("#") {
                itertools::Either::Right(count(block.lines().skip(1)))
            } else {
                itertools::Either::Left(count(block.lines().rev().skip(1)))
            }
        });
        keys.sort();
        locks.sort();
        locks
            .iter()
            .map(|lock| {
                keys.iter()
                    .filter(|key| (0..lock.len()).all(|i| lock[i] + key[i] < 6))
                    .count()
            })
            .sum()
    }

    fn bonus(_input: &str) -> usize
    where
        Self: Sized,
    {
        todo!()
    }
}
