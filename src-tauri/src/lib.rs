#![allow(dead_code, unused_variables)]

use std::time::Duration;

use serde_json::json;
use tauri::Manager;
use tauri_plugin_store::StoreBuilder;

use crate::model::state::{AppState, MetricsState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
mod commands;
mod model;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let window = app.get_webview_window("main").ok_or("not found window")?;
            
            // 加载 Store & 初始化 State
            let store = StoreBuilder::new(app.handle(), "settings.json")
                .auto_save(Duration::from_mins(1))
                .default("APP_WINDOWS", json!({"title": window.title()?}))
                .build()?;
            let app_state = AppState::load_from_store(&store);
            app.manage(app_state);
            app.manage(MetricsState::new());

            // 设置日志级别
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            // 窗口事件监听
            window.on_window_event(move |event| {
                match event {
                    tauri::WindowEvent::CloseRequested { api , .. } => {
                        store.save().expect("store save faield");
                        println!("Window closed.");
                    },
                    _ => {},
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::user::home, commands::example])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
