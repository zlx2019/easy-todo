// Todos commands

use std::sync::atomic::Ordering;
use tauri::State;
use crate::{base::CmdResult, model::{state::{AppState, MetricsState}, todo::Todo}};


#[tauri::command]
pub async fn incr_counter(state: State<'_, MetricsState>) -> CmdResult<u64>{
    Ok(state.counter.fetch_add(1, Ordering::SeqCst) + 1)
}


#[tauri::command]
pub async fn todo_list(state: State<'_, AppState>) -> CmdResult<Vec<Todo>> {
    let todos = state.todos.lock().unwrap();
    println!("{:#?}", todos);
    let values = todos.values().cloned().collect::<Vec<Todo>>();
    Ok(values)
}