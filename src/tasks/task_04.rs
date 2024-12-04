use super::TaskRun;
use std::io::{BufRead, BufReader, Read};

const XMAS: &str = "XMAS";
const SAMX: &str = "SAMX";

struct Data {
    data: Vec<String>,
}

impl Data {
    fn check_horizonal(&self, x: usize, y: usize) -> Option<()> {
        match self.data[x].get(y..y + XMAS.len())? {
            XMAS | SAMX => Some(()),
            _ => None,
        }
    }

    fn check_vertical(&self, x: usize, y: usize) -> Option<()> {
        match self
            .data
            .get(x..x + XMAS.len())?
            .iter()
            .map(|c| c.chars().nth(y).unwrap())
            .collect::<String>()
            .as_str()
        {
            XMAS | SAMX => Some(()),
            _ => None,
        }
    }

    fn check_diagonal_right(&self, x: usize, y: usize) -> Option<()> {
        match self
            .data
            .get(x..x + XMAS.len())?
            .iter()
            .enumerate()
            .map(|(index, row)| row.chars().nth(y + index))
            .collect::<Option<String>>()?
            .as_str()
        {
            XMAS | SAMX => Some(()),
            _ => None,
        }
    }

    fn check_diagonal_left(&self, x: usize, y: usize) -> Option<()> {
        match self
            .data
            .get(x..x + XMAS.len())?
            .iter()
            .enumerate()
            .filter(|(index, _)| y >= *index)
            .map(|(index, row)| row.chars().nth(y - index))
            .collect::<Option<String>>()?
            .as_str()
        {
            XMAS | SAMX => Some(()),
            _ => None,
        }
    }
}

pub struct Task04;

impl Task04 {
    fn read<R: Read>(reader: R) -> Data {
        Data {
            data: BufReader::new(reader)
                .lines()
                .map(|l| l.unwrap().chars().collect())
                .collect(),
        }
    }
}

impl TaskRun for Task04 {
    fn normal<R: Read>(reader: R) -> usize {
        let d = Task04::read(reader);
        (0..d.data.len())
            .map(|x| {
                (0..d.data[0].len())
                    .flat_map(|y| {
                        [
                            d.check_horizonal(x, y),
                            d.check_vertical(x, y),
                            d.check_diagonal_right(x, y),
                            d.check_diagonal_left(x, y),
                        ]
                    })
                    .filter(|v| v.is_some())
                    .count()
            })
            .sum()
    }

    fn bonus<R: Read>(reader: R) -> usize {
        let d = Task04::read(reader);
        (1..d.data.len() - 1)
            .map(|x| {
                (1..d.data[0].len() - 1)
                    .filter(|y| d.data[x].chars().nth(*y).unwrap() == 'A')
                    .filter(|y| {
                        matches!(
                            (
                                d.data[x + 1].chars().nth(*y + 1).unwrap(),
                                d.data[x - 1].chars().nth(*y - 1).unwrap(),
                            ),
                            ('M', 'S') | ('S', 'M')
                        )
                    })
                    .filter(|y| {
                        matches!(
                            (
                                d.data[x + 1].chars().nth(*y - 1).unwrap(),
                                d.data[x - 1].chars().nth(*y + 1).unwrap(),
                            ),
                            ('M', 'S') | ('S', 'M')
                        )
                    })
                    .count()
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
        let t = Task::new(4, TaskType::Normal);
        assert_eq!(
            t.run(t.get_example_in_path()),
            t.get_output(t.get_example_out_path())
        );
        assert_eq!(t.run(t.get_in_path()), t.get_output(t.get_out_path()));
    }

    #[test]
    fn bonus() {
        let t = Task::new(4, TaskType::Bonus);
        assert_eq!(
            t.run(t.get_example_in_path()),
            t.get_output(t.get_example_out_path())
        );
        assert_eq!(t.run(t.get_in_path()), t.get_output(t.get_out_path()));
    }

    #[bench]
    fn normal_bench(b: &mut Bencher) {
        let t = Task::new(4, TaskType::Normal);
        let input = t.get_input(t.get_in_path());
        b.iter(|| Task04::normal(input.as_bytes()))
    }

    #[bench]
    fn bonus_bench(b: &mut Bencher) {
        let t = Task::new(4, TaskType::Bonus);
        let input = t.get_input(t.get_in_path());
        b.iter(|| Task04::bonus(input.as_bytes()))
    }
}
