use crate::tasks::TaskRun;
use std::fmt::Display;
use std::str::FromStr;

pub struct Task13;

#[derive(Debug, PartialEq, Eq)]
pub struct Button {
    x: i64,
    y: i64,
}

impl FromStr for Button {
    type Err = u32; // TODO some meaningful

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let button = s
            .get("Button A: X+".len()..)
            .unwrap()
            .split_once(", Y+")
            .unwrap();
        Ok(Button {
            x: button.0.parse().unwrap(),
            y: button.1.parse().unwrap(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Solution {
    a: i64,
    b: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Automat {
    a: Button,
    b: Button,
    x: i64,
    y: i64,
}

impl Automat {
    fn solve(&self) -> Option<Solution> {
        let denom = self.a.y * self.b.x - self.a.x * self.b.y;

        let num_a = self.y * self.b.x - self.x * self.b.y;
        if num_a % denom != 0 {
            return None;
        }

        let num_b = self.y * self.a.x - self.x * self.a.y;
        if num_b % -denom != 0 {
            return None;
        }

        Some(Solution {
            a: num_a / denom,
            b: num_b / -denom,
        })
    }
}

impl FromStr for Automat {
    type Err = i32; // TOD some meaningful

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let a = Button::from_str(lines.next().unwrap()).unwrap();
        let b = Button::from_str(lines.next().unwrap()).unwrap();
        let price = lines
            .next()
            .unwrap()
            .strip_prefix("Prize: X=")
            .unwrap()
            .split_once(", Y=")
            .unwrap();
        Ok(Self {
            a,
            b,
            x: price.0.parse().unwrap(),
            y: price.1.parse().unwrap(),
        })
    }
}

impl TaskRun for Task13 {
    fn normal(input: &str) -> impl Display {
        input
            .split("\n\n")
            .filter_map(|s| Automat::from_str(s).unwrap().solve())
            .map(|solution| solution.a as usize * 3 + solution.b as usize)
            .sum::<usize>()
    }

    fn bonus(input: &str) -> impl Display {
        input
            .split("\n\n")
            .filter_map(|s| {
                let mut automat = Automat::from_str(s).unwrap();
                automat.x += 10_000_000_000_000;
                automat.y += 10_000_000_000_000;
                automat.solve()
            })
            .map(|solution| solution.a as usize * 3 + solution.b as usize)
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        assert_eq!(
            Automat {
                a: Button { x: 94, y: 34 },
                b: Button { x: 22, y: 67 },
                x: 8400,
                y: 5400,
            }
            .solve(),
            Some(Solution { a: 80, b: 40 })
        );
    }
}
