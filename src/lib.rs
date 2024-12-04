#![feature(test)]

extern crate test;

use std::{
    env,
    path::PathBuf,
    thread::{self, JoinHandle},
    time::Instant,
};

use tasks::Task;

pub mod tasks;

pub fn run_tasks(mut from: Task, to: Task) {
    let data_path = PathBuf::from(env::var("AOC_DATA").unwrap());
    loop {
        println!(
            "{}: {}",
            from,
            from.run(
                data_path
                    .join(format!("{:0>2}", from.task_number))
                    .join("in.txt")
            )
        );

        if from == to {
            break;
        }
        from = from.next()
    }
}

pub fn mesure_tasks(from: Task, to: Task) {
    let mut task = from.clone();
    let data_path = PathBuf::from(env::var("AOC_DATA").unwrap());
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    let now = Instant::now();
    while task != to {
        let path = data_path
            .join(format!("{:0>2}", from.task_number))
            .join("in.txt");
        let thread_task = from.clone();
        threads.push(thread::spawn(move || {
            thread_task.run(path);
        }));
        task = task.next()
    }
    for thread in threads {
        thread.join().unwrap()
    }
    let elapsed = now.elapsed();
    println!("Run tasks from: {}, to: {}", from, to);
    println!("Elapsed time: {:?}", elapsed);
}
