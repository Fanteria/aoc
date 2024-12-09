use std::iter;

use crate::tasks::TaskRun;

pub struct Task09;

impl Task09 {
    fn normal_read(input: &str) -> Vec<Option<usize>> {
        input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
            .flat_map(|(i, num)| {
                if i % 2 == 0 {
                    iter::repeat(Some(i / 2)).take(num as usize)
                } else {
                    iter::repeat(None).take(num as usize)
                }
            })
            .collect()
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

    fn bonus_read(input: &str) -> Vec<(Option<usize>, u32)> {
        input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
            .flat_map(|(i, num)| {
                if i % 2 == 0 {
                    iter::repeat((Some(i / 2), num)).take(num as usize)
                } else {
                    iter::repeat((None, num)).take(num as usize)
                }
            })
            .collect()
    }

    fn bonus_sort(mut vec: Vec<(Option<usize>, u32)>) -> Vec<(Option<usize>, u32)> {
        let mut i = vec.len();
        while i > 0 {
            i -= 1;
            if let (Some(num), size) = vec[i] {
                if let Some(j) = vec[0..i]
                    .iter()
                    .position(|(item, space_size)| item.is_none() && size <= *space_size)
                {
                    let space_size = vec[j].1;
                    for k in 0..size as usize {
                        vec[j + k] = (Some(num), size);
                        vec[i - k] = (None, 0);
                    }
                    let new_size = space_size - size;
                    for k in 0..new_size as usize {
                        vec[j + (size as usize) + k] = (None, new_size);
                    }
                }
            }
        }
        vec
    }

    fn check_sum<T>(vec: &Vec<T>, f: impl Fn(&T) -> &Option<usize>) -> usize {
        vec.iter()
            .enumerate()
            .filter_map(|(index, num)| f(&num).and_then(|num| Some(index * num)))
            .sum()
    }
}

impl TaskRun for Task09 {
    fn normal(input: &str) -> usize {
        Self::check_sum(&Self::normal_sort(Self::normal_read(input)), |num| num)
    }

    fn bonus(input: &str) -> usize {
        Self::check_sum(&Self::bonus_sort(Self::bonus_read(input)), |(num, _)| num)
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
    fn normal() {
        let s = |v| to_string(v, |num| num);
        assert_eq!(&s(Task09::normal_read("12321")), "0..111..2");
        assert_eq!(
            &s(Task09::normal_sort(Task09::normal_read("12321"))),
            "02111...."
        );
    }

    #[test]
    fn bonus() {
        let s = |v| to_string(v, |(num, _)| num);
        assert_eq!(
            &s(Task09::bonus_sort(Task09::bonus_read("12321"))),
            "02.111..."
        );
    }
}
