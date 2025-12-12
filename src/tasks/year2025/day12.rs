use crate::{tasks::TaskRun, utils::grid::Grid};
use anyhow::{Context, Result};
use std::fmt::Display;

pub struct Day12;

#[derive(PartialEq, Eq)]
enum ShapeCell {
    Filled,
    Empty,
}

impl From<char> for ShapeCell {
    fn from(value: char) -> Self {
        match value {
            '#' => ShapeCell::Filled,
            '.' => ShapeCell::Empty,
            _ => unreachable!(),
        }
    }
}

impl Display for ShapeCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ShapeCell::Filled => '#',
                ShapeCell::Empty => '.',
            }
        )
    }
}

type Shape = Grid<ShapeCell>;

#[derive(Debug)]
struct Place {
    width: usize,
    height: usize,
    shapes: Vec<usize>,
}

impl Day12 {
    fn read_input(input: &str) -> Result<(Vec<Shape>, Vec<Place>)> {
        let (shapes, places) = input.rsplit_once("\n\n").context("Missing places")?;
        let places: Vec<_> = places
            .lines()
            .map(|line| {
                let (dim, counts) = line.split_once(": ").context("Invalid place format")?;
                let (width, height) = dim.split_once('x').context("Invalid dimension format")?;
                Ok(Place {
                    width: width.parse().context("Cannot parse width")?,
                    height: height.parse().context("Cannot parse height")?,
                    shapes: counts
                        .split_whitespace()
                        .map(|times| Ok(times.parse()?))
                        .collect::<Result<Vec<_>>>()?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let shapes = shapes
            .split("\n\n")
            .map(|block| {
                Ok(Shape::from(
                    block.split_once('\n').context("Invalid shape format")?.1,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok((shapes, places))
    }
}

impl TaskRun for Day12 {
    fn normal(input: &str) -> Result<impl Display> {
        let (shapes, places) = Self::read_input(input)?;
        let raw_sizes = shapes
            .iter()
            .map(|shape| {
                shape
                    .items()
                    .filter(|cell| **cell == ShapeCell::Filled)
                    .count()
            })
            .collect::<Vec<_>>();

        let mut count = 0;
        for place in &places {
            let raw_space = place.height * place.width;
            let min_space = place
                .shapes
                .iter()
                .enumerate()
                .map(|(index, times)| times * raw_sizes[index])
                .sum::<usize>();
            if raw_space < min_space {
                continue;
            }
            let shape_count = place.shapes.iter().sum::<usize>();
            if shape_count <= (place.width / 3) * (place.height / 3) {
                count += 1;
                continue;
            }
            return Err(anyhow::anyhow!("This situation is too complicated and not needed for my input"));
        }
        println!("XXX");
        Ok(count)
    }

    fn bonus(_input: &str) -> Result<impl Display> {
        Ok("x-mas")
    }
}
