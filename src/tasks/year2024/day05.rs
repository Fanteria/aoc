use crate::tasks::TaskRun;
use crate::utils::Parser;
use ahash::{AHashMap as HashMap, AHashSet as HashSet};
use anyhow::Result;
use std::fmt::Display;
use std::{cmp::Ordering, fmt::Debug, hash::Hash, str::FromStr};

pub struct Day05;

impl Day05 {
    fn read<T>(input: &str) -> (HashMap<T, HashSet<T>>, impl Iterator<Item = Vec<T>> + '_)
    where
        T: FromStr + std::cmp::Eq + Hash + Copy,
        <T as FromStr>::Err: Debug,
    {
        let (rules_input, pages_input) = input.split_once("\n\n").unwrap();
        let mut rules: HashMap<T, HashSet<T>> = HashMap::new();
        Parser::iter_array_sep::<T, 2>(rules_input, "|").for_each(|rule| {
            rules.entry(rule[0]).or_default().insert(rule[1]);
        });
        (rules, Parser::iter_vec_sep(pages_input, ","))
    }
}

impl TaskRun for Day05 {
    fn normal(input: &str) -> Result<impl Display> {
        let (rules, pages_list) = Self::read::<usize>(input);
        Ok(pages_list
            .filter_map(|pages| {
                if pages.is_sorted_by(|from, to| rules.get(from).is_some_and(|i| i.contains(to))) {
                    Some(pages[pages.len() / 2])
                } else {
                    None
                }
            })
            .sum::<usize>())
    }

    fn bonus(input: &str) -> Result<impl Display> {
        let (rules, pages_list) = Self::read::<usize>(input);
        Ok(pages_list
            .filter_map(|mut pages| {
                if !pages.is_sorted_by(|from, to| rules.get(from).is_some_and(|i| i.contains(to))) {
                    pages.sort_by(|from, to| {
                        if rules.get(from).is_some_and(|i| i.contains(to)) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    });
                    Some(pages[pages.len() / 2])
                } else {
                    None
                }
            })
            .sum::<usize>())
    }
}
