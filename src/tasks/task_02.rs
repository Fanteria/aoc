use std::io::{BufRead, BufReader, Read};

use super::TaskRun;

pub struct Task02;

impl Task02 {
    fn read_lines<R: Read>(reader: R) -> Vec<Vec<usize>> {
        let reader = BufReader::new(reader);
        reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                line.split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect()
    }

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
    fn normal<R: Read>(reader: R) -> usize {
        let records = Self::read_lines(reader);
        records
            .iter()
            .filter(|record| Self::is_valid(record))
            .count()
    }

    fn bonus<R: Read>(reader: R) -> usize {
        let records = Self::read_lines(reader);
        records
            .iter()
            .filter(|record| {
                Self::is_valid(record)
                    || ((0..record.len())
                        .any(|i| Self::is_valid(&[&record[..i], &record[i + 1..]].concat())))
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::{Task, TaskType};
    use test::Bencher;

    #[test]
    fn normal() {
        let t = Task::new(2, TaskType::Normal);
        assert_eq!(
            t.run(t.get_example_in_path()),
            t.get_output(t.get_example_out_path())
        );
        assert_eq!(t.run(t.get_in_path()), t.get_output(t.get_out_path()));
    }

    #[test]
    fn bonus() {
        let t = Task::new(2, TaskType::Bonus);
        assert_eq!(
            t.run(t.get_example_in_path()),
            t.get_output(t.get_example_out_path())
        );
        assert_eq!(t.run(t.get_in_path()), t.get_output(t.get_out_path()));
    }

    #[bench]
    fn normal_bench(b: &mut Bencher) {
        let t = Task::new(2, TaskType::Normal);
        let input = t.get_input(t.get_in_path());
        b.iter(|| Task02::normal(input.as_bytes()))
    }

    #[bench]
    fn bonus_bench(b: &mut Bencher) {
        let t = Task::new(2, TaskType::Bonus);
        let input = t.get_input(t.get_in_path());
        b.iter(|| Task02::bonus(input.as_bytes()))
    }
}
