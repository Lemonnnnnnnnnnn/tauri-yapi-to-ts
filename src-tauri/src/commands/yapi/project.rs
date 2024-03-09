use serde_json::json;
use tauri::AppHandle;

use crate::{
    models::web_response::WebResponse,
    services::{
        log::{log, log_error},
        yapi::{
            config::{merge_category_to_project_config, merge_yapi_project_to_project_config},
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
            match merge_yapi_project_to_project_config(source_path, &yapi_project_base_info, token)
            {
                Ok(_) => {
                    log(
                        &app_handle,
                        format!("更新项目{}至配置文件成功", yapi_project_base_info.name),
                    );
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
        Ok(res) => {
            for category in &res {
                match merge_category_to_project_config(
                    category,
                    source_path,
                    &project_id.to_string(),
                ) {
                    Ok(_) => {
                        log(
                            &app_handle,
                            format!("更新类目{}至配置文件成功", category.name),
                        );
                    }
                    Err(e) => {
                        log(&app_handle, e.to_string());
                    }
                }
            }
            Ok(WebResponse {
                message: "获取yapi项目分类菜单成功".to_string(),
                data: Some(json!(res)),
            })
        }
        Err(err) => log_error(&app_handle, err.to_string()),
    }
}
