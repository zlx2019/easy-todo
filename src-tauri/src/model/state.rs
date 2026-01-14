use crate::model::Todo;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicU64, Mutex},
};
use tauri::AppHandle;
use tauri_plugin_store::{Store, StoreExt};

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
    pub fn load_from_store<R: tauri::Runtime>(store: &Store<R>) -> Self {
        let todos = store
            .get("TODOS")
            .and_then(|value| serde_json::from_value::<HashMap<String, Todo>>(value).ok())
            .unwrap_or_default();
        Self {
            todos: Mutex::new(todos),
        }
    }

    // write store
    pub fn write_to_store(&self, app: &AppHandle, save: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(store) = app.get_store("settings.json") {
            let todos = self.todos.lock().unwrap();
            store.set("TODOS", serde_json::to_value(todos.clone())?);
            if save {
                store.save()?;
            }
        }
        Ok(())
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
