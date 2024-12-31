#![feature(array_try_map)]

pub mod files;
pub mod tasks;
pub mod utils;

use anyhow::Result;
use tasks::Task;

pub fn run_tasks(mut from: Task, to: Task) -> Result<()> {
    let files = files::Files::from_env()?;
    loop {
        println!(
            "{from} result: {}",
            from.run(&files.get_input(files::FilesType::Task, &from)?)?
        );
        if from == to {
            break;
        }
        from = from.next()
    }
    Ok(())
}
