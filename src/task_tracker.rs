use std::fs::File;
use std::io::{self, Read, Write};

use crate::task::Task;
use serde_json;

pub fn get_tasks() -> io::Result<Vec<Task>> {
    let file = File::open("tasks.json");
    match file {
        Ok(mut tasks_file) => {
            let mut buffer = Vec::new();
            tasks_file.read_to_end(&mut buffer)?;
            let tasks: Vec<Task> = serde_json::from_slice(&buffer)?;
            return Ok(tasks);
        }
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                return Ok(Vec::new());
            }
            _ => {
                return Err(error);
            }
        },
    }
}

pub fn add_task(task: Task) -> io::Result<usize> {
    let mut tasks = get_tasks()?;
    let id = task.get_id();

    tasks.push(task);

    let mut file = File::options()
        .create(true)
        .write(true)
        .open("tasks.json")?;
    let tasks_json = serde_json::to_vec(&tasks)?;

    file.write(&tasks_json)?;
    Ok(id)
}
