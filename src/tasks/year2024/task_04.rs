use crate::tasks::TaskRun;
use itertools::Itertools;
use std::fmt::Display;

const XMAS: &str = "XMAS";
const SAMX: &str = "SAMX";

struct Data {
    data: Vec<String>,
}

impl Data {
    fn check_horizonal(&self, x: usize, y: usize) -> Option<()> {
        match self.data[x].get(y..y + XMAS.len())? {
            XMAS | SAMX => Some(()),
            _ => None,
        }
    }

    fn check_vertical(&self, x: usize, y: usize) -> Option<()> {
        match self
            .data
            .get(x..x + XMAS.len())?
            .iter()
            .map(|c| c.chars().nth(y).unwrap())
            .collect::<String>()
            .as_str()
        {
            XMAS | SAMX => Some(()),
            _ => None,
        }
    }

    fn check_diagonal_right(&self, x: usize, y: usize) -> Option<()> {
        match self
            .data
            .get(x..x + XMAS.len())?
            .iter()
            .enumerate()
            .map(|(index, row)| row.chars().nth(y + index))
            .collect::<Option<String>>()?
            .as_str()
        {
            XMAS | SAMX => Some(()),
            _ => None,
        }
    }

    fn check_diagonal_left(&self, x: usize, y: usize) -> Option<()> {
        match self
            .data
            .get(x..x + XMAS.len())?
            .iter()
            .enumerate()
            .filter(|(index, _)| y >= *index)
            .map(|(index, row)| row.chars().nth(y - index))
            .collect::<Option<String>>()?
            .as_str()
        {
            XMAS | SAMX => Some(()),
            _ => None,
        }
    }
}

pub struct Task04;

impl Task04 {
    fn read(input: &str) -> Data {
        Data {
            data: input.lines().map(|l| l.to_owned()).collect(),
        }
    }
}

impl TaskRun for Task04 {
    fn normal(input: &str) -> impl Display {
        let d = Task04::read(input);

        Itertools::cartesian_product(0..d.data.len(), 0..d.data[0].len())
            .flat_map(|(x, y)| {
                [
                    d.check_horizonal(x, y),
                    d.check_vertical(x, y),
                    d.check_diagonal_right(x, y),
                    d.check_diagonal_left(x, y),
                ]
            })
            .filter(|v| v.is_some())
            .count()
    }

    fn bonus(input: &str) -> impl Display {
        let d = Task04::read(input);
        Itertools::cartesian_product(1..d.data.len() - 1, 1..d.data[0].len() - 1)
            .filter(|(x, y)| d.data[*x].chars().nth(*y).unwrap() == 'A')
            .filter(|(x, y)| {
                matches!(
                    (
                        d.data[*x + 1].chars().nth(*y + 1).unwrap(),
                        d.data[*x - 1].chars().nth(*y - 1).unwrap(),
                    ),
                    ('M', 'S') | ('S', 'M')
                )
            })
            .filter(|(x, y)| {
                matches!(
                    (
                        d.data[x + 1].chars().nth(*y - 1).unwrap(),
                        d.data[x - 1].chars().nth(*y + 1).unwrap(),
                    ),
                    ('M', 'S') | ('S', 'M')
                )
            })
            .count()
    }
}
