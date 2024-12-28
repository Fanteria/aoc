use std::{fmt::Debug, str::FromStr};

pub struct Parser;

impl Parser {
    #[inline]
    pub fn iter_vec<T>(lines: &str) -> impl Iterator<Item = Vec<T>> + '_
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
    pub fn iter_array<T, const N: usize>(lines: &str) -> impl Iterator<Item = [T; N]> + '_
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
    pub fn iter_line_sep<'a, T>(line: &'a str, sep: &'a str) -> impl Iterator<Item = T> + 'a
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        line.split(sep).map(|num| num.trim().parse().unwrap())
    }

    #[inline]
    pub fn iter_vec_sep<'a, T>(lines: &'a str, sep: &'a str) -> impl Iterator<Item = Vec<T>> + 'a
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        lines
            .lines()
            .map(move |line| Self::iter_line_sep(line, sep).collect())
    }

    #[inline]
    pub fn iter_array_sep<'a, T, const N: usize>(
        lines: &'a str,
        sep: &'a str,
    ) -> impl Iterator<Item = [T; N]> + 'a
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
