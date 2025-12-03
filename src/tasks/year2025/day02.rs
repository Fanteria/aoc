use anyhow::Result;
use std::{cmp::Ordering, fmt::Display, str::FromStr};

use crate::{
    tasks::TaskRun,
    utils::{number_of_digits, Parser},
};

pub struct Day02;

struct Interval {
    from: u64,
    to: u64,
}

impl FromStr for Interval {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (from, to) = s.split_once('-').unwrap();
        Ok(Self {
            from: from.parse()?,
            to: to.parse()?,
        })
    }
}

impl Day02 {
    fn generate_pairs(from: u64, to: u64) -> Vec<u64> {
        fn cmd_number(num: u64) -> (u64, Ordering) {
            let nod = number_of_digits(num as f64);
            if nod.is_multiple_of(2) {
                let base = 10_u64.pow(nod as u32 / 2);
                match num / base {
                    x if x < num % base => (x, Ordering::Greater),
                    x if x > num % base => (x, Ordering::Less),
                    x => (x, Ordering::Equal),
                }
            } else {
                (10_u64.pow(nod as u32 / 2), Ordering::Less)
            }
        }
        let from = match cmd_number(from) {
            (x, Ordering::Greater) => x + 1,
            (x, Ordering::Less) => x,
            (x, Ordering::Equal) => x,
        };
        let to = match cmd_number(to) {
            (x, Ordering::Greater) => x,
            (x, Ordering::Less) => x - 1,
            (x, Ordering::Equal) => x,
        };
        (from..=to)
            .map(|i| {
                let nod = number_of_digits(i as u32);
                i * (10_u64.pow(nod as u32) + 1)
            })
            .collect()
    }

    fn generate_options() -> Vec<u64> {
        let max_number = 10_000_000_000_u64;
        let multipliers = [10, 100, 1_000, 10_000, 100_000]
            .into_iter()
            .map(|base| {
                let mut vec = Vec::new();
                let mut new = 1;
                loop {
                    new = new * base + 1;
                    if new >= max_number {
                        break;
                    }
                    vec.push(new);
                }
                vec
            })
            .collect::<Vec<_>>();
        let mut options = (1..99_999_u64)
            .flat_map(|i| {
                let nod = number_of_digits(i as u32);
                multipliers[nod - 1]
                    .iter()
                    .map(|multiplier| multiplier * i)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        options.sort();
        options.dedup();
        options
    }

    fn find_interval_options<'a>(options: &'a [u64], interval: &Interval) -> &'a [u64] {
        let from = match options.binary_search(&interval.from) {
            Ok(index) => index,
            Err(index) => index,
        };
        let to = match options.binary_search(&interval.to) {
            Ok(index) => index,
            Err(index) => index - 1,
        };
        &options[from..=to]
    }
}

impl TaskRun for Day02 {
    fn normal(input: &str) -> Result<impl Display> {
        Ok(Parser::iter_line_sep::<Interval>(input, ",")
            .flat_map(|Interval { from, to }| Self::generate_pairs(from, to))
            .sum::<u64>())
    }

    fn bonus(input: &str) -> Result<impl Display> {
        let options = Self::generate_options();
        Ok(Parser::iter_line_sep::<Interval>(input, ",")
            .flat_map(|interval| {
                Self::find_interval_options(&options, &interval)
            })
            .sum::<u64>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_pairs_test() {
        assert_eq!(
            Day02::generate_pairs(2, 2025),
            vec![
                11, 22, 33, 44, 55, 66, 77, 88, 99, 1010, 1111, 1212, 1313, 1414, 1515, 1616, 1717,
                1818, 1919, 2020
            ]
        );
    }

    #[test]
    fn generate_options_test() {
        assert_eq!(
            Day02::generate_options()[..10],
            vec![11, 22, 33, 44, 55, 66, 77, 88, 99, 111]
        );
    }

    #[test]
    fn find_interval_options_test() {
        let options = [1, 5, 7, 9, 12, 18];
        assert_eq!(
            Day02::find_interval_options(&options, &Interval { from: 6, to: 15 }),
            [7, 9, 12]
        );
        assert_eq!(
            Day02::find_interval_options(&options, &Interval { from: 7, to: 12 }),
            [7, 9, 12]
        );
    }
}
