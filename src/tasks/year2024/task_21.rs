use crate::{
    tasks::TaskRun,
    utils::grid::{Direction, Path},
};
use anyhow::Result;
use std::fmt::Display;
use std::{collections::HashMap, iter};

pub struct Task21;

const BUTTON_POSSITIONS: [Path; 11] = [
    Path::new(1, 3),
    Path::new(0, 2),
    Path::new(1, 2),
    Path::new(2, 2),
    Path::new(0, 1),
    Path::new(1, 1),
    Path::new(2, 1),
    Path::new(0, 0),
    Path::new(1, 0),
    Path::new(2, 0),
    Path::new(2, 3),
];

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum RobotMove {
    Move(Direction),
    Push,
}

fn get_move(delta: i64, negative: Direction, positive: Direction) -> Vec<RobotMove> {
    let direction = match delta {
        delta if delta > 0 => positive,
        delta if delta < 0 => negative,
        _ => return vec![],
    };
    iter::repeat(RobotMove::Move(direction))
        .take(delta.unsigned_abs() as usize)
        .collect()
}

struct Code<'a>(&'a str);

impl<'a> Code<'a> {
    fn get_num(&self) -> usize {
        self.0.strip_suffix("A").unwrap().parse::<usize>().unwrap()
    }

    fn digits(&self) -> Vec<u32> {
        self.0.chars().map(|c| c.to_digit(16).unwrap()).collect()
    }
}

fn keypad_moves(code: Code) -> Vec<RobotMove> {
    let digits = code.digits();
    code.digits()
        .iter()
        .zip([0x0A].iter().chain(digits.iter()))
        .map(|(act, prev)| {
            (
                &BUTTON_POSSITIONS[*prev as usize],
                &BUTTON_POSSITIONS[*act as usize],
            )
        })
        .flat_map(|(prev, act)| {
            let Path { x, y } = *act - *prev;
            let horizontal = get_move(x, Direction::Left, Direction::Right);
            let vertical = get_move(y, Direction::Up, Direction::Down);
            if (vertical.first() == Some(&RobotMove::Move(Direction::Down))
                && prev.x == 0
                && act.y == 3)
                || (horizontal.first() == Some(&RobotMove::Move(Direction::Left))
                    && !(prev.y == 3 && act.x == 0))
            {
                horizontal.into_iter().chain(vertical)
            } else {
                vertical.into_iter().chain(horizontal)
            }
            .chain([RobotMove::Push])
        })
        .collect()
}

fn robot_moves() -> HashMap<(RobotMove, RobotMove), Vec<RobotMove>> {
    use Direction::*;
    use RobotMove::*;
    HashMap::from([
        ((Push, Push), vec![Push]),
        ((Push, Move(Up)), vec![Move(Left), Push]),
        ((Push, Move(Right)), vec![Move(Down), Push]),
        (
            (Push, Move(Left)),
            vec![Move(Down), Move(Left), Move(Left), Push],
        ),
        ((Push, Move(Down)), vec![Move(Left), Move(Down), Push]),
        ((Move(Up), Push), vec![Move(Right), Push]),
        ((Move(Up), Move(Up)), vec![Push]),
        ((Move(Up), Move(Right)), vec![Move(Down), Move(Right), Push]),
        ((Move(Up), Move(Left)), vec![Move(Down), Move(Left), Push]),
        ((Move(Up), Move(Down)), vec![Move(Down), Push]),
        ((Move(Down), Push), vec![Move(Up), Move(Right), Push]),
        ((Move(Down), Move(Up)), vec![Move(Up), Push]),
        ((Move(Down), Move(Right)), vec![Move(Right), Push]),
        ((Move(Down), Move(Left)), vec![Move(Left), Push]),
        ((Move(Down), Move(Down)), vec![Push]),
        (
            (Move(Left), Push),
            vec![Move(Right), Move(Right), Move(Up), Push],
        ),
        ((Move(Left), Move(Up)), vec![Move(Right), Move(Up), Push]),
        (
            (Move(Left), Move(Right)),
            vec![Move(Right), Move(Right), Push],
        ),
        ((Move(Left), Move(Left)), vec![Push]),
        ((Move(Left), Move(Down)), vec![Move(Right), Push]),
        ((Move(Right), Push), vec![Move(Up), Push]),
        ((Move(Right), Move(Up)), vec![Move(Left), Move(Up), Push]),
        ((Move(Right), Move(Right)), vec![Push]),
        (
            (Move(Right), Move(Left)),
            vec![Move(Left), Move(Left), Push],
        ),
        ((Move(Right), Move(Down)), vec![Move(Left), Push]),
    ])
}

fn sequence(
    p: RobotMove,
    c: RobotMove,
    depth: usize,
    l0: &HashMap<(RobotMove, RobotMove), Vec<RobotMove>>,
    known: &mut HashMap<(RobotMove, RobotMove, usize), usize>,
) -> usize {
    if known.contains_key(&(p, c, depth)) {
        return known[&(p, c, depth)];
    }
    let t = l0[&(p, c)].clone();
    let mut r = 0;
    let mut prev = RobotMove::Push;
    for cc in t.iter() {
        r += sequence(prev, *cc, depth - 1, l0, known);
        prev = *cc;
    }
    known.insert((p, c, depth), r);
    known[&(p, c, depth)]
}

fn get_sequences_len_sum(input: &str, depth: usize) -> usize {
    let moves = robot_moves();
    let get_sequence_len = |s: &Vec<RobotMove>| -> usize {
        let mut known = moves
            .iter()
            .map(|(k, v)| ((k.0, k.1, 0), v.len()))
            .collect();
        s.iter()
            .zip([RobotMove::Push].iter().chain(s.iter()))
            .map(|(act, prev)| sequence(*prev, *act, depth, &moves, &mut known))
            .sum()
    };

    input
        .lines()
        .map(Code)
        .map(|code| code.get_num() * get_sequence_len(&keypad_moves(code)))
        .sum()
}

impl TaskRun for Task21 {
    fn normal(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        Ok(get_sequences_len_sum(input, 1))
    }

    fn bonus(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        Ok(get_sequences_len_sum(input, 24))
    }
}
