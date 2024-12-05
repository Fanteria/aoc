use crate::utils::Parser;

use super::TaskRun;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::FromStr,
    hash::Hash,
};

pub struct Task05;

impl Task05 {
    fn read<'a, T>(input: &'a str) -> (HashMap<T, HashSet<T>>, impl Iterator<Item = Vec<T>> + 'a)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::{Task, TaskType};
    use test::Bencher;

    #[test]
    fn normal() {
        let t = Task::new(5, TaskType::Normal);
        assert_eq!(
            t.run(t.get_example_in_path()),
            t.get_output(t.get_example_out_path())
        );
        assert_eq!(t.run(t.get_in_path()), t.get_output(t.get_out_path()));
    }

    #[test]
    fn bonus() {
        let t = Task::new(5, TaskType::Bonus);
        assert_eq!(
            t.run(t.get_example_in_path()),
            t.get_output(t.get_example_out_path())
        );
        assert_eq!(t.run(t.get_in_path()), t.get_output(t.get_out_path()));
    }

    #[bench]
    fn normal_bench(b: &mut Bencher) {
        let t = Task::new(5, TaskType::Normal);
        let input = Task::get_input(t.get_in_path());
        b.iter(|| Task05::normal(&input))
    }

    #[bench]
    fn bonus_bench(b: &mut Bencher) {
        let t = Task::new(5, TaskType::Bonus);
        let input = Task::get_input(t.get_in_path());
        b.iter(|| Task05::bonus(&input))
    }
}
