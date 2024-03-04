use serde_json::json;
use tauri::AppHandle;

use crate::{
    models::web_response::WebResponse,
    services::{log::log_error, yapi::category::fetch_cat_interface_list},
};

#[tauri::command]
pub async fn get_cat_interface_list(
    app_handle: AppHandle,
    token: &str,
    cat_id: u32,
    source_path: &str,
) -> Result<WebResponse, String> {
    match fetch_cat_interface_list(cat_id, token.to_string(), source_path, &app_handle).await {
        Ok(res) => Ok(WebResponse {
            data: Some(json!(res)),
            message: format!("获取分类{}的接口信息成功", cat_id),
        }),
        Err(err) => log_error(&app_handle, err.to_string()),
    }
}
