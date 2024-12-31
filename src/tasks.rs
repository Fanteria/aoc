mod year2024;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::fmt::Display;

trait TaskRun: Sync {
    fn normal(input: &str) -> Result<impl Display>
    where
        Self: Sized;

    fn bonus(input: &str) -> Result<impl Display>
    where
        Self: Sized;
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

    pub fn run(&self, input: &str) -> Result<String> {
        year2024::run_task(input, self.task_number, self.task_type)
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
