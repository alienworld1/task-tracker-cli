use std::fmt::Display;

use chrono::prelude::*;

#[derive(Debug)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            Self::Done => "Done",
            Self::InProgress => "InProgress",
            Self::Todo => "Todo",
        };
        write!(f, "{status}")
    }
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

    pub fn update_description(&mut self, new_description: String) {
        self.description = new_description;
    }

    pub fn update_status(&mut self, new_status: Status) {
        self.status = new_status;
    }

    pub fn to_json(&self) -> String {
        let json = format!(
            "{{\nid:{},\ndescription:{},\nstatus:{},\ncreated_at:{},\nupdated_at:{}\n}}",
            self.id, self.description, self.status, self.created_at, self.updated_at
        );
        json
    }
}
