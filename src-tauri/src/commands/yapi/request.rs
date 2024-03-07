use std::path::PathBuf;

use serde_json::json;
use tauri::AppHandle;

use crate::{
    models::web_response::WebResponse,
    services::{
        log::log_error,
        request::{get_file_tree, get_request_ts_string, write_request_ts_file},
        yapi::config::get_project_config,
    },
};

#[tauri::command]
pub fn load_file_tree(source_path: &str, app_handle: AppHandle) -> Result<WebResponse, String> {
    match get_file_tree(source_path, None) {
        Ok(file_tree) => Ok(WebResponse {
            data: Some(json!(file_tree)),
            message: "获取types文件树成功".to_string(),
        }),
        Err(err) => log_error(&app_handle, err.to_string()),
    }
}

#[tauri::command]
pub fn get_request_string(
    source_path: &str,
    app_handle: AppHandle,
    path: &str,
) -> Result<WebResponse, String> {
    match get_request_ts_string(source_path, &PathBuf::from(path)) {
        Ok(request_string) => Ok(WebResponse {
            data: Some(json!(request_string)),
            message: "获取请求字符串成功".to_string(),
        }),
        Err(err) => log_error(&app_handle, err.to_string()),
    }
}

#[tauri::command]
pub fn write_request_to_file(
    source_path: &str,
    app_handle: AppHandle,
    path: &str,
    content: String,
) -> Result<WebResponse, String> {
    match write_request_ts_file(source_path, &PathBuf::from(path), content) {
        Ok(_) => Ok(WebResponse {
            data: None,
            message: "写入文件成功".to_string(),
        }),
        Err(err) => log_error(&app_handle, err.to_string()),
    }
}
