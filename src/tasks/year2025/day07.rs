use crate::{
    tasks::TaskRun,
    utils::grid::Grid,
};
use anyhow::Result;
use std::fmt::Display;

pub struct Day07;

#[derive(PartialEq, Eq)]
enum Manifold {
    Start,
    Splitter { used: bool },
    Empty,
}

impl From<char> for Manifold {
    fn from(value: char) -> Self {
        match value {
            'S' => Manifold::Start,
            '^' => Manifold::Splitter { used: false },
            '.' => Manifold::Empty,
            _ => unreachable!(),
        }
    }
}

impl Display for Manifold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Manifold::Start => 'S',
                Manifold::Splitter { used: _ } => '^',
                Manifold::Empty => '.',
            }
        )
    }
}

impl TaskRun for Day07 {
    fn normal(input: &str) -> Result<impl Display> {
        let grid = Grid::<Manifold>::from(input);
        let (_, count) = grid.lines().fold(
            (vec![0u64; grid.get_width()], 0),
            |(mut buffer, mut count), line| {
                line.iter()
                    .enumerate()
                    .for_each(|(index, manifold)| match manifold {
                        Manifold::Start => buffer[index] = 1,
                        Manifold::Splitter { used: _ } => {
                            if buffer[index] != 0 {
                                count += 1;
                            }
                            buffer[index] = 0;
                            buffer[index - 1] = 1;
                            buffer[index + 1] = 1;
                        }
                        Manifold::Empty => {}
                    });
                (buffer, count)
            },
        );
        Ok(count)
    }

    fn bonus(input: &str) -> Result<impl Display> {
        let grid = Grid::<Manifold>::from(input);
        let buffer = grid
            .lines()
            .fold(vec![0u64; grid.get_width()], |mut buffer, line| {
                line.iter()
                    .enumerate()
                    .for_each(|(index, manifold)| match manifold {
                        Manifold::Start => buffer[index] = 1,
                        Manifold::Splitter { used: _ } => {
                            let val = buffer[index];
                            buffer[index] = 0;
                            buffer[index - 1] += val;
                            buffer[index + 1] += val;
                        }
                        Manifold::Empty => {}
                    });
                buffer
            });
        Ok(buffer.into_iter().sum::<u64>())
    }
}
