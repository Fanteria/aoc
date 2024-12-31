use crate::{
    tasks::TaskRun,
    utils::grid::{Direction, Grid, Point},
};
use ahash::{AHashMap as HashMap, AHashSet as HashSet};
use anyhow::{Context, Result};
use std::{collections::VecDeque, fmt::Display};

pub struct Day16;

#[derive(PartialEq, Eq, Clone)]
enum Cell {
    Wall,
    Empty,
    End,
    Start,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '.' => Self::Empty,
            'E' => Self::End,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Wall => '#',
                Cell::Empty => '.',
                Cell::End => 'E',
                Cell::Start => 'S',
            }
        )
    }
}

#[derive(Debug)]
struct Position {
    point: Point,
    direction: Direction,
    score: u32,
    tiles: Vec<Point>,
}

impl Position {
    fn new(grid: &Grid<Cell>) -> Result<Self> {
        let point = grid.find(&Cell::Start).context("Cannot find point")?;
        let mut tiles = Vec::with_capacity(500);
        tiles.push(point);
        Ok(Position {
            direction: Direction::Right,
            score: 0,
            tiles,
            point,
        })
    }

    fn next(
        mut self,
        grid: &Grid<Cell>,
        mut check_validity: impl FnMut(&Position) -> bool,
    ) -> Vec<Position> {
        self.tiles.push(self.point);

        let mut get_next = |direction: Direction| -> Option<Position> {
            let point = self.point.adjacent(direction, grid).unwrap();
            if *grid.get_at(&point) == Cell::Wall {
                None
            } else {
                let position = Position {
                    point,
                    direction,
                    score: self.score + if direction == self.direction { 1 } else { 1001 },
                    tiles: Vec::new(),
                };
                if check_validity(&position) {
                    // position.tiles = Vec::with_capacity(self.tiles.capacity());
                    // position.tiles.extend(&self.tiles);
                    Some(position)
                } else {
                    None
                }
            }
        };

        // print!(
        //     "{}, ",
        //     [
        //         get_next(self.direction),
        //         get_next(self.direction.clockwise(2)),
        //         get_next(self.direction.counter_clockwise(2)),
        //     ]
        //     .into_iter()
        //     .flatten()
        //     .count()
        // );

        let mut ret = [
            get_next(self.direction),
            get_next(self.direction.clockwise(2)),
            get_next(self.direction.counter_clockwise(2)),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        if ret.len() == 1 {
            ret[0].tiles = self.tiles;
            ret
        } else {
            ret.iter_mut().for_each(|position| {
                position.tiles = Vec::with_capacity(self.tiles.capacity());
                position.tiles.extend(&self.tiles);
            });
            ret
        }
    }
}

impl TaskRun for Day16 {
    fn normal(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        let grid = Grid::<Cell>::from(input);
        let mut points = VecDeque::new();
        points.push_front(Position::new(&grid)?);

        let mut solution: u32 = u32::MAX;
        let mut tiles: Vec<Point> = Vec::new();
        let mut crosroads: HashMap<Point, (u32, Direction)> = HashMap::new();
        let mut i = 0;
        while let Some(act) = points.pop_front() {
            points.extend(
                act.next(&grid, |p| {
                    if !crosroads.contains_key(&p.point) {
                        crosroads.insert(p.point, (p.score, p.direction));
                    } else {
                        let (score, _) = crosroads.get_mut(&p.point).unwrap();
                        if p.score >= *score {
                            return false;
                        } else {
                            *score = p.score;
                        }
                    }
                    true
                })
                .into_iter()
                .filter(|p| {
                    if p.score > solution {
                        false
                    } else if *grid.get_at(&p.point) == Cell::End {
                        if p.score < solution {
                            solution = p.score;
                            tiles = p.tiles.clone();
                        }
                        false
                    } else {
                        i += 1;
                        true
                    }
                }),
            );
        }
        Ok(solution)
    }

    fn bonus(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        let grid = Grid::<Cell>::from(input);
        let mut points = VecDeque::new();
        points.push_front(Position::new(&grid)?);
        let mut solution: u32 = u32::MAX;
        let mut tiles: HashSet<Point> = HashSet::new();
        let mut crosroads: HashMap<Point, u32> = HashMap::new();
        while let Some(act) = points.pop_front() {
            points.extend(
                act.next(&grid, |p| {
                    if !crosroads.contains_key(&p.point) {
                        crosroads.insert(p.point, p.score);
                    } else {
                        let score = crosroads.get_mut(&p.point).unwrap();
                        if p.score > *score + 1000 {
                            return false;
                        } else {
                            *score = p.score;
                        }
                    }
                    true
                })
                .into_iter()
                .filter(|p| match grid.get_at(&p.point) {
                    Cell::End if p.score < solution => {
                        solution = p.score;
                        tiles.clear();
                        tiles.extend(p.tiles.clone().into_iter());
                        false
                    }
                    Cell::End if p.score == solution => {
                        tiles.extend(p.tiles.clone().into_iter());
                        false
                    }
                    Cell::End => false,
                    _ => true,
                }),
            );
        }

        // println!("{tiles:?}");
        // let mut g = grid.clone();
        // for point in &tiles {
        //     *g.get_at_mut(point) = Cell::Start;
        // }
        // println!("{g}");
        Ok(tiles.len() + 1)
    }
}
