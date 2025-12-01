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
    let task = Task::new(task_num, task_type, 2024);
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

#[bench]
fn day10_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(10, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day10_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(10, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day11_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(11, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day11_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(11, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day12_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(12, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day12_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(12, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day13_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(13, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day13_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(13, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day14_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(14, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day14_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(14, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day15_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(15, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day15_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(15, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day16_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(16, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day16_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(16, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day17_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(17, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day17_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(17, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day18_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(18, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day18_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(18, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day19_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(19, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day19_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(19, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day20_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(20, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day20_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(20, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day21_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(21, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day21_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(21, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day22_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(22, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day22_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(22, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day23_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(23, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day23_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(23, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day24_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(24, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day24_bonus(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(24, TaskType::Bonus)?;
    b.iter(|| t.run(&input));
    Ok(())
}

#[bench]
fn day25_normal(b: &mut Bencher) -> Result<()> {
    let (t, input) = prepare(25, TaskType::Normal)?;
    b.iter(|| t.run(&input));
    Ok(())
}
