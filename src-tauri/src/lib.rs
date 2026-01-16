#![allow(dead_code, unused_variables)]
use std::sync::Mutex;
#[cfg_attr(mobile, tauri::mobile_entry_point)]

use std::time::Duration;
use tauri::{Emitter, Listener, Manager};
use tauri_plugin_store::StoreBuilder;
use time::macros::format_description;
use tracing::info;
use tracing_subscriber::fmt::time::LocalTime;
use crate::{commands::publish_global_event, types::{state::{AppState, MetricsState}, store::settings::AppUsttings}};
use crate::commands::todos::*;

mod commands;
mod types;
mod errors;
mod domain;
mod consts;
mod listen;

pub fn run() {
    let time_format = LocalTime::new(format_description!("[year]/[month]/[day] [hour]:[minute]:[second].[subsecond digits:3]"));
    tracing_subscriber::fmt()
                .with_max_level(tracing::Level::INFO)
                .with_target(false)
                .with_thread_ids(false)
                .with_file(true)
                .with_timer(time_format)
                .with_line_number(true)
                .init();
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let handle = app.handle();
            let window = app.get_webview_window("main").ok_or("not found window")?;
            // 加载 Store & 初始化 State
            let store = StoreBuilder::new(handle, consts::STORE_CONFIG)
                .auto_save(Duration::from_mins(1))
                .build()?;
            

            // 从 Store 中加载状态数据 & 初始化
            app.manage(AppState::load_from_store(handle)?);
            app.manage(Mutex::new(AppUsttings::load_from_store(handle)?));
            app.manage(MetricsState::new());

            // 窗口事件监听
            window.on_window_event(move |event| {
                match event {
                    tauri::WindowEvent::CloseRequested { api , .. } => {
                        store.save().expect("store save faield");
                        info!("Window closed.");
                    },
                    _ => {},
                }
            });

            // 注册事件监听器
            app.listen("download-started", listen::download_started_listen);
            app.listen("download-progress", listen::download_progress_listen);
            app.listen("download-finished", listen::download_finished_listen);
            // 仅监听一次事件，监听到事件后立刻卸载
            app.listen("app-started", listen::once_event_listen);

            // 发送单次事件
            app.emit("app-started", ())?;
            info!("App setup success");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::user::home,
            commands::example, 
            incr_counter,
            publish_global_event,
            todo_list,
            add_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
