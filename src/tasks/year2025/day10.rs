use crate::tasks::TaskRun;
use anyhow::{Context, Result};
use good_lp::{variable, Expression, ProblemVariables, Solution, SolverModel};
use itertools::Itertools;
use std::{fmt::Display, str::FromStr};

pub struct Day10;

#[derive(Debug)]
struct Machine {
    expected_leds: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joytage: Vec<u64>,
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (expected, s) = s.split_once(' ').context("missing expected")?;
        let expected_leds: Vec<_> = expected[1..expected.len() - 1]
            .chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => unreachable!(),
            })
            .collect();
        let mut it = s.split_whitespace().rev();
        let joytage = it.next().context("missing joytage")?;
        let joytage = joytage[1..joytage.len() - 1]
            .split(',')
            .map(|joy| joy.parse())
            .collect::<Result<Vec<u64>, _>>()?;
        let buttons = it
            .map(|button| {
                button[1..button.len() - 1]
                    .split(',')
                    .map(|switch| switch.parse())
                    .collect::<Result<Vec<usize>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            expected_leds,
            buttons,
            joytage,
        })
    }
}

impl Day10 {
    fn read_input(input: &str) -> Result<Vec<Machine>> {
        input.lines().map(Machine::from_str).collect()
    }
}

impl TaskRun for Day10 {
    fn normal(input: &str) -> Result<impl Display> {
        Ok(Self::read_input(input)?
            .into_iter()
            .map(|machine| {
                let mut count = 1;
                while !machine
                    .buttons
                    .iter()
                    .combinations(count)
                    .any(|button_combination| {
                        button_combination.iter().fold(
                            vec![false; machine.expected_leds.len()],
                            |mut leds, button| {
                                button.iter().for_each(|index| leds[*index] = !leds[*index]);
                                leds
                            },
                        ) == machine.expected_leds
                    })
                {
                    count += 1;
                }
                count
            })
            .sum::<usize>())
    }

    fn bonus(input: &str) -> Result<impl Display> {
        Ok(Self::read_input(input)?
            .into_iter()
            .map(|machine| {
                // Really do not like to use linear programming library, but do not know any suitable solution
                let mut vars = ProblemVariables::new();

                let button_presses: Vec<_> = machine
                    .buttons
                    .iter()
                    .map(|_| vars.add(variable().min(0).integer()))
                    .collect();

                let solution = machine
                    .buttons
                    .iter()
                    .enumerate()
                    .fold(
                        vec![
                            Expression::with_capacity(machine.buttons.len());
                            machine.joytage.len()
                        ],
                        |mut expressions, (i, button)| {
                            button
                                .iter()
                                .for_each(|x| expressions[*x] += button_presses[i]);
                            expressions
                        },
                    )
                    .into_iter()
                    .zip(machine.joytage)
                    .fold(
                        good_lp::highs(vars.minimise(button_presses.iter().sum::<Expression>())),
                        |mut problem, (e, j)| {
                            problem.add_constraint(e.eq(j as f64));
                            problem
                        },
                    )
                    .solve()
                    .unwrap();

                button_presses
                    .iter()
                    .map(|&v| solution.value(v))
                    .sum::<f64>() as usize
            })
            .sum::<usize>())
    }
}
