use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use tauri::{Manager, State};

use crate::structs::{
    context::ContextGlobal,
    request::{Request, RequestNode},
};

use super::common::{CustomResponse, SearchRequest};

#[tauri::command]
pub async fn check_request_config(handle: tauri::AppHandle) -> Result<CustomResponse, String> {
    let context_global: State<'_, ContextGlobal> = handle.state();
    let source_path = context_global.source_path.lock().await.clone().unwrap();

    let request = Request::from_source_path(source_path);

    if request.request_full_path.is_none()
        || request.header_template.is_none()
        || request.request_template.is_none()
        || request.type_import_template.is_none()
    {
        return Err("请初始化数据".to_string());
    }

    Ok(CustomResponse {
        message: "已读取配置".to_string(),
        data: None,
    })
}

#[tauri::command]
pub async fn get_request_list(
    data: String,
    handle: tauri::AppHandle,
) -> Result<CustomResponse, String> {
    let params = from_str::<SearchRequest>(&data).unwrap();
    let context_global: State<'_, ContextGlobal> = handle.state();
    let source_path = context_global.source_path.lock().await.clone().unwrap();

    let request = Request::from_source_path(source_path);

    let list = request.get_node_list(None);

    let mut root_node = RequestNode {
        full_path: request.context.types_full_path.clone().unwrap(),
        name: "root".to_string(),
        children: list,
    };

    match params.key {
        Some(key) => {
            if !key.is_empty() {
                filter_request_list(&mut root_node, key);
            }
        }
        None => {}
    }

    Ok(CustomResponse {
        message: "获取成功".to_string(),
        data: Some(json!(Vec::from([root_node]))),
    })
}

fn filter_request_list(node: &mut RequestNode, key: String) -> bool {
    if node.children.len() == 0 {
        if node.name.contains(&key) {
            true
        } else {
            false
        }
    } else {
        node.children
            .retain_mut(|sub_node| filter_request_list(sub_node, key.clone()));

        if node.children.len() == 0 {
            false
        } else {
            true
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateRequest {
    full_path: Vec<String>,
}

#[tauri::command]
pub async fn update_request(
    data: String,
    handle: tauri::AppHandle,
) -> Result<CustomResponse, String> {
    let form = from_str::<UpdateRequest>(&data).unwrap();
    let context_global: State<'_, ContextGlobal> = handle.state();
    let source_path = context_global.source_path.lock().await.clone().unwrap();

    let request = Request::from_source_path(source_path);

    for fp in form.full_path {
        let path = PathBuf::from(fp);
        request.write_ts(&path);
    }

    Ok(CustomResponse {
        message: "已成功写入ts文件".to_string(),
        data: None,
    })
}
