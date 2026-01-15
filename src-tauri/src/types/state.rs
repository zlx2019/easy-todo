use anyhow::Ok;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicU64, Mutex},
};
use tauri::AppHandle;

use crate::{domain::todo::Todo, types::store::{load_from_store, write_to_store}};


#[derive(Debug, Deserialize, Serialize)]
pub struct AppState {
    pub todos: Mutex<HashMap<String, Todo>>,
}

impl AppState {
    pub fn new(todos: Option<HashMap<String, Todo>>) -> Self {
        Self {
            todos: Mutex::new(todos.unwrap_or(HashMap::new())),
        }
    }

    // load store
    pub fn load_from_store(app: &AppHandle) -> anyhow::Result<Self> {
        let todos = load_from_store::<HashMap<String, Todo>>(app, "settings.json", "todos")?;
        Ok(Self::new(Some(todos)))
    }

    // write store
    pub fn write_to_store(&self, app: &AppHandle, save: bool,
    ) -> anyhow::Result<()> {
        Ok(write_to_store(app, "settings.json", "todos", self.todos.lock().unwrap().clone(), save)?)
    }
}

/// 指标状态
pub struct MetricsState {
    pub counter: AtomicU64,
}

impl MetricsState {
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
        }
    }
}
