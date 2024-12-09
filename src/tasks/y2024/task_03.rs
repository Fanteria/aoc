use regex::Regex;

use crate::tasks::TaskRun;

pub struct Task03;

impl TaskRun for Task03 {
    fn normal(input: &str) -> usize {
        Regex::new(r"mul\((\d+),(\d+)\)")
            .unwrap()
            .captures_iter(input)
            .map(|c| c.extract::<2>().1)
            .map(|v| v[0].parse::<usize>().unwrap() * v[1].parse::<usize>().unwrap())
            .sum()
    }

    fn bonus(input: &str) -> usize {
        let mut do_multiply = true;
        let mut result = 0;
        Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")
            .unwrap()
            .captures_iter(input)
            .for_each(|c| match c.get(0).map(|m| m.as_str()) {
                Some("do()") => do_multiply = true,
                Some("don't()") => do_multiply = false,
                Some(_) if do_multiply => {
                    result += c.get(1).unwrap().as_str().parse::<usize>().unwrap()
                        * c.get(2).unwrap().as_str().parse::<usize>().unwrap();
                }
                _ => {}
            });
        result
    }
}
