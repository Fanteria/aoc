use std::{fmt::Display, str::FromStr};

use crate::{
    tasks::TaskRun,
    utils::grid::{Direction, Grid, Point},
};

pub struct Task15;

#[derive(PartialEq, Eq, Clone, Copy)]
enum PlaceBox {
    Left,
    Right,
}

impl PlaceBox {
    fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn adjacent_direction(&self) -> Direction {
        match self {
            PlaceBox::Left => Direction::Right,
            PlaceBox::Right => Direction::Left,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Place {
    Barrier,
    Box(PlaceBox),
    Empty,
    Robot,
}

impl From<char> for Place {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Barrier,
            'O' => Self::Box(PlaceBox::Left),
            '.' => Self::Empty,
            '@' => Self::Robot,
            _ => unreachable!(),
        }
    }
}

impl Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Barrier => '#',
                Self::Box(PlaceBox::Left) => '[',
                Self::Box(PlaceBox::Right) => ']',
                Self::Empty => '.',
                Self::Robot => '@',
            }
        )
    }
}

fn read(input: &str) -> (Grid<Place>, Vec<Direction>) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    (
        Grid::<Place>::from_str(grid).unwrap(),
        moves
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| match c {
                '>' => Direction::Right,
                '<' => Direction::Left,
                '^' => Direction::Up,
                'v' => Direction::Down,
                _ => unreachable!(),
            })
            .collect(),
    )
}

fn gps(grid: Grid<Place>) -> usize {
    grid.items_with_points()
        .filter(|(_, place)| **place == Place::Box(PlaceBox::Left))
        .map(|(point, _)| point.get_x() * 100 + point.get_y())
        .sum()
}

fn can_move_wide_boxes(start: &Point, direction: Direction, grid: &mut Grid<Place>) -> bool {
    let ret = match grid.get_at(start) {
        Place::Barrier => false,
        Place::Box(PlaceBox::Left) => {
            let next = start.adjacent(direction, grid).unwrap();
            if direction.is_vertical() {
                let adjacent = next.adjacent(Direction::Right, grid).unwrap();
                can_move_wide_boxes(&next, direction, grid)
                    && can_move_wide_boxes(&adjacent, direction, grid)
            } else {
                can_move_wide_boxes(&next, direction, grid)
            }
        }
        Place::Box(PlaceBox::Right) => {
            let next = start.adjacent(direction, grid).unwrap();
            if direction.is_vertical() {
                let adjacent = next.adjacent(Direction::Left, grid).unwrap();
                can_move_wide_boxes(&next, direction, grid)
                    && can_move_wide_boxes(&adjacent, direction, grid)
            } else {
                can_move_wide_boxes(&next, direction, grid)
            }
        }
        Place::Empty => true,
        Place::Robot => unreachable!(),
    };
    ret
}

fn force_move_wide_boxes(start: &Point, direction: Direction, grid: &mut Grid<Place>) {
    if let Place::Box(box_part) = *grid.get_at(start) {
        let next = start.adjacent(direction, grid).unwrap();
        let adjacent_direction = box_part.adjacent_direction();
        if direction.is_vertical() {
            let adjacent = next.adjacent(adjacent_direction, grid).unwrap();
            force_move_wide_boxes(&next, direction, grid);
            force_move_wide_boxes(&adjacent, direction, grid);
            *grid.get_at_mut(&start.adjacent(adjacent_direction, grid).unwrap()) = Place::Empty;
            *grid.get_at_mut(&next) = Place::Box(box_part);
            *grid.get_at_mut(&adjacent) = Place::Box(box_part.opposite());
        } else {
            force_move_wide_boxes(&next, direction, grid);
            let adjacent = next.adjacent(adjacent_direction, grid).unwrap();
            *grid.get_at_mut(&next) = Place::Box(box_part);
            *grid.get_at_mut(&adjacent) = Place::Box(box_part.opposite());
        }
    }
}

fn robot_walkthrough(
    grid: &mut Grid<Place>,
    moves: Vec<Direction>,
    move_boxes: impl Fn(&Point, Direction, &mut Grid<Place>) -> bool,
) {
    let mut robot = grid.find(&Place::Robot).unwrap();
    moves.into_iter().for_each(|direction| {
        let next = robot.adjacent(direction, grid).unwrap();
        match grid.get_at(&next) {
            Place::Barrier => {}
            Place::Box(_) => {
                if move_boxes(&next, direction, grid) {
                    *grid.get_at_mut(&next) = Place::Robot;
                    *grid.get_at_mut(&robot) = Place::Empty;
                    robot = next;
                }
            }
            Place::Empty => {
                *grid.get_at_mut(&next) = Place::Robot;
                *grid.get_at_mut(&robot) = Place::Empty;
                robot = next;
            }
            Place::Robot => {
                unreachable!()
            }
        }
        // std::thread::sleep(std::time::Duration::from_millis(300));
        // println!("{grid}");
    });
}

impl TaskRun for Task15 {
    fn normal(input: &str) -> impl Display
    where
        Self: Sized,
    {
        fn move_boxes(start: &Point, direction: Direction, grid: &mut Grid<Place>) -> bool {
            match grid.get_at(start) {
                Place::Barrier => false,
                Place::Box(_) => {
                    let next = start.adjacent(direction, grid).unwrap();
                    let ret = move_boxes(&next, direction, grid);
                    if ret {
                        *grid.get_at_mut(&next) = Place::Box(PlaceBox::Left);
                    }
                    ret
                }
                Place::Empty => true,
                Place::Robot => unreachable!(),
            }
        }

        let (mut grid, moves) = read(input);
        robot_walkthrough(&mut grid, moves, move_boxes);
        gps(grid)
    }

    fn bonus(input: &str) -> impl Display
    where
        Self: Sized,
    {
        let (grid, moves) = read(input);
        let width = grid.get_width();
        let mut grid = Grid::<Place>::new(
            grid.consume_data()
                .into_iter()
                .flat_map(|place| match place {
                    Place::Barrier => [Place::Barrier, Place::Barrier],
                    Place::Box(PlaceBox::Left) => {
                        [Place::Box(PlaceBox::Left), Place::Box(PlaceBox::Right)]
                    }
                    Place::Empty => [Place::Empty, Place::Empty],
                    Place::Robot => [Place::Robot, Place::Empty],
                    _ => unreachable!(),
                })
                .collect(),
            width * 2,
        );
        robot_walkthrough(&mut grid, moves, |start, direction, grid| {
            if can_move_wide_boxes(start, direction, grid) {
                force_move_wide_boxes(start, direction, grid);
                true
            } else {
                false
            }
        });
        gps(grid)
    }
}
