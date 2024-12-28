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

#[test]
fn task_10() {
    let f = Files::from_env();

    let t = Task::new(10, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(10, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_11() {
    let f = Files::from_env();

    let t = Task::new(11, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(11, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_12() {
    let f = Files::from_env();

    let t = Task::new(12, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Custom("2"), &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(12, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Custom("2"), &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Custom("3"), &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Custom("4"), &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_13() {
    let f = Files::from_env();

    let t = Task::new(13, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(13, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_14() {
    let f = Files::from_env();

    let t = Task::new(14, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(14, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_15() {
    let f = Files::from_env();

    let t = Task::new(15, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Custom("2"), &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(15, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Custom("3"), &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Custom("4"), &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_16() {
    let f = Files::from_env();

    let t = Task::new(16, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Custom("1"), &t);
    assert_eq!(t.run(&input), output);
    // let (input, output) = get_io(&f, FilesType::Task, &t);
    // assert_eq!(t.run(&input), output);

    let t = Task::new(16, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Custom("1"), &t);
    assert_eq!(t.run(&input), output);
    // let (input, output) = get_io(&f, FilesType::Task, &t);
    // assert_eq!(t.run(&input), output);
}

#[test]
fn task_17() {
    let f = Files::from_env();

    let t = Task::new(17, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(17, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Custom("1"), &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_18() {
    let f = Files::from_env();

    let t = Task::new(18, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(18, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_19() {
    let f = Files::from_env();

    let t = Task::new(19, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(19, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_20() {
    let f = Files::from_env();

    let t = Task::new(20, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(20, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_21() {
    let f = Files::from_env();

    let t = Task::new(21, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(21, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_22() {
    let f = Files::from_env();

    let t = Task::new(22, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(22, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}

#[test]
fn task_23() {
    let f = Files::from_env();

    let t = Task::new(23, TaskType::Normal);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);

    let t = Task::new(23, TaskType::Bonus);
    let (input, output) = get_io(&f, FilesType::Example, &t);
    assert_eq!(t.run(&input), output);
    let (input, output) = get_io(&f, FilesType::Task, &t);
    assert_eq!(t.run(&input), output);
}
