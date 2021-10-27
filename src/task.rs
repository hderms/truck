use std::collections::HashMap;
use topological_sort::TopologicalSort;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Task {
    pub id: usize,
    pub command: String,
    pub depends_on: Vec<usize>,
    pub args: Vec<String>,
}
pub fn build_dag(tasks: &[Task]) -> TopologicalSort<usize> {
    let mut ts = TopologicalSort::<usize>::new();

    for task in tasks {
        if task.depends_on.is_empty() {
            ts.insert(task.id);

        } else {
            for dependency in &task.depends_on {
                ts.add_dependency(*dependency, task.id);
            }
        }
    }
    ts
}

pub fn build_task_array(batch: Vec<usize>, task_map: &HashMap<usize, Task>) -> Vec<&Task> {
    let mut task_array = Vec::with_capacity(batch.len());
    for task_id in batch {
        let task = task_map.get(&task_id);
        let task = task.expect("Could not find a task which matches id");
        task_array.push(task);
    }
    task_array
}

pub fn build_task_map(tasks: Vec<Task>) -> HashMap<usize, Task> {
    let mut task_map: HashMap<usize, Task> = HashMap::with_capacity(tasks.len());

    for task in tasks {
        task_map.insert(task.id, task);
    }
    task_map
}

#[derive( Deserialize)]
pub struct TaskWrapper{
    pub tasks: Vec<Task>

}
