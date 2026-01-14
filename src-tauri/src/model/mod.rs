use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub mod state;
pub mod store;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub content: String,
    pub status: TodoStatus,
    pub created_at: DateTime<Local>,
}

impl Todo {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.into(),
            content: "xxx".to_string(),
            status: TodoStatus::Pending,
            created_at: Local::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TodoStatus {
    Pending = 0,
    Completed = 1,
}
