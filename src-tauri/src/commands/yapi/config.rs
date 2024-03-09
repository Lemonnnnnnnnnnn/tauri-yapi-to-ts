use tauri::AppHandle;

use crate::{
    models::{web_response::WebResponse, yapi::config::YapiConfigRequest},
    services::{
        log::log_error,
        yapi::config::{
            get_project_config, init_project_config, merge_config_projects, write_project_config,
        },
    },
};

#[tauri::command]
pub fn update_project_config(
    source_path: &str,
    data: YapiConfigRequest,
    app_handle: AppHandle,
) -> Result<WebResponse, String> {
    let config = get_project_config(source_path);
    match config {
        Ok(mut config) => {
            config.merge_from_request(data);

            match write_project_config(source_path, config) {
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

#[tauri::command]
pub fn load_project_config(
    source_path: &str,
    app_handle: AppHandle,
) -> Result<WebResponse, String> {
    match get_project_config(source_path) {
        Ok(config) => {
            if config.base_url.is_empty() {
                return Err("请初始化配置".to_string());
            }
            Ok(WebResponse {
                message: String::from("获取项目成功！"),
                data: Some(config.into()),
            })
        }
        Err(e) => {
            if let Err(e) = init_project_config(source_path.to_string()) {
                return Err(format!("初始化配置文件失败，错误信息：{}", e.to_string()));
            }
            log_error(&app_handle, format!("获取项目失败！ {}", e.to_string()))
        }
    }
}

#[tauri::command]
pub fn merge_project_config(
    source_path: &str,
    other_config_path: &str,
    app_handle: AppHandle
) -> Result<WebResponse, String> {
    match merge_config_projects(source_path, other_config_path) {
        Ok(_) => Ok(WebResponse {
            message: String::from("合并成功！"),
            data: None,
        }),
        Err(e) => log_error(&app_handle, e.to_string()),
    }
}
