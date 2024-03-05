use serde_json::json;
use tauri::AppHandle;

use crate::{
    models::web_response::WebResponse,
    services::{
        log::{log, log_error},
        yapi::{category::fetch_cat_interface_list, config::merge_interface_to_project_config},
    },
};

#[tauri::command]
pub async fn get_cat_interface_list(
    app_handle: AppHandle,
    token: &str,
    cat_id: u32,
    source_path: &str,
) -> Result<WebResponse, String> {
    match fetch_cat_interface_list(cat_id, token.to_string(), source_path, &app_handle).await {
        Ok(res) => {
            for interface in &res.list {
                match merge_interface_to_project_config(interface, source_path, &cat_id.to_string())
                {
                    Ok(_) => {
                        log(
                            &app_handle,
                            format!("分类{}的接口{}合并到配置文件成功", cat_id, interface._id),
                        );
                    }
                    Err(err) => {
                        log(&app_handle, err.to_string());
                    }
                }
            }

            Ok(WebResponse {
                data: Some(json!(res)),
                message: format!("获取分类{}的接口信息成功", cat_id),
            })
        }
        Err(err) => log_error(&app_handle, err.to_string()),
    }
}
