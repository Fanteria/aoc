use crate::tasks::TaskRun;
use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::{fmt::Display, str::FromStr};

pub struct Day13;

#[derive(Debug, PartialEq, Eq)]
pub struct Button {
    x: i64,
    y: i64,
}

impl FromStr for Button {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let button = s
            .get("Button A: X+".len()..)
            .with_context(|| format!("Cannot get prefix from '{}'", s))?
            .split_once(", Y+")
            .with_context(|| format!("Cannot split to X and Y '{}'", s))?;
        Ok(Button {
            x: button.0.parse()?,
            y: button.1.parse()?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Solution {
    a: i64,
    b: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Automat {
    a: Button,
    b: Button,
    x: i64,
    y: i64,
}

impl Automat {
    fn solve(&self) -> Option<Solution> {
        let denom = self.a.y * self.b.x - self.a.x * self.b.y;

        let num_a = self.y * self.b.x - self.x * self.b.y;
        if num_a % denom != 0 {
            return None;
        }

        let num_b = self.y * self.a.x - self.x * self.a.y;
        if num_b % -denom != 0 {
            return None;
        }

        Some(Solution {
            a: num_a / denom,
            b: num_b / -denom,
        })
    }
}

impl FromStr for Automat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: [&str; 3] = s
            .lines()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|e| anyhow!("Cannot convert {:?} to automat.", e))?;
        let (x, y) = lines[2]
            .strip_prefix("Prize: X=")
            .with_context(|| format!("Prize preffix is wrong '{}'", lines[2]))?
            .split_once(", Y=")
            .with_context(|| format!("Cannot split X and Y of price '{}'", lines[2]))?;
        Ok(Self {
            a: Button::from_str(lines[0])?,
            b: Button::from_str(lines[1])?,
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

impl TaskRun for Day13 {
    fn normal(input: &str) -> Result<impl Display> {
        input
            .split("\n\n")
            .map(Automat::from_str)
            .process_results(|it| {
                it.filter_map(|automat| automat.solve())
                    .map(|solution| solution.a as usize * 3 + solution.b as usize)
                    .sum::<usize>()
            })
    }

    fn bonus(input: &str) -> Result<impl Display> {
        input
            .split("\n\n")
            .map(Automat::from_str)
            .process_results(|it| {
                it.filter_map(|mut automat| {
                    automat.x += 10_000_000_000_000;
                    automat.y += 10_000_000_000_000;
                    automat.solve()
                })
                .map(|solution| solution.a as usize * 3 + solution.b as usize)
                .sum::<usize>()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        assert_eq!(
            Automat {
                a: Button { x: 94, y: 34 },
                b: Button { x: 22, y: 67 },
                x: 8400,
                y: 5400,
            }
            .solve(),
            Some(Solution { a: 80, b: 40 })
        );
    }
}
