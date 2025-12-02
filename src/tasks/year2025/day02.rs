use anyhow::Result;
use std::{cmp::Ordering, fmt::Display, str::FromStr};

use crate::{
    tasks::TaskRun,
    utils::{number_of_digits, Parser},
};

pub struct Day02;

// From the same example as before:
// 
//     11-22 still has two invalid IDs, 11 and 22.
//     95-115 now has two invalid IDs, 99 and 111.
//     998-1012 now has two invalid IDs, 999 and 1010.
//     1188511880-1188511890 still has one invalid ID, 1188511885.
//     222220-222224 still has one invalid ID, 222222.
//     1698522-1698528 still contains no invalid IDs.
//     446443-446449 still has one invalid ID, 446446.
//     38593856-38593862 still has one invalid ID, 38593859.
//     565653-565659 now has one invalid ID, 565656.
//     824824821-824824827 now has one invalid ID, 824824824.
//     2121212118-2121212124 now has one invalid ID, 2121212121.
// 
// Adding up all the invalid IDs in this example produces 4174379265

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

    fn generate(from: u64, to: u64) {

    }
}

impl TaskRun for Day02 {
    fn normal(input: &str) -> Result<impl Display> {
        Ok(Parser::iter_line_sep::<Interval>(input, ",")
            .flat_map(|Interval { from, to }| {
                let vec = Self::generate_pairs(from, to);
                println!("from: {from}, to: {to} = {vec:?}");
                vec
            })
            .sum::<u64>())
    }

    fn bonus(input: &str) -> Result<impl Display> {
        Ok("")
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
}
