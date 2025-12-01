mod year2024;
mod year2025;

use anyhow::{anyhow, Result};
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
    pub task_type: TaskType,
    pub year: u32,
}

impl Task {
    pub fn new(task_number: u8, task_type: TaskType, year: u32) -> Self {
        Self {
            task_number,
            task_type,
            year,
        }
    }

    pub fn run(&self, input: &str) -> Result<String> {
        match self.year {
            2015..=2023 => Err(anyhow!("This year is not implemented")),
            2024 => year2024::run_task(input, self.task_number, self.task_type),
            2025 => year2025::run_task(input, self.task_number, self.task_type),
            _ => Err(anyhow!("Invalid year")),
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
