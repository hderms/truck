mod config;
mod executor;
mod task;

use clap::Parser;
use config::Config;
use executor::process_tasks;
use task::TaskWrapper;

fn main() {
    let config: Config = Config::parse();

    let output = std::fs::read_to_string(config.filename).unwrap();

    let task_wrapper: TaskWrapper = toml::from_str(&output).unwrap();
    process_tasks(task_wrapper.tasks);
}
