use serde_json::json;
use tauri::AppHandle;

use crate::{
    models::web_response::WebResponse,
    services::{log::log_error, request::get_file_tree, yapi::config::get_project_config},
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
