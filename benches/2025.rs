#![feature(test)]
#![feature(iter_next_chunk)]

extern crate test;

use anyhow::Result;
use aoc::{
    files::{Files, FilesType},
    tasks::{Task, TaskType},
};
use test::Bencher;

fn prepare(task_num: u8, task_type: TaskType) -> Result<(Task, String)> {
    let task = Task::new(task_num, task_type, 2025);
    let input = Files::from_env()?.get_input(FilesType::Task, &task)?;
    Ok((task, input))
}

#[bench]
fn day01_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(1, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day01_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(1, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day02_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(2, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day02_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(2, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day03_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(3, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day03_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(3, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day04_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(4, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day04_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(4, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day05_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(5, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day05_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(5, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day06_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(6, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day06_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(6, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day07_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(7, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day07_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(7, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day08_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(8, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day08_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(8, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day09_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(9, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day09_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(9, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

// #[bench]
// fn day10_normal(b: &mut Bencher) -> Result<()> {
//     let (t, input) = prepare(10, TaskType::Normal)?;
//     b.iter(|| t.run(&input));
//     Ok(())
// }
//
// #[bench]
// fn day10_bonus(b: &mut Bencher) -> Result<()> {
//     let (t, input) = prepare(10, TaskType::Bonus)?;
//     b.iter(|| t.run(&input));
//     Ok(())
// }
//
// #[bench]
// fn day11_normal(b: &mut Bencher) -> Result<()> {
//     let (t, input) = prepare(11, TaskType::Normal)?;
//     b.iter(|| t.run(&input));
//     Ok(())
// }
//
// #[bench]
// fn day11_bonus(b: &mut Bencher) -> Result<()> {
//     let (t, input) = prepare(11, TaskType::Bonus)?;
//     b.iter(|| t.run(&input));
//     Ok(())
// }
//
// #[bench]
// fn day12_normal(b: &mut Bencher) -> Result<()> {
//     let (t, input) = prepare(12, TaskType::Normal)?;
//     b.iter(|| t.run(&input));
//     Ok(())
// }
//
// #[bench]
// fn day12_bonus(b: &mut Bencher) -> Result<()> {
//     let (t, input) = prepare(12, TaskType::Bonus)?;
//     b.iter(|| t.run(&input));
//     Ok(())
// }
