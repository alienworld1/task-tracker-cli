use std::fs::File;
use std::io::{self, Read, Write};

use crate::task::{Status, Task};
use serde_json;

fn get_tasks() -> io::Result<Vec<Task>> {
    let file = File::open("tasks.json");
    match file {
        Ok(mut tasks_file) => {
            let mut buffer = Vec::new();
            tasks_file.read_to_end(&mut buffer)?;
            let tasks: Vec<Task> = serde_json::from_slice(&buffer)?;
            Ok(tasks)
        }
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => Ok(Vec::new()),
            _ => Err(error),
        },
    }
}

fn write_tasks_to_file(tasks: &Vec<Task>) -> io::Result<()> {
    let mut file = File::options()
        .create(true)
        .write(true)
        .open("tasks.json")?;
    let tasks_json = serde_json::to_vec(tasks)?;

    file.write(&tasks_json)?;
    Ok(())
}

pub fn add_task(description: String) -> io::Result<usize> {
    let mut tasks = get_tasks()?;

    let last_task_id = match tasks.last() {
        Some(task) => task.get_id(),
        None => 0,
    };
    let new_task = Task::new(last_task_id + 1, description);
    let id = new_task.get_id();

    tasks.push(new_task);
    write_tasks_to_file(&tasks)?;

    Ok(id)
}

pub fn update_task(id: usize, new_description: String) -> io::Result<()> {
    let mut tasks = get_tasks()?;

    for task in &mut tasks {
        if task.get_id() == id {
            task.update_description(new_description);
            break;
        }
    }

    write_tasks_to_file(&tasks)?;

    Ok(())
}

pub fn update_status(id: usize, new_status: Status) -> io::Result<()> {
    let mut tasks = get_tasks()?;

    for task in &mut tasks {
        if task.get_id() == id {
            task.update_status(new_status);
            break;
        }
    }

    write_tasks_to_file(&tasks)?;
    Ok(())
}

pub fn list_all_tasks() -> io::Result<Vec<Task>> {
    let tasks = get_tasks()?;
    Ok(tasks)
}
