use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use tauri::{Manager, State};

use crate::structs::{context::{Context, ContextGlobal}, interface::Interface, queue::Queue};

use super::common::CustomResponse;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InterfaceList {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateInterfaces {
    pub interfaces: Vec<InterfaceList>,
    pub token: String,
}

// 更新接口
#[tauri::command]
pub async fn update_interface(
    data: String,
    handle: tauri::AppHandle,
) -> Result<CustomResponse, String> {
    let context_global: State<'_, ContextGlobal> = handle.state();
    let queue: State<'_, Queue> = handle.state();
    let handle_clone = handle.clone();

    let source_path = context_global.source_path.lock().await.clone().unwrap();
    let context = Context::from_source_path(&source_path);

    let data = from_str::<UpdateInterfaces>(&data).unwrap();
    let token = data.token;

    let interfaces: Vec<_> = data
        .interfaces
        .iter()
        .map(|item| Interface::new(token.clone(), context.clone(), item.id.to_string()))
        .collect();

    add_interfaces_task(&queue, interfaces).await;

    tauri::async_runtime::spawn(async move {
        let queue: State<'_, Queue> = handle_clone.state();
        queue.start_task().await;
    });

    return Ok(CustomResponse {
        message: String::from("添加任务成功"),
        data: Some(json!(queue.total_count.lock().await.clone())),
    });
}

pub async fn add_interfaces_task(queue: &State<'_, Queue>, interfaces: Vec<Interface>) {
    for interface in interfaces {
        queue.add_task(interface).await;
    }
}
