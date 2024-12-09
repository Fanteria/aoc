use crate::tasks::TaskRun;
use crate::utils::Parser;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    str::FromStr,
};

pub struct Task05;

impl Task05 {
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

impl TaskRun for Task05 {
    fn normal(input: &str) -> usize {
        let (rules, pages_list) = Self::read::<usize>(input);
        pages_list
            .filter_map(|pages| {
                if pages.is_sorted_by(|from, to| rules.get(from).is_some_and(|i| i.contains(to))) {
                    Some(pages[pages.len() / 2])
                } else {
                    None
                }
            })
            .sum()
    }

    fn bonus(input: &str) -> usize {
        let (rules, pages_list) = Self::read::<usize>(input);
        pages_list
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
            .sum()
    }
}
