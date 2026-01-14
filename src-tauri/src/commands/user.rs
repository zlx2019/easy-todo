use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub async fn home(app: AppHandle) -> Result<String, &'static str>{
    // 获取 store
    let store =  app.store("store.json").map_err(|_|{
        "get store error"
    })?;

    // 获取 store 某个值
    if let Some(value) = store.get("APP_WINDOWS") {
        if let Some(title) = value["title"].as_str() {
            println!("title: {}", title);   
        }
    }
    Ok("Hello Zero.".to_string())
}


