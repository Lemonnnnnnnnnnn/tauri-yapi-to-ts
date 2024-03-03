use tauri::AppHandle;

use crate::{
    models::{
        web_response::WebResponse,
        yapi::config::{YapiConfig, YapiConfigRequest},
    },
    services::{
        log::{log, log_error},
        yapi::config::{get_project_config, init_project_config, write_project_config},
    },
};

// #[tauri::command]
// pub fn init_project_config(
//     source_path: &str,
//     app_handle: AppHandle,
// ) -> Result<WebResponse, String> {
//     match write_project_config(source_path.to_string(), YapiConfig::default()) {
//         Ok(_) => Ok(WebResponse {
//             message: String::from("初始化成功！"),
//             data: None,
//         }),
//         Err(e) => log_error(&app_handle, e.to_string()),
//     }
// }

#[tauri::command]
pub fn update_project_config(
    source_path: &str,
    data: YapiConfigRequest,
    app_handle: AppHandle,
) -> Result<WebResponse, String> {
    let config = get_project_config(source_path.to_string());
    match config {
        Ok(mut config) => {
            config.merge_from_request(data);

            match write_project_config(source_path.to_string(), config) {
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
pub fn load_project(source_path: &str, app_handle: AppHandle) -> Result<WebResponse, String> {
    match get_project_config(source_path.to_string()) {
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
