use crate::tasks::TaskRun;
use anyhow::{anyhow, Result};
use std::{fmt::Display, iter};

pub struct Day09;

#[derive(Debug, Clone)]
pub struct Disk {
    blocks: Vec<File>,
    spaces: [Vec<Space>; 10], // usize is index in `blocks`
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sorted = self
            .blocks
            .iter()
            .map(|b| (None, Some(b)))
            .chain(
                self.spaces
                    .iter()
                    .flat_map(|v| v.iter())
                    .map(|s| (Some(s), None)),
            )
            .map(|d| match d {
                (None, Some(b)) => (b.start_index, d),
                (Some(s), None) => (s.start_index, d),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));
        sorted.iter().try_for_each(|(_, d)| match d {
            (None, Some(b)) => write!(f, "{}", format!("{}", b.id).repeat(b.size)),
            (Some(s), None) => write!(f, "{}", ".".repeat(s.size)),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Clone)]
struct File {
    id: usize,
    size: usize,
    start_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Space {
    size: usize,
    start_index: usize,
}

impl Day09 {
    fn base_read(input: &str) -> Result<Vec<usize>> {
        input
            .trim()
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|n| n as usize)
                    .ok_or_else(|| anyhow!("'{}' is not digit", c))
            })
            .collect()
    }

    fn normal_read(input: &str) -> Result<Vec<Option<usize>>> {
        Ok(Self::base_read(input)?
            .into_iter()
            .enumerate()
            .flat_map(|(i, num)| {
                if i % 2 == 0 {
                    iter::repeat(Some(i / 2)).take(num)
                } else {
                    iter::repeat(None).take(num)
                }
            })
            .collect())
    }

    fn normal_sort(mut vec: Vec<Option<usize>>) -> Vec<Option<usize>> {
        let mut i = vec.len();
        let mut start = 0;
        while i > 0 && start < i {
            i -= 1;
            if let Some(num) = vec[i] {
                if let Some(j) = vec[start..i].iter().position(|item| item.is_none()) {
                    start += j;
                    vec[start] = Some(num);
                    vec[i] = None;
                }
            }
        }
        vec
    }

    fn bonus_read(input: &str) -> Result<Disk> {
        let mut spaces = std::array::from_fn(|_| Vec::new());
        let mut blocks = Vec::new();
        let mut start_index = 0;
        Self::base_read(input)?
            .into_iter()
            .enumerate()
            .for_each(|(i, size)| {
                if i % 2 == 0 {
                    blocks.push(File {
                        id: i / 2,
                        size,
                        start_index,
                    })
                } else {
                    spaces[size].push(Space { size, start_index })
                };
                start_index += size;
            });
        Ok(Disk { blocks, spaces })
    }

    fn bonus_sort(mut disk: Disk) -> Disk {
        let mut i = disk.blocks.len();
        while i > 0 {
            i -= 1;
            if let Some((j, k, _)) = disk
                .spaces
                .iter()
                .enumerate()
                .skip(disk.blocks[i].size)
                .filter_map(|(j, v)| {
                    Some((
                        j,
                        v.iter()
                            .enumerate()
                            .filter(|(_, s)| s.start_index < disk.blocks[i].start_index)
                            .min_by(|a, b| a.1.start_index.cmp(&b.1.start_index))?,
                    ))
                })
                .map(|(j, (k, w))| (j, k, w))
                .min_by(|a, b| a.2.start_index.cmp(&b.2.start_index))
            {
                let mut space = disk.spaces[j].remove(k);
                disk.spaces[disk.blocks[i].size].push(Space {
                    size: disk.blocks[i].size,
                    start_index: disk.blocks[i].start_index,
                });
                disk.blocks[i].start_index = space.start_index;
                space.start_index += disk.blocks[i].size;
                space.size -= disk.blocks[i].size;
                disk.spaces[space.size].push(space);
            }
        }
        disk
    }

    fn check_sum<T>(vec: &[T], f: impl Fn(&T) -> &Option<usize>) -> usize {
        vec.iter()
            .enumerate()
            .filter_map(|(index, num)| f(num).and_then(|num| Some(index * num)))
            .sum()
    }
}

impl TaskRun for Day09 {
    fn normal(input: &str) -> Result<impl Display> {
        Ok(Self::check_sum(
            &Self::normal_sort(Self::normal_read(input)?),
            |num| num,
        ))
    }

    fn bonus(input: &str) -> Result<impl Display> {
        Ok(Self::bonus_sort(Self::bonus_read(input)?)
            .blocks
            .iter()
            .map(|b| {
                (b.start_index..b.start_index + b.size)
                    .map(|index| index * b.id)
                    .sum::<usize>()
            })
            .sum::<usize>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_string<T>(vec: Vec<T>, f: impl Fn(&T) -> &Option<usize>) -> String {
        vec.iter()
            .map(|num| match f(num) {
                Some(num) => format!("{num}"),
                None => String::from("."),
            })
            .collect()
    }

    #[test]
    fn normal() -> Result<()> {
        let s = |v| to_string(v, |num| num);
        assert_eq!(&s(Day09::normal_read("12321")?), "0..111..2");
        assert_eq!(
            &s(Day09::normal_sort(Day09::normal_read("12321")?)),
            "02111...."
        );

        Ok(())
    }
}
