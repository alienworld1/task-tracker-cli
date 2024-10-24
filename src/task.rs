use chrono::prelude::*;

#[derive(Debug)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug)]
pub struct Task {
    id: usize,
    description: String,
    status: Status,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl Task {
    pub fn new(id: usize, description: String, status: Status) -> Task {
        Task {
            id,
            description,
            status,
            created_at: Local::now(),
            updated_at: Local::now(),
        }
    }
}
