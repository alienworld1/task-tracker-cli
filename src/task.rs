use std::cmp::{Eq, PartialEq};
use std::fmt::Display;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            Self::Done => "Done",
            Self::InProgress => "In Progress",
            Self::Todo => "Todo",
        };
        write!(f, "{status}")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: usize,
    description: String,
    status: Status,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl Task {
    pub fn new(id: usize, description: String) -> Task {
        Task {
            id,
            description,
            status: Status::Todo,
            created_at: Local::now(),
            updated_at: Local::now(),
        }
    }

    pub fn update_description(&mut self, new_description: String) {
        self.description = new_description;
        self.updated_at = Local::now();
    }

    pub fn update_status(&mut self, new_status: Status) {
        self.status = new_status;
        self.updated_at = Local::now();
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn compare_status(&self, status: &Status) -> bool {
        *status == self.status
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id: {}\nDescription: {}\nStatus: {}\nCreated at: {}\nUpdated at: {}\n",
            self.id,
            self.description,
            self.status,
            self.created_at.format("%d/%m/%Y %H:%M"),
            self.updated_at.format("%d/%m/%Y %H:%M")
        )
    }
}
