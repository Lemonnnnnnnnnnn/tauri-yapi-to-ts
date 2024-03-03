use serde_json::json;
use tauri::{AppHandle, Manager};

use crate::{
    models::{global_config::{self, GlobalConfigRequest}, web_response::WebResponse},
    services::{
        global_config::{get_global_config, get_latest_project_source_path, update_project, write_config},
        log::{log, log_error},
        yapi::config::get_project_config,
    },
};

// 初始化
#[tauri::command]
pub fn add_project(source_path: &str, app_handle: AppHandle) -> Result<WebResponse, String> {
    match get_global_config(&app_handle) {
        Ok(mut global_config) => {
            match update_project(&app_handle, &source_path.to_string(), &mut global_config) {
                Ok(_) => {
                    app_handle
                        .emit_all("load_project", source_path.to_string())
                        .unwrap();
                    Ok(WebResponse {
                        message: String::from("成功添加工程！"),
                        data: None,
                    })
                }
                Err(e) => log_error(&app_handle, e.to_string()),
            }
        }
        Err(e) => log_error(&app_handle, e.to_string()),
    }
}

#[tauri::command]
pub fn load_latest_project(app_handle: tauri::AppHandle) -> Result<WebResponse, String> {
    match get_latest_project_source_path(&app_handle) {
        Ok(source_path) => {
            app_handle.emit_all("load_project", source_path).unwrap();
            Ok(WebResponse {
                message: String::from("获取最新项目成功！"),
                data: None,
            })
        }
        Err(e) => {
            app_handle
                .emit_all("load_project_error", Some(e.to_string()))
                .unwrap();
            log_error(&app_handle, e.to_string())
        }
    }
}

#[tauri::command]
pub fn update_config(
    data: GlobalConfigRequest,
    app_handle: AppHandle,
) -> Result<WebResponse, String> {
    let config = get_global_config(&app_handle);
    match config { 
        Ok(mut global_config) => {
            global_config.merge_from_request(data);

            match write_config(&app_handle, &global_config) {
                Ok(_) => Ok(WebResponse {
                    message: String::from("更新成功！"),
                    data: None,
                }),
                Err(e) => log_error(&app_handle, e.to_string()),
            }
        }
        Err(e) => log_error(&app_handle, e.to_string()),
    }
}
