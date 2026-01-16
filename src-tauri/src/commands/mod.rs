use std::{sync::Mutex, time::Duration};

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_store::StoreExt;
use tracing::info;

use crate::{
    domain::todo::Todo,
    types::{CmdResult, state::AppState, store::settings::AppUsttings},
};

pub mod todos;
pub mod user;

/// 操作示例
#[tauri::command]
pub async fn example(app: AppHandle) -> CmdResult<String> {
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
    state
        .write_to_store(&app, true)
        .expect("write to store error");
    Ok("OK".to_string())
}

// 事件：Rust和前端都可以发布和订阅
// 发布全局事件
#[tauri::command]
pub async fn publish_global_event(app: AppHandle) -> CmdResult<()> {
    tokio::spawn(async move {
        info!("publish global event");
        // 发送全局事件
        app.emit("download-started", "https://xxx.com/user.jpg")
            .unwrap();
        for progress in [1, 15, 50, 80, 100] {
            app.emit("download-progress", progress).unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        app.emit("download-finished", "https://xxx.com/user.jpg")
            .unwrap();
    });
    Ok(())
}

