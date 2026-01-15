use std::sync::Mutex;

use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

use crate::{domain::todo::Todo, types::{CmdResult, state::AppState, store::settings::AppUsttings}};


pub mod todos;
pub mod user;

/// 操作示例
#[tauri::command]
pub async fn example(app: AppHandle) -> CmdResult<String>{
    // 获取 state
    let state = app.state::<AppState>();
    {
        let mut todos = state.todos.lock().unwrap();
        println!("todos: {:#?}", todos);
        todos.insert("zhaoliu".to_string(), Todo::new("zhaoliu", ""));
    }

    let settings_state = app.state::<Mutex<AppUsttings>>();
    let mut setting = settings_state.lock().unwrap();
    setting.title = "my title".to_string();
    setting.write_to_store(&app)?;

    // 获取 store
    let store = app.store("settings.json").unwrap();
    state.write_to_store(&app, true).expect("write to store error");
    Ok("OK".to_string())
}