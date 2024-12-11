use std::collections::HashMap;

use crate::{tasks::TaskRun, utils::Parser};

pub struct Task11;

impl TaskRun for Task11 {
    fn normal(input: &str) -> usize {
        let stones = Parser::iter_vec::<u64>(input).take(1).next().unwrap();
        stones
            .into_iter()
            .map(|stone| calculate_lengths(stone, 25))
            .sum()
    }

    fn bonus(input: &str) -> usize {
        let stones = Parser::iter_vec::<u64>(input).take(1).next().unwrap();
        stones
            .into_iter()
            .map(|stone| calculate_lengths(stone, 75))
            .sum()
    }
}

fn blink(num: u64) -> (u64, Option<u64>) {
    if num == 0 {
        return (1, None);
    }
    let len = (num as f64).log10() as usize + 1;
    if len % 2 == 0 {
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
    map.iter().map(|(_, times)| times).sum()
}
