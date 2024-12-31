use crate::tasks::TaskRun;
use ahash::AHashMap as HashMap;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::fmt::Display;

pub struct Day24;

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Id<'a> {
    Other(&'a str),
    X(&'a str),
    Y(&'a str),
    Z(&'a str),
}

impl Display for Id<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Id::X(x) => x,
                Id::Y(y) => y,
                Id::Z(z) => z,
                Id::Other(o) => o,
            }
        )
    }
}

impl<'a> From<&'a str> for Id<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            value if value.starts_with("x") => Self::X(value),
            value if value.starts_with("y") => Self::Y(value),
            value if value.starts_with("z") => Self::Z(value),
            _ => Self::Other(value),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Expression<'a> {
    And(Id<'a>, Id<'a>),
    Or(Id<'a>, Id<'a>),
    Xor(Id<'a>, Id<'a>),
    Value(bool),
}

#[derive(Debug)]
struct Wiring<'a> {
    wiring: HashMap<Id<'a>, Expression<'a>>,
}

impl<'a> Wiring<'a> {
    fn get(&self, key: &Id) -> bool {
        match self.wiring.get(key).unwrap() {
            Expression::And(l, r) => self.get(l) && self.get(r),
            Expression::Or(l, r) => self.get(l) || self.get(r),
            Expression::Xor(l, r) => self.get(l) ^ self.get(r),
            Expression::Value(v) => *v,
        }
    }
}

impl<'a> TryFrom<&'a str> for Wiring<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        let (initial, expressions) = value.split_once("\n\n").context("Cannot split blocks")?;
        Ok(Self {
            wiring: initial
                .lines()
                .map(|line| {
                    let (key, value) =
                        line.split_once(":").context("Cannot split key and value")?;
                    Ok::<_, anyhow::Error>((
                        Id::from(key.trim()),
                        match value.trim() {
                            "1" => Expression::Value(true),
                            "0" => Expression::Value(false),
                            _ => unreachable!(),
                        },
                    ))
                })
                .process_results(|it| {
                    it.chain(expressions.lines().map(|line| {
                        let (expression, output) = line.split_once(" -> ").unwrap();
                        let output = Id::from(output);
                        let items = expression.split_whitespace().collect_vec();
                        let (left, right) = if items[0] > items[2] {
                            (Id::from(items[2]), Id::from(items[0]))
                        } else {
                            (Id::from(items[0]), Id::from(items[2]))
                        };
                        (
                            output,
                            match items[1] {
                                "AND" => Expression::And(left, right),
                                "OR" => Expression::Or(left, right),
                                "XOR" => Expression::Xor(left, right),
                                _ => unreachable!(),
                            },
                        )
                    }))
                    .collect()
                })?,
        })
    }
}

impl TaskRun for Day24 {
    fn normal(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        let wiring = Wiring::try_from(input)?;
        Ok(wiring
            .wiring
            .iter()
            .filter_map(|(k, _)| if let Id::Z(_) = k { Some(k) } else { None })
            .sorted()
            .enumerate()
            .map(|(index, id)| if wiring.get(id) { 1 << index } else { 0 })
            .sum::<usize>())
    }

    fn bonus(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        let wiring = Wiring::try_from(input)?;
        let last_z_bit = wiring
            .wiring
            .keys()
            .filter(|key| matches!(key, Id::Z(_)))
            .max()
            .unwrap();

        Ok(wiring
            .wiring
            .iter()
            .filter_map(|(wire, expression)| match (wire, expression) {
                (_, Expression::Value(_)) => None,
                (Id::Z("z00"), Expression::And(Id::X("x00"), Id::Y("y00"))) => None,
                (Id::Z(z), Expression::Or(_, _)) if Id::Z(z) == *last_z_bit => None,
                (Id::Z(_), Expression::And(_, _) | Expression::Or(_, _)) => Some(wire),
                (output, Expression::Xor(Id::X(l), Id::Y(r))) if *l != "x00" && *r != "y00" => {
                    if wiring.wiring.values().any(
                        |e| matches!(e, Expression::And(l, r) if *l == *output || *r == *output),
                    ) {
                        None
                    } else {
                        Some(output)
                    }
                }
                (_, Expression::Xor(Id::X(_), Id::Y(_))) => None,
                (Id::Z(_), Expression::Xor(_, _)) => None,
                (_, Expression::Xor(_, _)) => Some(wire),
                (output, Expression::And(l, r)) if *l != Id::X("x00") && *r != Id::Y("y00") => {
                    if wiring.wiring.values().any(
                        |e| matches!(e, Expression::Or(l, r) if *l == *output || *r == *output),
                    ) {
                        None
                    } else {
                        Some(output)
                    }
                }
                _ => None,
            })
            .sorted()
            .join(","))
    }
}
