mod task_01;
mod task_02;
mod task_03;
mod task_04;
mod task_05;
mod task_06;
mod task_07;
mod task_08;

use task_01::Task01;
use task_02::Task02;
use task_03::Task03;
use task_04::Task04;
use task_05::Task05;
use task_06::Task06;
use task_07::Task07;
use task_08::Task08;

use core::panic;
use std::{
    env,
    fmt::Display,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use clap::{Parser, ValueEnum};

pub trait TaskRun {
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
        write!(f, "{self:?}")
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

    fn get_file(&self, file_name: &str) -> PathBuf {
        let dir =
            PathBuf::from(env::var("AOC_DATA").unwrap()).join(format!("{:0>2}", self.task_number));
        let file_path = dir.join(format!(
            "{}_{}.txt",
            self.task_type.to_string().to_lowercase(),
            file_name
        ));
        // println!("get file {:?}", file_path);
        if file_path.exists() {
            file_path
        } else {
            dir.join(format!("{}.txt", file_name))
        }
    }

    pub fn get_example_in_path(&self) -> PathBuf {
        self.get_file("example_in")
    }

    pub fn get_custom_example_in_path(&self, name: &str) -> PathBuf {
        self.get_file(&format!("example_{name}_in"))
    }

    pub fn get_example_out_path(&self) -> PathBuf {
        self.get_file("example_out")
    }

    pub fn get_custom_example_out_path(&self, name: &str) -> PathBuf {
        self.get_file(&format!("example_{name}_out"))
    }

    pub fn get_in_path(&self) -> PathBuf {
        self.get_file("in")
    }

    pub fn get_out_path(&self) -> PathBuf {
        self.get_file("out")
    }

    pub fn get_output(&self, path: impl AsRef<Path>) -> usize {
        let mut file = File::open(path.as_ref())
            .unwrap_or_else(|e| panic!("Failed to open file {:?}. Error: {e}", path.as_ref()));
        let mut ret = String::new();
        file.read_to_string(&mut ret)
            .unwrap_or_else(|e| panic!("Failed to read fiel {:?}. Error: {e}", path.as_ref()));
        ret.trim()
            .parse()
            .expect("Out file should contain only number")
    }

    pub fn get_input(path: impl AsRef<Path>) -> String {
        let mut file = File::open(path.as_ref())
            .unwrap_or_else(|e| panic!("Failed to open file {:?}. Error: {e}", path.as_ref()));
        let mut ret = String::new();
        file.read_to_string(&mut ret)
            .unwrap_or_else(|e| panic!("Failed to read fiel {:?}. Error: {e}", path.as_ref()));
        ret
    }

    pub fn run(&self, path: impl AsRef<Path>) -> usize {
        match (self.task_number, &self.task_type) {
            (1, TaskType::Normal) => Task01::normal(&Task::get_input(path)),
            (1, TaskType::Bonus) => Task01::bonus(&Task::get_input(path)),
            (2, TaskType::Normal) => Task02::normal(&Task::get_input(path)),
            (2, TaskType::Bonus) => Task02::bonus(&Task::get_input(path)),
            (3, TaskType::Normal) => Task03::normal(&Task::get_input(path)),
            (3, TaskType::Bonus) => Task03::bonus(&Task::get_input(path)),
            (4, TaskType::Normal) => Task04::normal(&Task::get_input(path)),
            (4, TaskType::Bonus) => Task04::bonus(&Task::get_input(path)),
            (5, TaskType::Normal) => Task05::normal(&Task::get_input(path)),
            (5, TaskType::Bonus) => Task05::bonus(&Task::get_input(path)),
            (6, TaskType::Normal) => Task06::normal(&Task::get_input(path)),
            (6, TaskType::Bonus) => Task06::bonus(&Task::get_input(path)),
            (7, TaskType::Normal) => Task07::normal(&Task::get_input(path)),
            (7, TaskType::Bonus) => Task07::bonus(&Task::get_input(path)),
            (8, TaskType::Normal) => Task08::normal(&Task::get_input(path)),
            (8, TaskType::Bonus) => Task08::bonus(&Task::get_input(path)),
            _ => panic!("Task solution not implemented."),
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
