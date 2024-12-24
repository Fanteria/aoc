#![feature(test)]

extern crate test;

use aoc::{
    files::{Files, FilesType},
    tasks::{Task, TaskType},
};
use test::Bencher;

fn prepare(task_num: u8, task_type: TaskType) -> (Task, String) {
    let task = Task::new(task_num, task_type);
    let input = Files::from_env().get_input(FilesType::Task, &task);
    (task, input)
}

#[bench]
fn task_01_normal(b: &mut Bencher) {
    let (t, input) = prepare(1, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_01_bonus(b: &mut Bencher) {
    let (t, input) = prepare(1, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_02_normal(b: &mut Bencher) {
    let (t, input) = prepare(2, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_02_bonus(b: &mut Bencher) {
    let (t, input) = prepare(2, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_03_normal(b: &mut Bencher) {
    let (t, input) = prepare(3, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_03_bonus(b: &mut Bencher) {
    let (t, input) = prepare(3, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_04_normal(b: &mut Bencher) {
    let (t, input) = prepare(4, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_04_bonus(b: &mut Bencher) {
    let (t, input) = prepare(4, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_05_normal(b: &mut Bencher) {
    let (t, input) = prepare(5, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_05_bonus(b: &mut Bencher) {
    let (t, input) = prepare(5, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_06_normal(b: &mut Bencher) {
    let (t, input) = prepare(6, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_06_bonus(b: &mut Bencher) {
    let (t, input) = prepare(6, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_07_normal(b: &mut Bencher) {
    let (t, input) = prepare(7, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_07_bonus(b: &mut Bencher) {
    let (t, input) = prepare(7, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_08_normal(b: &mut Bencher) {
    let (t, input) = prepare(8, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_08_bonus(b: &mut Bencher) {
    let (t, input) = prepare(8, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_09_normal(b: &mut Bencher) {
    let (t, input) = prepare(9, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_09_bonus(b: &mut Bencher) {
    let (t, input) = prepare(9, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_10_normal(b: &mut Bencher) {
    let (t, input) = prepare(10, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_10_bonus(b: &mut Bencher) {
    let (t, input) = prepare(10, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_11_normal(b: &mut Bencher) {
    let (t, input) = prepare(11, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_11_bonus(b: &mut Bencher) {
    let (t, input) = prepare(11, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_12_normal(b: &mut Bencher) {
    let (t, input) = prepare(12, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_12_bonus(b: &mut Bencher) {
    let (t, input) = prepare(12, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_13_normal(b: &mut Bencher) {
    let (t, input) = prepare(13, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_13_bonus(b: &mut Bencher) {
    let (t, input) = prepare(13, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_14_normal(b: &mut Bencher) {
    let (t, input) = prepare(14, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_14_bonus(b: &mut Bencher) {
    let (t, input) = prepare(14, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_15_normal(b: &mut Bencher) {
    let (t, input) = prepare(15, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_15_bonus(b: &mut Bencher) {
    let (t, input) = prepare(15, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_16_normal(b: &mut Bencher) {
    let (t, input) = prepare(16, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_16_bonus(b: &mut Bencher) {
    let (t, input) = prepare(16, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_17_normal(b: &mut Bencher) {
    let (t, input) = prepare(17, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_17_bonus(b: &mut Bencher) {
    let (t, input) = prepare(17, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_18_normal(b: &mut Bencher) {
    let (t, input) = prepare(18, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_18_bonus(b: &mut Bencher) {
    let (t, input) = prepare(18, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

// TODO missing 19

#[bench]
fn task_20_normal(b: &mut Bencher) {
    let (t, input) = prepare(20, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_20_bonus(b: &mut Bencher) {
    let (t, input) = prepare(20, TaskType::Bonus);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_21_normal(b: &mut Bencher) {
    let (t, input) = prepare(21, TaskType::Normal);
    b.iter(|| t.run(&input))
}

#[bench]
fn task_21_bonus(b: &mut Bencher) {
    let (t, input) = prepare(21, TaskType::Bonus);
    b.iter(|| t.run(&input))
}
