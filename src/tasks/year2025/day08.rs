use crate::tasks::TaskRun;
use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashMap, fmt::Display, str::FromStr};

pub struct Day08;

#[derive(Debug)]
struct Vector {
    x: u64,
    y: u64,
    z: u64,
}

impl FromStr for Vector {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let [x, y, z] = s
            .splitn(3, ',')
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|v| anyhow::anyhow!("expected 3 elements, got {:?}", v))?;
        Ok(Vector { x, y, z })
    }
}

#[derive(Debug)]
struct Distance {
    from: usize,
    to: usize,
    distance: u64,
}

impl Vector {
    fn distance_square(&self, other: &Self) -> u64 {
        let x = self.x.abs_diff(other.x);
        let y = self.y.abs_diff(other.y);
        let z = self.z.abs_diff(other.z);
        x * x + y * y + z * z
    }
}

impl Day08 {
    fn read_input(input: &str) -> Result<(Vec<Vector>, Vec<Distance>)> {
        let vectors = input
            .lines()
            .map(Vector::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        let len = vectors.len();
        let distances: Vec<_> = (0..len)
            .flat_map(|i| {
                (i + 1..len)
                    .map(|j| Distance {
                        from: i,
                        to: j,
                        distance: vectors[i].distance_square(&vectors[j]),
                    })
                    .collect::<Vec<_>>()
            })
            .sorted_by_key(|distance| distance.distance)
            .collect();
        Ok((vectors, distances))
    }
}

impl TaskRun for Day08 {
    fn normal(input: &str) -> Result<impl Display> {
        let (_, distances) = Self::read_input(input)?;

        let mut circuits: Vec<Vec<usize>> = Vec::new();
        let mut circuits_lookup: HashMap<usize, usize> = HashMap::new();
        distances.iter().take(1000).for_each(|d| {
            match (circuits_lookup.get(&d.from), circuits_lookup.get(&d.to)) {
                (None, None) => {
                    circuits.push(vec![d.from, d.to]);
                    circuits_lookup.insert(d.from, circuits.len() - 1);
                    circuits_lookup.insert(d.to, circuits.len() - 1);
                }
                (None, Some(index)) => {
                    circuits[*index].push(d.from);
                    circuits_lookup.insert(d.from, *index);
                }
                (Some(index), None) => {
                    circuits[*index].push(d.to);
                    circuits_lookup.insert(d.to, *index);
                }
                (Some(from_index), Some(to_index)) if from_index != to_index => {
                    let from_index = *from_index;
                    // Connect two already existing circuits
                    let mut to_circuit = vec![];
                    to_circuit.append(&mut circuits[*to_index]);
                    to_circuit.iter().for_each(|index| {
                        *circuits_lookup.get_mut(index).unwrap() = from_index;
                    });
                    circuits[from_index].append(&mut to_circuit);
                }
                _ => { /* Skip, already added */ }
            };
        });
        Ok(circuits
            .iter()
            .map(|circuit| circuit.len())
            .sorted()
            .rev()
            .take(3)
            .product::<usize>())
    }

    fn bonus(input: &str) -> Result<impl Display> {
        let (vectors, distances) = Self::read_input(input)?;

        let mut circuits: Vec<Vec<usize>> = Vec::new();
        let mut circuits_lookup: HashMap<usize, usize> = HashMap::new();
        let mut last = (0, 0);
        distances.iter().for_each(|d| {
            match (circuits_lookup.get(&d.from), circuits_lookup.get(&d.to)) {
                (None, None) => {
                    circuits.push(vec![d.from, d.to]);
                    circuits_lookup.insert(d.from, circuits.len() - 1);
                    circuits_lookup.insert(d.to, circuits.len() - 1);
                    last = (d.from, d.to)
                }
                (None, Some(index)) => {
                    circuits[*index].push(d.from);
                    circuits_lookup.insert(d.from, *index);
                    last = (d.from, d.to)
                }
                (Some(index), None) => {
                    circuits[*index].push(d.to);
                    circuits_lookup.insert(d.to, *index);
                    last = (d.from, d.to)
                }
                (Some(from_index), Some(to_index)) if from_index != to_index => {
                    let from_index = *from_index;
                    // Connect two already existing circuits
                    let mut to_circuit = vec![];
                    to_circuit.append(&mut circuits[*to_index]);
                    to_circuit.iter().for_each(|index| {
                        *circuits_lookup.get_mut(index).unwrap() = from_index;
                    });
                    circuits[from_index].append(&mut to_circuit);
                    last = (d.from, d.to)
                }
                _ => { /* Skip, already added */ }
            };
        });
        Ok(vectors[last.0].x * vectors[last.1].x)
    }
}
