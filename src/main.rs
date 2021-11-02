mod config;
mod task;

use crate::task::{build_dag, build_task_array, build_task_map, TaskWrapper};
use clap::Parser;
use config::Config;
use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, Barrier};
use task::Task;

fn process_batch(batch: Vec<usize>, task_map: &HashMap<usize, Task>) -> std::thread::Result<()> {

    let len = batch.len();
    let barrier = Arc::new(Barrier::new(len + 1));
    let task_array = build_task_array(batch, task_map);

    crossbeam::scope(|s| {
        for task in task_array {
            let b = barrier.clone();
            s.spawn(move |_| {
                let mut command = Command::new(&task.command);
                command.arg(&task.args[0]);
                println!("Task: {:?}", command);
                let output = command.output().expect("failed to execute process");
                b.wait();
                println!("\t{}", String::from_utf8(output.stdout).unwrap());
            });
        }
        barrier.wait();
    })
}

fn process_tasks(tasks: Vec<Task>) {
    let mut ts = build_dag(&tasks);

    let task_map = build_task_map(tasks);

    let task_map = Arc::new(task_map);
    loop {
        let batch = ts.pop_all();

        if batch.is_empty() && !ts.is_empty() {
            panic!("Found a cycle");
        } else if batch.is_empty() {
            return;
        }
        process_batch(batch, &task_map).expect("Thread group failed to run");
    }
}

fn main() {
    let config: Config = Config::parse();

    let output = std::fs::read_to_string(config.filename).unwrap();

    let task_wrapper: TaskWrapper = toml::from_str(&output).unwrap();
    process_tasks(task_wrapper.tasks);
}
