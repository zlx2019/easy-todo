// Todos commands

use std::sync::atomic::Ordering;
use serde::Deserialize;
use tauri::{AppHandle, State};
use tracing::info;

use crate::{
    domain::todo::Todo, types::{
        CmdResult, 
        state::{AppState, MetricsState}
    }
};


#[tauri::command]
pub async fn incr_counter(state: State<'_, MetricsState>) -> CmdResult<u64>{
    Ok(state.counter.fetch_add(1, Ordering::SeqCst) + 1)
}


/// 获取所有待办
#[tauri::command]
pub async fn todo_list(state: State<'_, AppState>) -> CmdResult<Vec<Todo>> {
    let todos = state.todos.lock().unwrap();
    let values = todos.values().cloned().collect::<Vec<Todo>>();
    Ok(values)
}

#[derive(Debug, Deserialize)]
pub struct TodoReq {
    pub id: Option<String>,
    pub title: String,
    pub content: String
}

/// 添加待办
#[tauri::command]
pub async fn add_todo(app: AppHandle, state: State<'_, AppState>, req: TodoReq) -> CmdResult<()>{
    let todo = Todo::new(req.title, req.content);
    {
        let mut todos = state.todos.lock().unwrap();
        todos.insert(todo.id.clone(), todo);
    }
    state.write_to_store(&app, true)?;
    info!("hello");
    Ok(())
}

///删除待办
#[tauri::command]
pub async fn delete_todo(app: AppHandle, todo_id: String) -> CmdResult<()>{
    Ok(())
}


/// 编辑待办
#[tauri::command]
pub async fn update_todo() -> CmdResult<()>{
    Ok(())
}