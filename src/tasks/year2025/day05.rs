use crate::tasks::TaskRun;
use anyhow::{Context, Result};
use std::{fmt::Display, ops::RangeInclusive};

pub struct Day05;

impl Day05 {
    fn read_input(input: &str) -> Result<(Vec<RangeInclusive<u64>>, Vec<u64>)> {
        let (intervals, ids) = input
            .split_once("\n\n")
            .context("Cannot split to ranges and IDs")?;
        let mut ranges: Vec<_> = intervals
            .lines()
            .map(|line| {
                let (from, to) = line.split_once('-').unwrap();
                Ok(RangeInclusive::new(
                    from.parse::<u64>()?,
                    to.parse::<u64>()?,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        ranges.sort_by_key(|a| *a.start());
        Ok((
            ranges,
            ids.lines()
                .map(|line| Ok(line.parse::<u64>()?))
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

impl TaskRun for Day05 {
    fn normal(input: &str) -> Result<impl Display> {
        let (ranges, ids) = Self::read_input(input)?;
        Ok(ids
            .iter()
            .filter(|id| {
                for interval in ranges.iter() {
                    if interval.contains(id) {
                        return true;
                    }
                    if interval.start() > id {
                        return false;
                    }
                }
                false
            })
            .count())
    }

    fn bonus(input: &str) -> Result<impl Display> {
        let (ranges, _) = Self::read_input(input)?;
        let mut total = 0u64;
        let mut current = ranges[0].clone();

        for r in ranges.into_iter().skip(1) {
            if r.start() <= current.end() {
                current = *current.start()..=(*current.end()).max(*r.end());
            } else {
                total += current.end() - current.start() + 1;
                current = r;
            }
        }
        total += current.end() - current.start() + 1;

        Ok(total)
    }
}
