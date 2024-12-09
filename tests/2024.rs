use aoc::{
    files::{Files, FilesType},
    tasks::{Task, TaskType},
};

pub fn get_io(files: &Files, file_type: FilesType, task: &Task) -> (String, usize) {
    (
        files.get_input(file_type, task),
        files.get_output(file_type, task),
    )
}

#[test]
fn task_01() {
    let f = Files::from_env();

    let t = Task::new(1, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(1, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_02() {
    let t = Task::new(2, TaskType::Normal);
    let f = Files::from_env();
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(2, TaskType::Bonus);
    let f = Files::from_env();
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_03() {
    let t = Task::new(3, TaskType::Normal);
    let f = Files::from_env();
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(3, TaskType::Bonus);
    let f = Files::from_env();
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_04() {
    let t = Task::new(4, TaskType::Normal);
    let f = Files::from_env();
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(4, TaskType::Bonus);
    let f = Files::from_env();
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_05() {
    let t = Task::new(5, TaskType::Normal);
    let f = Files::from_env();
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(5, TaskType::Bonus);
    let f = Files::from_env();
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_06() {
    let t = Task::new(6, TaskType::Normal);
    let f = Files::from_env();
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(6, TaskType::Bonus);
    let f = Files::from_env();
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_07() {
    let t = Task::new(7, TaskType::Normal);
    let f = Files::from_env();
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(7, TaskType::Bonus);
    let f = Files::from_env();
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_08() {
    let f = Files::from_env();

    let t = Task::new(8, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Custom("2"), &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Custom("3"), &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(8, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Custom("4"), &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_09() {
    let f = Files::from_env();

    let t = Task::new(9, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(9, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}
