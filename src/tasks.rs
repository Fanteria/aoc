mod task_01;
mod task_02;
mod task_03;
mod task_04;
mod task_05;
mod task_06;
mod task_07;
mod task_08;

use clap::{Parser, ValueEnum};
use std::fmt::Display;

static TASKS_NORMAL: &[fn(input: &str) -> usize] = &[
    task_01::Task01::normal,
    task_02::Task02::normal,
    task_03::Task03::normal,
    task_04::Task04::normal,
    task_05::Task05::normal,
    task_06::Task06::normal,
    task_07::Task07::normal,
    task_08::Task08::normal,
];

static TASKS_BONUS: &[fn(input: &str) -> usize] = &[
    task_01::Task01::bonus,
    task_02::Task02::bonus,
    task_03::Task03::bonus,
    task_04::Task04::bonus,
    task_05::Task05::bonus,
    task_06::Task06::bonus,
    task_07::Task07::bonus,
    task_08::Task08::bonus,
];

trait TaskRun {
    fn normal(input: &str) -> usize;

    fn bonus(input: &str) -> usize;
}

#[derive(ValueEnum, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TaskType {
    #[default]
    Normal,
    Bonus,
}

impl Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TaskType::Normal => "normal",
                TaskType::Bonus => "bonus",
            }
        )
    }
}

#[derive(Parser, Clone, Debug, PartialEq, Eq)]
pub struct Task {
    pub task_number: u8,

    #[arg(default_value_t = TaskType::Normal)]
    pub task_type: TaskType,
}

impl Task {
    pub fn new(task_number: u8, task_type: TaskType) -> Self {
        Self {
            task_number,
            task_type,
        }
    }

    pub fn run(&self, input: &str) -> usize {
        match self.task_type {
            TaskType::Normal => TASKS_NORMAL[(self.task_number - 1) as usize](input),
            TaskType::Bonus => TASKS_BONUS[(self.task_number - 1) as usize](input),
        }
    }

    pub fn next(mut self) -> Self {
        match &self.task_type {
            TaskType::Normal => self.task_type = TaskType::Bonus,
            TaskType::Bonus => {
                self.task_type = TaskType::Normal;
                self.task_number += 1;
            }
        }
        self
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task {:0>2} {}", self.task_number, self.task_type)
    }
}
