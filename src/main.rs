use std::env;

mod task;
mod task_tracker;

fn main() -> std::io::Result<()> {
    let command = env::args()
        .nth(1)
        .expect("The command to perform was not given!");
    println!("{command}");

    // let task = Task::new(1, String::from("buy groceries"), Status::Todo);
    // task_tracker::add_task(task)?;
    // let task2 = Task::new(2, String::from("task 2"), Status::InProgress);
    // task_tracker::add_task(task2)?;

    task_tracker::update_task(1, "buy groceries and cook dinner".to_string())?;

    Ok(())
}
