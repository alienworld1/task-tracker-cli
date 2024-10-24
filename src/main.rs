use std::env;

mod task;
mod task_tracker;

use task::{Status, Task};

fn main() {
    let command = env::args()
        .nth(1)
        .expect("The command to perform was not given!");
    println!("{command}");
    task_tracker::add();
    println!("{:?}", Task::new(1, String::from("buy groceries"), Status::Todo));
}
