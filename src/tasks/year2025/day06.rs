use crate::tasks::TaskRun;
use anyhow::Result;
use std::fmt::Display;

pub struct Day06;

enum Operator {
    Add,
    Multiply,
}

impl TaskRun for Day06 {
    fn normal(input: &str) -> Result<impl Display> {
        let mut lines = input.lines().rev();
        let mut values: Vec<_> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|operator| match operator {
                "+" => (Operator::Add, 0u64),
                "*" => (Operator::Multiply, 1u64),
                _ => unreachable!(),
            })
            .collect();
        lines.for_each(|line| {
            line.split_whitespace()
                .enumerate()
                .for_each(|(index, value)| match values[index].0 {
                    Operator::Add => values[index].1 += value.parse::<u64>().unwrap(),
                    Operator::Multiply => values[index].1 *= value.parse::<u64>().unwrap(),
                })
        });
        Ok(values.into_iter().map(|(_, value)| value).sum::<u64>())
    }

    fn bonus(input: &str) -> Result<impl Display> {
        let mut lines: Vec<_> = input.lines().collect();
        let operators: Vec<_> = lines
            .pop()
            .unwrap()
            .char_indices()
            .filter_map(|(i, c)| match c {
                '+' => Some((i, Operator::Add)),
                '*' => Some((i, Operator::Multiply)),
                _ => None,
            })
            .collect();
        let mut sum = 0;
        let mut iter = operators.iter().peekable();
        while let Some((start, operator)) = iter.next() {
            let end = iter
                .peek()
                .map(|(end, _)| *end)
                .unwrap_or(lines.first().unwrap().len() + 1);
            // Create column numbers iterator
            let it = (*start..end - 1).map(|index| {
                // Parse column number
                let x = lines
                    .iter()
                    .filter_map(|line| line[index..=index].parse::<u64>().ok())
                    .fold(0, |acc, value| 10 * acc + value);
                print!("{x},");
                x
            });
            sum += match operator {
                Operator::Add => it.sum::<u64>(),
                Operator::Multiply => it.product::<u64>(),
            };
        }
        Ok(sum)
    }
}
