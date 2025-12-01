use crate::tasks::TaskRun;
use anyhow::Result;
use std::fmt::Display;

pub struct Day01;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left(i32),
    Right(i32),
}

impl Day01 {
    fn read_lines(input: &str) -> Vec<Direction> {
        input
            .lines()
            .map(|line| {
                let mut it = line.chars();
                let ch = it.next().unwrap();
                let num = it.fold(0, |acc, ch| acc * 10 + ch.to_digit(10).unwrap()) as i32;
                if ch == 'L' {
                    Direction::Left(num)
                } else {
                    Direction::Right(num)
                }
            })
            .collect()
    }
}

impl TaskRun for Day01 {
    fn normal(input: &str) -> Result<impl Display> {
        let vault = Self::read_lines(input).into_iter().fold(
            (0, 50),
            |(mut count, mut pos), direction| {
                match direction {
                    Direction::Left(delta) => pos -= delta,
                    Direction::Right(delta) => pos += delta,
                };
                pos %= 100;
                if pos == 0 {
                    count += 1;
                }
                (count, pos)
            },
        );
        Ok(vault.0)
    }

    fn bonus(input: &str) -> Result<impl Display> {
        let vault = Self::read_lines(input).into_iter().fold(
            (0, 50),
            |(mut count, mut pos), direction| {
                match direction {
                    Direction::Left(delta) => {
                        pos += delta;
                        count += pos / 100;
                        pos %= 100;
                    }
                    Direction::Right(delta) => {
                        if pos == 0 {
                            count += delta / 100;
                        } else if delta >= pos {
                            count += ((delta - pos) / 100) + 1;
                        }
                        pos = (((pos - delta) % 100) + 100) % 100;
                    }
                }
                (count, pos)
            },
        );
        Ok(vault.0)
    }
}
