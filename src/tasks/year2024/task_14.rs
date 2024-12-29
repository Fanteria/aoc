use crate::tasks::TaskRun;
use std::{cmp::Ordering, fmt::Display, str::FromStr};

pub struct Task14;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn quadrant(&self, width: usize, height: usize) -> Option<usize> {
        match (
            self.x as i64 - (height / 2) as i64,
            self.y as i64 - (width / 2) as i64,
        ) {
            (x, y) if x > 0 && y > 0 => Some(1),
            (x, y) if x < 0 && y > 0 => Some(2),
            (x, y) if x < 0 && y < 0 => Some(3),
            (x, y) if x > 0 && y < 0 => Some(4),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Robot {
    position: Position,
    vector: Vector,
}

impl FromStr for Robot {
    type Err = u32; // TODO some meaninful

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vec) = s.split_once(' ').unwrap();
        let (y, x) = pos.strip_prefix("p=").unwrap().split_once(',').unwrap();
        let position = Position {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        };
        let (y, x) = vec.strip_prefix("v=").unwrap().split_once(',').unwrap();
        let vector = Vector {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        };
        Ok(Robot { position, vector })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Area {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

impl Area {
    fn move_roborts(&mut self, seconds: u32) {
        fn move_coordinate(position: u32, vector: i32, times: u32, max_size: usize) -> u32 {
            let to = position as i64 + (times as i64 * vector as i64);
            (((to % max_size as i64) + max_size as i64) % max_size as i64) as u32
        }
        self.robots.iter_mut().for_each(|robot| {
            robot.position.x =
                move_coordinate(robot.position.x, robot.vector.x, seconds, self.height);
            robot.position.y =
                move_coordinate(robot.position.y, robot.vector.y, seconds, self.width);
        })
    }
    fn robots_in_quadrants(&self) -> [usize; 4] {
        self.robots
            .iter()
            .filter_map(|robot| robot.position.quadrant(self.width, self.height))
            .fold([0_usize; 4], |mut counts, quadrant| {
                counts[quadrant - 1] += 1;
                counts
            })
    }

    fn read_area(input: &str) -> Self {
        Area {
            width: 101,
            height: 103,
            robots: input
                .lines()
                .map(|line| Robot::from_str(line).unwrap())
                .collect(),
        }
    }

    fn variances(&self) -> (f64, f64) {
        let mean =
            self.robots.iter().map(|r| r.position.x).sum::<u32>() as f64 / self.robots.len() as f64;
        let square_differences = self
            .robots
            .iter()
            .map(|r| (r.position.x as f64 - mean).powf(2.0))
            .sum::<f64>();
        let sample_variance_x = square_differences / (self.robots.len() - 1) as f64;

        let mean =
            self.robots.iter().map(|r| r.position.y).sum::<u32>() as f64 / self.robots.len() as f64;
        let square_differences = self
            .robots
            .iter()
            .map(|r| (r.position.y as f64 - mean).powf(2.0))
            .sum::<f64>();
        let sample_variance_y = square_differences / (self.robots.len() - 1) as f64;

        (sample_variance_x, sample_variance_y)
        // println!("Sample variance: x: {sample_variance_x}, y: {sample_variance_y}");
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height as u32 {
            for j in 0..self.width as u32 {
                if self
                    .robots
                    .iter()
                    .any(|robot| robot.position.x == i && robot.position.y == j)
                {
                    write!(f, "█")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl TaskRun for Task14 {
    fn normal(input: &str) -> impl Display {
        let mut area = Area::read_area(input);
        area.move_roborts(100);
        area.robots_in_quadrants().iter().product::<usize>()
    }

    fn bonus(input: &str) -> impl Display {
        let mut area = Area::read_area(input);
        let variances = (0..std::cmp::max(area.height, area.width))
            .map(|i| {
                area.move_roborts(1);
                let (x, y) = area.variances();
                (i, x as u32, y as u32)
            })
            .collect::<Vec<_>>();
        type FnCompare = fn(&(usize, u32, u32), &(usize, u32, u32)) -> Ordering;
        let min =
            |compare: FnCompare| variances.iter().min_by(|a, b| compare(a, b)).unwrap().0 as i32;

        let min_x_index = min(|(_, ax, _), (_, bx, _)| ax.cmp(bx));
        let min_y_index = min(|(_, _, ay), (_, _, by)| ay.cmp(by));

        fn modular_inverse(a: i32, m: i32) -> i32 {
            // Naive gcd
            fn gcd(a: i32, b: i32) -> (i32, i32) {
                if b == 0 {
                    (1, 0)
                } else {
                    let (x1, y1) = gcd(b, a % b);
                    (y1, x1 - (a / b) * y1)
                }
            }
            let (x, _) = gcd(a, m);
            (x % m + m) % m
        }

        let modulo = area.height as i32;
        let modulo_inverse = modular_inverse(area.width as i32, modulo);
        let k = ((min_x_index - min_y_index) * modulo_inverse) % modulo;
        let k = (k + modulo) % modulo;
        let seconds = k * (area.width as i32) + min_y_index;

        // let mut area = Area::read_area(input);
        // area.move_roborts(seconds as u32 + 1);
        // println!("{area}");

        seconds as usize + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(
            Robot {
                position: Position { x: 4, y: 2 },
                vector: Vector { x: -3, y: 2 },
            },
            Robot::from_str("p=2,4 v=2,-3").unwrap()
        );
    }

    #[test]
    fn movement() {
        let mut area = Area {
            width: 11,
            height: 7,
            robots: vec![Robot {
                position: Position { x: 4, y: 2 },
                vector: Vector { x: -3, y: 2 },
            }],
        };
        area.move_roborts(1);
        assert_eq!(
            area.robots,
            vec! { Robot { position: Position{ x: 1, y: 4}, vector: Vector{ x:-3, y:2}}}
        );
        area.move_roborts(1);
        assert_eq!(
            area.robots,
            vec! { Robot { position: Position{ x: 5, y: 6}, vector: Vector{ x:-3, y:2}}}
        );
        area.move_roborts(1);
        assert_eq!(
            area.robots,
            vec! { Robot { position: Position{ x: 2, y: 8}, vector: Vector{ x:-3, y:2}}}
        );
        area.move_roborts(1);
        assert_eq!(
            area.robots,
            vec! { Robot { position: Position{ x: 6, y: 10}, vector: Vector{ x:-3, y:2}}}
        );
        area.move_roborts(1);
        assert_eq!(
            area.robots,
            vec! { Robot { position: Position{ x: 3, y: 1}, vector: Vector{ x:-3, y:2}}}
        );

        let mut area = Area {
            width: 11,
            height: 7,
            robots: vec![Robot {
                position: Position { x: 4, y: 2 },
                vector: Vector { x: -3, y: 2 },
            }],
        };
        area.move_roborts(5);
        assert_eq!(
            area.robots,
            vec! { Robot { position: Position{ x: 3, y: 1}, vector: Vector{ x:-3, y:2}}}
        );
    }
}
