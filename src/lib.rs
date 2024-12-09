pub mod files;
pub mod tasks;
pub mod utils;

use std::{
    env,
    path::PathBuf,
    thread::{self, JoinHandle},
    time::Instant,
};

use tasks::Task;

pub fn run_tasks(mut from: Task, to: Task) {
    let files = files::Files::from_env();
    while from != to {
        println!(
            "{from}: {}",
            from.run(&files.get_input(files::FilesType::Task, &from))
        );
        from = from.next()
    }
}

pub fn mesure_tasks(mut from: Task, to: Task) {
    let files = files::Files::new(PathBuf::from(env::var("AOC_DATA").unwrap()));
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    let now = Instant::now();
    while from != to {
        let thread_task = from.clone();
        let input = files.get_input(files::FilesType::Task, &thread_task);
        threads.push(thread::spawn(move || {
            thread_task.run(&input);
        }));
        from = from.next()
    }
    for thread in threads {
        thread.join().unwrap()
    }
    let elapsed = now.elapsed();
    println!("Run tasks from: {}, to: {}", from, to);
    println!("Elapsed time: {:?}", elapsed);
}
