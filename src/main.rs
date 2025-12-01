use anyhow::Result;
use aoc::{
    run_tasks,
    tasks::{Task, TaskType},
};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// The starting task number
    from_task: u8,

    /// The starting task type
    #[arg(default_value = "normal")]
    from_task_type: TaskType,

    #[arg(default_value = "2025")]
    from_task_year: u32,

    /// The ending task number, if not set, from task will be used
    to_task: Option<u8>,

    /// The ending task type
    to_task_type: Option<TaskType>,

    to_task_year: Option<u32>,
}

impl Args {
    pub fn get_task_range(&self) -> (Task, Task) {
        let from = Task::new(self.from_task, self.from_task_type, self.from_task_year);
        let to = Task::new(
            self.to_task.unwrap_or(from.task_number),
            self.to_task_type.unwrap_or(from.task_type),
            self.to_task_year.unwrap_or(from.year),
        );

        (from, to)
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let (from, to) = args.get_task_range();
    run_tasks(from, to)
}
