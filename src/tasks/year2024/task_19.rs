use crate::{tasks::TaskRun, utils::Parser};
use ahash::AHashMap as HashMap;
use anyhow::Result;
use itertools::Itertools;
use std::fmt::Display;

pub struct Task19;

#[derive(Default)]
struct Trie {
    is_leaf: bool,
    child: HashMap<char, Trie>,
}

impl Trie {
    fn add(&mut self, s: &[char]) {
        match s {
            [] => self.is_leaf = true,
            [h, t @ ..] => {
                // Use entry if possible
                match self.child.get_mut(h) {
                    Some(trie) => trie.add(t),
                    None => {
                        let mut new_trie = Trie::default();
                        new_trie.add(t);
                        self.child.insert(*h, new_trie);
                    }
                }
            }
        }
    }

    fn new(s: &[String]) -> Self {
        let mut trie = Self::default();
        s.iter()
            .for_each(|pattern| trie.add(pattern.chars().collect::<Vec<_>>().as_slice()));
        trie
    }

    fn possible_next(&self, pattern: &[char], depth: usize) -> Option<Vec<usize>> {
        match (pattern, self.is_leaf) {
            ([], false) => None,
            ([], true) => Some(vec![depth]),
            ([h, t @ ..], false) => self.child.get(h)?.possible_next(t, depth + 1),
            ([h, t @ ..], true) => Some(
                self.child
                    .get(h)
                    .and_then(|child| child.possible_next(t, depth + 1))
                    .map_or(vec![depth], |mut nexts| {
                        nexts.push(depth);
                        nexts
                    }),
            ),
        }
    }

    fn count_possible_designs(&self, pattern: &[char]) -> usize {
        *(0..pattern.len() + 1)
            .filter_map(|i| Some((i, self.possible_next(&pattern[i..], i)?)))
            .flat_map(|(i, vec)| vec.into_iter().map(move |next| (i, next)))
            .fold(
                [vec![1], vec![0; pattern.len()]].concat(),
                |mut counts, (i, next)| {
                    counts[next] += counts[i];
                    counts
                },
            )
            .last()
            .unwrap()
    }
}

fn possible_design(towels: &HashMap<char, Vec<String>>, design: &str) -> bool {
    if design.is_empty() {
        return true;
    }
    if let Some(towels_list) = towels.get(&design.chars().next().unwrap()) {
        towels_list.iter().any(|towel| {
            design.starts_with(towel) && possible_design(towels, design.get(towel.len()..).unwrap())
        })
    } else {
        false
    }
}

impl TaskRun for Task19 {
    fn normal(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        let (towels_list, designs) = input.split_once("\n\n").unwrap();
        let mut towels: HashMap<char, Vec<String>> = HashMap::new();
        Parser::iter_line_sep::<String>(towels_list, ",").for_each(|towel| {
            towels
                .entry(towel.chars().next().unwrap())
                .or_default()
                .push(towel)
        });
        Ok(designs
            .lines()
            .filter(|design| possible_design(&towels, design))
            // .map(|design| println!("{design}"))
            .count())
    }

    fn bonus(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        let (towels_list, designs) = input.split_once("\n\n").unwrap();
        let trie = Trie::new(
            Parser::iter_line_sep::<String>(towels_list, ",")
                .collect::<Vec<_>>()
                .as_slice(),
        );
        Ok(designs
            .lines()
            .map(|design| trie.count_possible_designs(design.chars().collect_vec().as_slice()))
            .sum::<usize>())
    }
}
