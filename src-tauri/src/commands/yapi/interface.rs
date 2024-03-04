use serde_json::json;
use tauri::{async_runtime::spawn, AppHandle, Manager, State};

use crate::{
    models::{
        web_response::WebResponse,
        yapi::{interface::InterfaceFetchParams, queue::Queue},
    },
    services::{log::log_error, yapi::interface::write_content_to_interface_path},
};

#[tauri::command]
pub async fn add_interface_task(
    app_handle: AppHandle,
    data: InterfaceFetchParams,
) -> Result<WebResponse, String> {
    let queue: State<'_, Queue> = app_handle.state();

    queue.add_task(data).await;

    Ok(WebResponse {
        data: None,
        message: "已添加任务".to_string(),
    })
}

#[tauri::command]
pub async fn start_task(app_handle: AppHandle) -> Result<WebResponse, String> {
    let queue: State<'_, Queue> = app_handle.state();
    let count = queue.waiting_queue.lock().await.len();
    spawn(async move {
        let app_handle_clone = app_handle.clone();
        let queue: State<'_, Queue> = app_handle_clone.state();
        queue.start_execute(&app_handle_clone).await;
    });

    Ok(WebResponse {
        data: Some(json!(count)),
        message: "已启动跑批任务".to_string(),
    })
}

#[tauri::command]
pub async fn cancel_task(app_handle: AppHandle) -> Result<WebResponse, String> {
    let queue: State<'_, Queue> = app_handle.state();
    queue.cancel_execute().await;
    Ok(WebResponse {
        data: None,
        message: "已停止跑批任务".to_string(),
    })
}

#[tauri::command]
pub fn write_to_file(
    path: String,
    content: String,
    source_path: &str,
    app_handle: AppHandle,
) -> Result<WebResponse, String> {
    match write_content_to_interface_path(path, source_path, content) {
        Err(e) => log_error(&app_handle, e.to_string()),
        Ok(_) => Ok(WebResponse {
            data: None,
            message: "已写入文件".to_string(),
        }),
    }
}
