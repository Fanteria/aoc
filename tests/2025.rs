use anyhow::Result;
use aoc::{
    files::{Files, FilesType},
    tasks::{Task, TaskType},
};

fn get_io(files: &Files, file_type: FilesType, task: &Task) -> Result<(String, String)> {
    Ok((
        files.get_input(file_type, task)?,
        files.get_output(file_type, task),
    ))
}

#[test]
fn task_01() -> Result<()> {
    let f = Files::from_env()?;

    let t = Task::new(1, TaskType::Normal, 2025);
    let (input, output) = get_io(&f, FilesType::Example, &t)?;
    assert_eq!(t.run(&input)?, output);
    let (input, output) = get_io(&f, FilesType::Task, &t)?;
    assert_eq!(t.run(&input)?, output);

    let t = Task::new(1, TaskType::Bonus, 2025);
    let (input, output) = get_io(&f, FilesType::Example, &t)?;
    assert_eq!(t.run(&input)?, output);
    let (input, output) = get_io(&f, FilesType::Task, &t)?;
    assert_eq!(t.run(&input)?, output);

    Ok(())
}

#[test]
fn task_02() -> Result<()> {
    let f = Files::from_env()?;

    let t = Task::new(2, TaskType::Normal, 2025);
    let (input, output) = get_io(&f, FilesType::Example, &t)?;
    assert_eq!(t.run(&input)?, output);
    let (input, output) = get_io(&f, FilesType::Task, &t)?;
    assert_eq!(t.run(&input)?, output);

    let t = Task::new(2, TaskType::Bonus, 2025);
    let (input, output) = get_io(&f, FilesType::Example, &t)?;
    assert_eq!(t.run(&input)?, output);
    let (input, output) = get_io(&f, FilesType::Task, &t)?;
    assert_eq!(t.run(&input)?, output);

    Ok(())
}
