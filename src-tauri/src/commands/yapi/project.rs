use serde_json::json;
use tauri::AppHandle;

use crate::{
    models::web_response::WebResponse,
    services::{
        log::{log, log_error},
        yapi::project::{fetch_project_base_info, fetch_project_cat_menu},
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
            log(
                &app_handle,
                format!("获取yapi项目{}成功", yapi_project_base_info.name),
            );
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
