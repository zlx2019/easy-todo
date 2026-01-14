// use std::collections::HashMap;

// use serde::{Deserialize, Serialize};
// use serde_json::Value;
// use tauri_plugin_store::Store;

// use crate::model::Todo;


// #[derive(Debug, Serialize, Deserialize)]
// pub struct AppStore {
//     pub todos: HashMap<String, Todo>
// }

// impl AppStore {
//     pub fn new () -> Self {
//         Self { todos: HashMap::new() }
//     }

//     /// 从 Store 中加载数据
//     pub fn load_from_store<R: tauri::Runtime>(store: &Store<R>) -> Result<Self, Box<dyn std::error::Error>>{
//         let store=  store.get("TODOS")
//             .map(AppStore::from)
//             .unwrap_or_else(|| Self::new());
//         Ok(store)
//     }

//     // 将数据写入到 Store 中
//     pub fn write_to_store<R: tauri::Runtime>(&self, store: &Store<R>) -> Result<(), Box<dyn std::error::Error>>{
//         store.set("TODOS", self);
//         store.save()?;
//         Ok(())
//     }
// }

// impl Into<Value> for AppStore {
//     fn into(self) -> Value {
//         serde_json::to_value(self).unwrap()
//     }
// }

// impl From<Value> for AppStore {
//     fn from(value: Value) -> Self {
//         serde_json::from_value(value).expect("AppStore to Value error")
//     }
// }

// impl From<&AppStore> for Value {
//     fn from(value: &AppStore) -> Self {
//         serde_json::to_value(value).unwrap()
//     }
// }
