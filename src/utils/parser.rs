use std::{fmt::Debug, str::FromStr};

pub struct Parser;

impl Parser {
    #[inline]
    #[must_use]
    pub fn iter_vec<'a, T>(lines: &'a str) -> impl Iterator<Item = Vec<T>> + 'a
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        lines.lines().map(move |line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
    }

    #[inline]
    #[must_use]
    pub fn iter_array<'a, T, const N: usize>(lines: &'a str) -> impl Iterator<Item = [T; N]> + 'a
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        lines.lines().map(move |line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<T>>()
                .try_into()
                .unwrap_or_else(|_| panic!("Expected exactly {} items.", N))
        })
    }

    #[inline]
    #[must_use]
    pub fn iter_vec_sep<'a, T>(lines: &'a str, sep: &'a str) -> impl Iterator<Item = Vec<T>> + 'a
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        lines.lines().map(move |line| {
            line.split(sep)
                .map(|num| num.trim().parse().unwrap())
                .collect()
        })
    }

    #[inline]
    #[must_use]
    pub fn iter_array_sep<'a, T, const N: usize>(lines: &'a str, sep: &'a str) -> impl Iterator<Item = [T; N]> + 'a
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        lines.lines().map(move |line| {
            line.split(sep)
                .map(|num| num.trim().parse().unwrap())
                .collect::<Vec<T>>()
                .try_into()
                .unwrap_or_else(|_| panic!("Expected exactly {} items.", N))
        })
    }
}
