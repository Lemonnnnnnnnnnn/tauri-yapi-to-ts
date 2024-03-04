use serde_json::json;
use tauri::AppHandle;

use crate::{
    models::{web_response::WebResponse, yapi::config::YapiProject},
    services::{
        log::{log, log_error},
        yapi::{
            config::{get_project_config, write_project_config},
            project::{fetch_project_base_info, fetch_project_cat_menu},
        },
    },
};

#[tauri::command]
pub async fn get_yapi_project_base_info(
    app_handle: AppHandle,
    token: &str,
    source_path: &str,
) -> Result<WebResponse, String> {
    match fetch_project_base_info(token.to_string(), source_path, &app_handle).await {
        Ok(yapi_project_base_info) => {
            match get_project_config(source_path) {
                Ok(mut config) => {
                    // if config.project_list has not new project, add its to config
                    if !config.project_list.iter().any(|project| {
                        project.project_id == format!("{}", yapi_project_base_info._id)
                    }) {
                        config.project_list.push(YapiProject {
                            token: token.to_string(),
                            project_id: format!("{}", yapi_project_base_info._id),
                            project_name: Some(yapi_project_base_info.name.clone()),
                            categories: vec![],
                        });

                        // write config
                        match write_project_config(source_path, config) {
                            Ok(_) => {}
                            Err(e) => {
                                log(&app_handle, e.to_string());
                            }
                        };
                    }
                }
                Err(e) => {
                    log(&app_handle, e.to_string());
                }
            }
            Ok(WebResponse {
                message: format!("获取yapi项目{}成功", yapi_project_base_info.name),
                data: Some(json!(yapi_project_base_info)),
            })
        }
        Err(err) => log_error(&app_handle, err.to_string()),
    }
}

#[tauri::command]
pub async fn get_yapi_project_cat_menu(
    app_handle: AppHandle,
    token: &str,
    project_id: u32,
    source_path: &str,
) -> Result<WebResponse, String> {
    match fetch_project_cat_menu(project_id, token.to_string(), source_path, &app_handle).await {
        Ok(res) => Ok(WebResponse {
            message: "获取yapi项目分类菜单成功".to_string(),
            data: Some(json!(res)),
        }),
        Err(err) => log_error(&app_handle, err.to_string()),
    }
}
