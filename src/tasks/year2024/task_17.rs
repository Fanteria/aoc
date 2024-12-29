use crate::tasks::TaskRun;
use anyhow::{Context, Result};
use std::fmt::Display;

pub struct Task17;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

impl Registers {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Self { a, b, c }
    }

    fn combo(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

fn read(input: &str) -> (Registers, Vec<u8>) {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let mut registers = registers
        .lines()
        .map(|line| line["Register X: ".len()..].trim().parse().unwrap());
    (
        Registers {
            a: registers.next().unwrap(),
            b: registers.next().unwrap(),
            c: registers.next().unwrap(),
        },
        program
            .strip_prefix("Program: ")
            .unwrap()
            .trim()
            .split(',')
            .map(|c| c.parse::<u8>().unwrap())
            .collect::<Vec<_>>(),
    )
}

fn run(registers: &mut Registers, program: &[u8]) -> Vec<u8> {
    let mut pc = 0;
    let mut output = Vec::new();
    while pc < program.len() {
        match program[pc] {
            0 => registers.a /= 2_usize.pow(registers.combo(program[pc + 1]) as u32),
            1 => registers.b ^= program[pc + 1] as usize,
            2 => registers.b = registers.combo(program[pc + 1]) % 8,
            3 => {
                if registers.a != 0 {
                    pc = program[pc + 1] as usize;
                    continue;
                }
            }
            4 => registers.b ^= registers.c,
            5 => output.push((registers.combo(program[pc + 1]) % 8) as u8),
            6 => registers.b = registers.a / 2_usize.pow(registers.combo(program[pc + 1]) as u32),
            7 => registers.c = registers.a / 2_usize.pow(registers.combo(program[pc + 1]) as u32),
            _ => unreachable!(),
        }
        pc += 2;
    }

    output
}

impl TaskRun for Task17 {
    fn normal(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        let (mut registers, program) = read(input);
        Ok(run(&mut registers, &program)
            .into_iter()
            .fold(0_usize, |acc, num| (10 * acc) + num as usize))
    }

    fn bonus(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        let (regs, prog) = read(input);
        let mut res = Vec::new();
        let mut inputs = (0..8).collect::<Vec<_>>();
        (0..prog.len()).for_each(|i| {
            inputs = inputs
                .iter()
                .map(|num| (num, run(&mut Registers::new(*num, regs.b, regs.c), &prog)))
                .filter(|(_, output)| prog[prog.len() - 1 - i..] == *output)
                .flat_map(|(num, output)| {
                    if output.len() == prog.len() {
                        res.push(*num);
                    }
                    (0..8)
                        .map(|k| num * 8 + k)
                        .filter(|k| k / 8 == *num)
                        .collect::<Vec<_>>()
                })
                .collect();
        });

        res.into_iter().min().context("Initial value not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_program_test() {
        let mut registers = Registers::new(0, 0, 9);
        assert_eq!(run(&mut registers, &vec![2, 6]), vec![]);
        assert_eq!(registers, Registers::new(0, 1, 9));

        let mut registers = Registers::new(10, 0, 0);
        assert_eq!(run(&mut registers, &vec![5, 0, 5, 1, 5, 4]), vec![0, 1, 2]);
        assert_eq!(registers, Registers::new(10, 0, 0));

        let mut registers = Registers::new(2024, 0, 0);
        assert_eq!(
            run(&mut registers, &vec![0, 1, 5, 4, 3, 0]),
            vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]
        );
        assert_eq!(registers, Registers::new(0, 0, 0));

        let mut registers = Registers::new(0, 29, 0);
        assert_eq!(run(&mut registers, &vec![1, 7]), vec![]);
        assert_eq!(registers, Registers::new(0, 26, 0));

        let mut registers = Registers::new(0, 2024, 43690);
        assert_eq!(run(&mut registers, &vec![4, 0]), vec![]);
        assert_eq!(registers, Registers::new(0, 44354, 43690));
    }
}
