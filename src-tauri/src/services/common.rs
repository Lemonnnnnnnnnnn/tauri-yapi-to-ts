use serde::{Deserialize, Serialize};
use tauri::{State, Manager};

use crate::structs::queue::Queue;

#[derive(serde::Serialize)]
pub struct CustomResponse {
    pub message: String,
    pub data: Option<serde_json::Value>,
}


#[tauri::command]
pub async fn execute(handle: tauri::AppHandle) -> Result<CustomResponse, String> {
    tauri::async_runtime::spawn(async move {
        let queue: State<'_, Queue> = handle.state();
        queue.start_task().await;
    });
    return Ok(CustomResponse {
        message: String::from("正在异步执行任务..."),
        data: None,
    });
}

#[tauri::command]
pub async fn pause(handle: tauri::AppHandle) -> Result<CustomResponse, String> {
    tauri::async_runtime::spawn(async move {
        let queue: State<'_, Queue> = handle.state();
        queue.stop_task().await;
    });
    return Ok(CustomResponse {
        message: String::from("任务已取消！"),
        data: None,
    });
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchRequest {
    pub key: Option<String>,
}