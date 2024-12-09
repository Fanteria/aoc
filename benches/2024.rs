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
