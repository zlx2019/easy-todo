use tauri::Event;
use tracing::info;


/// `download-started` 事件监听器
pub fn download_started_listen(event: Event){
    info!("[download-started-listen] id: {}, payload: {}", &event.id(), &event.payload());
}

/// `download-progress` 事件监听器
pub fn download_progress_listen(event: Event){
    info!("[download-progress-listen] id: {}, payload: {}", &event.id(), &event.payload());
}

/// `download-finished` 事件监听器
pub fn download_finished_listen(event: Event){
    info!("[download-finished-listen] id: {}, payload: {}", &event.id(), &event.payload());
}


/// `app-startedt` 事件监听器
pub fn once_event_listen(event: Event){
    info!("[once-event-listen] 程序已启动");
}