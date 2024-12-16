use std::{collections::{HashMap, HashSet}, fmt::Display, str::FromStr};

use crate::{
    tasks::TaskRun,
    utils::grid::{Direction, Grid, Point},
};

pub struct Task16;

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
    fn new(grid: &Grid<Cell>) -> Self {
        let point = grid.find(&Cell::Start).unwrap();
        Position {
            direction: Direction::Right,
            score: 0,
            tiles: vec![point.clone()],
            point,
        }
    }

    fn next(mut self, grid: &Grid<Cell>) -> impl Iterator<Item = Position> {
        self.tiles.push(self.point.clone());

        let get_next = |direction: Direction| -> Option<Position> {
            let point = self.point.adjacent(direction, grid).unwrap();
            if *grid.get_at(&point) == Cell::Wall {
                None
            } else {
                Some(Position {
                    point,
                    direction,
                    score: self.score + if direction == self.direction { 1 } else { 1001 },
                    tiles: self.tiles.clone()
                })
            }
        };

        [
            get_next(self.direction),
            get_next(self.direction.clockwise(2)),
            get_next(self.direction.counter_clockwise(2)),
        ]
        .into_iter()
        .filter_map(|p| p)
    }
}

impl TaskRun for Task16 {
    fn normal(input: &str) -> usize
    where
        Self: Sized,
    {
        let grid = Grid::<Cell>::from_str(input).unwrap();
        let mut points = vec![Position::new(&grid)];

        let mut solution: u32 = u32::MAX;
        let mut tiles: Vec<Point> = Vec::new();
        let mut crosroads: HashMap<Point, (u32, Direction)> = HashMap::new();
        let mut i = 0;
        while let Some(act) = points.pop() {
            points.extend(act.next(&grid).filter(|p| {
                if !crosroads.contains_key(&p.point) {
                    crosroads.insert(p.point.clone(), (p.score, p.direction));
                } else  {
                    let (score, direction) = crosroads.get_mut(&p.point).unwrap();
                    if p.score >= *score {
                        return false;
                    } else {
                        *score = p.score;
                    }
                }

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
            }));
        }
        solution as usize
    }

    fn bonus(input: &str) -> usize
    where
        Self: Sized,
    {
        let grid = Grid::<Cell>::from_str(input).unwrap();
        let mut points = vec![Position::new(&grid)];

        let mut solution: u32 = u32::MAX;
        let mut tiles: HashSet<Point> = HashSet::new();
        let mut crosroads: HashMap<Point, (u32, Direction)> = HashMap::new();
        while let Some(act) = points.pop() {
            points.extend(act.next(&grid).filter(|p| {

                // std::thread::sleep(std::time::Duration::from_millis(100));
                // println!("{p:?}");
                // let mut g = grid.clone();
                // for point in &p.tiles {
                //     *g.get_at_mut(point) = Cell::Start;
                // }
                // println!("{g}");

                if !crosroads.contains_key(&p.point) {
                    crosroads.insert(p.point.clone(), (p.score, p.direction));
                } else  {
                    let (score, direction) = crosroads.get_mut(&p.point).unwrap();
                    // println!("p.score: {}, score: {}, direction: {:?}", p.score, *score, p.direction);
                    if (p.score > *score && p.direction == *direction) || (p.score > *score + 1000) {
                        return false;
                    } else {
                        *score = p.score;
                    }
                }

                if *grid.get_at(&p.point) == Cell::End {
                    if p.score < solution {
                        solution = p.score;
                        tiles.clear();
                        tiles.extend(p.tiles.clone().into_iter());
                    } else if p.score == solution {
                        tiles.extend(p.tiles.clone().into_iter());
                    }
                    false
                } else {
                    true
                }
            }));
        }

        println!("{tiles:?}");
        let mut g = grid.clone();
        for point in &tiles {
            *g.get_at_mut(point) = Cell::Start;
        }
        println!("{g}");
        tiles.len() + 1
    }
}
