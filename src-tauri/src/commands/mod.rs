use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

use crate::model::{Todo, state::AppState};

pub mod todo;
pub mod user;


/// 操作示例
#[tauri::command]
pub async fn example(app: AppHandle) -> Result<String, &'static str>{
    // 获取 state
    let state = app.state::<AppState>();
    {
        let mut todos = state.todos.lock().unwrap();
        println!("todos: {:#?}", todos);
        todos.insert("zhaoliu".to_string(), Todo::new("zhaoliu"));
    }

    // 获取 store
    let store = app.store("settings.json").unwrap();
    state.write_to_store(&app, false).expect("write to store error");
    Ok("OK".to_string())
}