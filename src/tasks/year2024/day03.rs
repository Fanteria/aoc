use anyhow::{anyhow, Result};
use regex::{Captures, Regex};
use std::fmt::Display;

use crate::tasks::TaskRun;

pub struct Day03;

impl TaskRun for Day03 {
    fn normal(input: &str) -> Result<impl Display> {
        Regex::new(r"mul\((\d+),(\d+)\)")?
            .captures_iter(input)
            .map(|c| c.extract::<2>().1)
            .map(|v| v.try_map(|v| v.parse::<usize>()))
            .try_fold(0, |result, v| Ok(result + v.map(|v| v[0] * v[1])?))
    }

    fn bonus(input: &str) -> Result<impl Display> {
        fn get_val(c: &Captures<'_>, index: usize) -> Result<usize> {
            Ok(c.get(index)
                .ok_or_else(|| anyhow!("mul value missing"))?
                .as_str()
                .parse::<usize>()?)
        }

        let mut do_multiply = true;
        Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")?
            .captures_iter(input)
            .try_fold(0, |mut result, c| {
                match c.get(0).map(|m| m.as_str()) {
                    Some("do()") => do_multiply = true,
                    Some("don't()") => do_multiply = false,
                    Some(_) if do_multiply => result += get_val(&c, 1)? * get_val(&c, 2)?,
                    _ => {}
                };
                Ok(result)
            })
    }
}
