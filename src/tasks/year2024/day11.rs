use crate::{tasks::TaskRun, utils::Parser};
use ahash::AHashMap as HashMap;
use anyhow::Result;
use std::fmt::Display;

pub struct Day11;

impl TaskRun for Day11 {
    fn normal(input: &str) -> Result<impl Display> {
        Ok(Parser::line_vec(input)?
            .into_iter()
            .map(|stone| calculate_lengths(stone, 25))
            .sum::<usize>())
    }

    fn bonus(input: &str) -> Result<impl Display> {
        Ok(Parser::line_vec(input)?
            .into_iter()
            .map(|stone| calculate_lengths(stone, 75))
            .sum::<usize>())
    }
}

fn blink(num: u64) -> (u64, Option<u64>) {
    if num == 0 {
        return (1, None);
    }
    let len = (num as f64).log10() as usize + 1;
    if len.is_multiple_of(2) {
        let pow10 = 10_u64.pow(len as u32 / 2);
        (num / pow10, Some(num % pow10))
    } else {
        (num * 2024, None)
    }
}

fn calculate_lengths(num: u64, times: u64) -> usize {
    let mut map: HashMap<u64, usize> = HashMap::default();
    map.insert(num, 1);
    for _ in 0..times {
        let mut new_map: HashMap<u64, usize> = HashMap::default();
        map.iter().for_each(|(stone, count)| {
            let (s1, s2) = blink(*stone);
            *new_map.entry(s1).or_insert(0) += count;
            if let Some(s2) = s2 {
                *new_map.entry(s2).or_insert(0) += count;
            }
        });
        map = new_map;
    }
    map.values().sum()
}
