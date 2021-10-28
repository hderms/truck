use crate::task::{build_dag, build_task_array, build_task_map, Task};
use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, Barrier};



/// Process a Vector of Tasks by topologically sorting them based on dependencies,
/// popping off batches of free tasks and then running them in parallel until we're done
pub fn process_tasks(tasks: Vec<Task>) {
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
