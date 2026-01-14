use chrono::{DateTime, Local, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub content: String,
    pub status: TodoStatus,
    #[serde(with = "datetime_format")]
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

mod datetime_format {
    use super::*;
    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";
    pub fn serialize<S>(datetime: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = datetime.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
            .map(|dt| DateTime::<Local>::from_naive_utc_and_offset(dt, *Local::now().offset()))
    }
}