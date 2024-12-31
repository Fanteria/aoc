use crate::{
    tasks::TaskRun,
    utils::grid::{Direction, Grid, Point},
};
use ahash::AHashSet as HashSet;
use anyhow::Result;
use std::{fmt::Display, ops::AddAssign};

pub struct Day12;

#[derive(Debug, PartialEq, Eq)]
struct Garden {
    area: HashSet<Point>,
    fence: usize,
}

impl From<Point> for Garden {
    fn from(value: Point) -> Self {
        let mut area = HashSet::new();
        area.insert(value);
        Garden { fence: 0, area }
    }
}

impl AddAssign for Garden {
    fn add_assign(&mut self, rhs: Self) {
        rhs.area.into_iter().for_each(|item| {
            self.area.insert(item);
        });
        self.fence += rhs.fence;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct GardenFenceDisard {
    points: HashSet<Point>,
}

impl From<Point> for GardenFenceDisard {
    fn from(value: Point) -> Self {
        let mut points = HashSet::new();
        points.insert(value);
        GardenFenceDisard { points }
    }
}

impl AddAssign for GardenFenceDisard {
    fn add_assign(&mut self, rhs: Self) {
        rhs.points.into_iter().for_each(|item| {
            self.points.insert(item);
        });
    }
}

fn walkthrough_gardens<G>(grid: &Grid<char>, f: &impl Fn(&mut G, &Point)) -> Vec<G>
where
    G: From<Point> + AddAssign,
{
    let mut visited: HashSet<Point> = HashSet::new();
    grid.items_with_points()
        .filter_map(|(point, _)| {
            if !visited.contains(&point) {
                let garden = dfs(&point, grid, &mut visited, f);
                Some(garden)
            } else {
                None
            }
        })
        .collect()
}

fn dfs<G>(
    point: &Point,
    grid: &Grid<char>,
    visited: &mut HashSet<Point>,
    add_fence: &impl Fn(&mut G, &Point),
) -> G
where
    G: From<Point> + AddAssign,
{
    visited.insert(*point);
    let mut garden = G::from(*point);
    (0..8)
        .step_by(2)
        .for_each(|i| match point.adjacent(Direction::Up.clockwise(i), grid) {
            Some(up) if grid.get_at(&up) != grid.get_at(point) => add_fence(&mut garden, point),
            Some(up) if !visited.contains(&up) => {
                garden += dfs(&up, grid, visited, add_fence);
            }
            Some(up) if visited.contains(&up) => {}
            Some(_) => {
                unreachable!()
            }
            None => add_fence(&mut garden, point),
        });
    garden
}

impl TaskRun for Day12 {
    fn normal(input: &str) -> Result<impl Display> {
        let grid = Grid::<char>::from(input);
        Ok(
            walkthrough_gardens::<Garden>(&grid, &|garden, _| garden.fence += 1)
                .iter()
                .map(|g| g.fence * g.area.len())
                .sum::<usize>(),
        )
    }

    fn bonus(input: &str) -> Result<impl Display> {
        let grid = Grid::<char>::from(input);
        Ok(
            walkthrough_gardens::<GardenFenceDisard>(&grid, &|garden, point| {
                garden.points.insert(*point);
            })
            .iter()
            .map(|g| {
                let mut count = 0;
                g.points.iter().for_each(|point| {
                    Direction::Up.iter().step_by(2).for_each(|direction| {
                        // check outer corners
                        if !match (
                            point.adjacent(direction, &grid),
                            point.adjacent(direction.clockwise(2), &grid),
                        ) {
                            (Some(p), _) if g.points.contains(&p) => false,
                            (_, Some(p)) if g.points.contains(&p) => false,
                            _ => {
                                count += 1;
                                true
                            }
                        } {
                            // check inner corners
                            match (
                                point.adjacent(direction, &grid),
                                point.adjacent(direction.clockwise(1), &grid),
                            ) {
                                (Some(base), Some(diagonal))
                                    if !g.points.contains(&base)
                                        && g.points.contains(&diagonal) =>
                                {
                                    count += 1;
                                }
                                _ => {}
                            }
                        }
                    })
                });
                count * g.points.len()
            })
            .sum::<usize>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs_test() {
        const INPUT1: &str = "AABB\nAABB";
        let mut visited: HashSet<Point> = HashSet::new();
        let grid = Grid::<char>::from(INPUT1);
        let point = Point::new(0, 0, &grid).unwrap();
        let garden = dfs::<Garden>(&point, &grid, &mut visited, &|garden, _| garden.fence += 1);
        assert_eq!(garden.area.len(), 4);
        assert_eq!(garden.fence, 8);

        const INPUT2: &str = "AAAA\nBBCC";
        let mut visited: HashSet<Point> = HashSet::new();
        let grid = Grid::<char>::from(INPUT2);
        let point = Point::new(0, 0, &grid).unwrap();
        let garden = dfs::<Garden>(&point, &grid, &mut visited, &|garden, _| garden.fence += 1);
        assert_eq!(garden.area.len(), 4);
        assert_eq!(garden.fence, 10);
    }
}
