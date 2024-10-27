use std::env;
use std::process;

mod task;
mod task_tracker;

use task::Status;
use task_tracker::list_all_tasks;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let command = Command::parse_arguments(&args).unwrap_or_else(|err| {
        eprintln!("Unable to parse arguments: {err}");
        process::exit(1);
    });

    match command {
        Command::Add(description) => {
            let task_id = task_tracker::add_task(description)?;
            println!("Task added successfully (ID: {task_id})");
        }
        Command::Update(id_to_update, new_description) => {
            task_tracker::update_task(id_to_update, new_description)?;
        }
        Command::MarkDone(id) => {
            task_tracker::update_status(id, Status::Done)?;
        }
        Command::MarkInProgress(id) => {
            task_tracker::update_status(id, Status::InProgress)?;
        }
        Command::ListAllTasks(provided_status) => {
            let tasks = list_all_tasks()?;
            match provided_status {
                Some(status) => {
                    for task in &tasks {
                        if task.compare_status(&status) {
                            println!("{}", task);
                        }
                    }
                }
                None => {
                    println!("displaying");
                    for task in &tasks {
                        println!("{}", task);
                    }
                }
            }
        }
    }

    Ok(())
}

enum Command {
    Add(String),
    Update(usize, String),
    MarkInProgress(usize),
    MarkDone(usize),
    ListAllTasks(Option<Status>),
}

impl Command {
    fn parse_arguments(args: &Vec<String>) -> Result<Command, &'static str> {
        if args.len() < 2 {
            return Err("Usage: task-cli <command> <arguments>");
        }

        if args[1] == "add" {
            if let Some(description) = args.get(2) {
                let command = Command::Add(description.clone());
                Ok(command)
            } else {
                Err("Description to add not specified")
            }
        } else if args[1] == "update" {
            if let Some(update) = args.get(2..4) {
                if let Ok(id_to_update) = update[0].parse::<usize>() {
                    let command = Command::Update(id_to_update, update[1].clone());
                    Ok(command)
                } else {
                    Err("Unable to parse provided id")
                }
            } else {
                Err("Usage: task-cli update <id> <new_description>")
            }
        } else if args[1] == "mark-in-progress" {
            if let Some(id) = args.get(2) {
                match id.parse::<usize>() {
                    Ok(id) => {
                        let command = Command::MarkInProgress(id);
                        Ok(command)
                    }
                    Err(_) => Err("Unable to parse provided id"),
                }
            } else {
                Err("The task id is not provided")
            }
        } else if args[1] == "mark-done" {
            if let Some(id) = args.get(2) {
                match id.parse::<usize>() {
                    Ok(id) => {
                        let command = Command::MarkDone(id);
                        Ok(command)
                    }
                    Err(_) => Err("Unable to parse provided id"),
                }
            } else {
                Err("The task id is not provided")
            }
        } else if args[1] == "list" {
            if let Some(status) = args.get(2) {
                if status == "done" {
                    Ok(Command::ListAllTasks(Some(Status::Done)))
                } else if status == "in-progress" {
                    Ok(Command::ListAllTasks(Some(Status::InProgress)))
                } else if status == "todo" {
                    Ok(Command::ListAllTasks(Some(Status::Todo)))
                } else {
                    Err("the 2nd argument has to be a status - either \"done\", \"in-progress\" or \"todo\"")
                }
            } else {
                Ok(Command::ListAllTasks(None))
            }
        } else {
            Err("The provided arguments are invalid")
        }
    }
}
