use anyhow::{Context, Result};
use itertools::Itertools;
use rayon::iter::*;
use std::fmt::Display;

use crate::tasks::TaskRun;

pub struct Day22;

fn next_secret(mut secret: usize) -> usize {
    const MOD: usize = 16777216;
    secret = ((secret << 6) ^ secret) % MOD;
    secret = ((secret >> 5) ^ secret) % MOD;
    secret = ((secret * 2048) ^ secret) % MOD;
    secret
}

impl TaskRun for Day22 {
    fn normal(input: &str) -> anyhow::Result<impl Display>
    where
        Self: Sized,
    {
        Ok(input
            .lines()
            .map(|line| line.parse::<usize>())
            .process_results(|it| {
                it.par_bridge()
                    .map(|secret| (0..2000).fold(secret, |act_secret, _| next_secret(act_secret)))
                    .sum::<usize>()
            })?)
    }

    fn bonus(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        // Map also works, but is cca 6 times slower.
        let mut map = [0; 130_000];

        input
            .lines()
            .map(|line| line.parse::<i32>())
            .process_results(|it| {
                it.for_each(|mut secret| {
                    let mut seen = [false; 130_000];
                    let mut deltas = vec![];
                    let mut price = secret % 10;

                    (0..2000).for_each(|_| {
                        secret = next_secret(secret as usize) as i32;
                        let new_price = secret % 10;
                        deltas.push(new_price - price);
                        if let Some(i) = deltas.iter().rev().next_tuple().map(|(a, b, c, d)| {
                            ((a + 9) * 19_i32.pow(3)
                                + (b + 9) * 19_i32.pow(2)
                                + (c + 9) * 19_i32
                                + (d + 9)) as usize
                        }) {
                            if !seen[i] {
                                seen[i] = true;
                                map[i] += new_price;
                            }
                        }
                        price = new_price;
                    });
                })
            })?;

        map.into_iter().max().context("Cannot get bananas")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_secret_test() {
        assert_eq!(403653, next_secret(3));
        assert_eq!(4337993, next_secret(403653));
    }
}
