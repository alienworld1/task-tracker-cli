use std::env;
use std::process;

mod task;
mod task_tracker;

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
    }

    Ok(())
}

enum Command {
    Add(String),
    Update(usize, String),
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
        } else {
            Err("The provided arguments are invalid")
        }
    }
}
