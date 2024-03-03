use serde_json::json;
use tauri::AppHandle;

use crate::{
    models::{notification::NotificationDesc, web_response::WebResponse},
    services::{
        log::{log, log_error},
        notification::notification,
        yapi::project::{fetch_project_base_info, fetch_project_cat_menu},
    },
};

use super::category::get_cat_interface_list;

#[tauri::command]
pub async fn get_yapi_project_base_info(
    app_handle: AppHandle,
    token: &str,
    source_path: &str,
) -> Result<WebResponse, String> {
    match fetch_project_base_info(token.to_string(), source_path.to_string(), &app_handle).await {
        Ok(yapi_project_base_info) => {
            log(
                &app_handle,
                format!("获取yapi项目{}成功", yapi_project_base_info.name),
            );
            Ok(WebResponse {
                message: format!("获取yapi项目{}成功", yapi_project_base_info.name),
                data: Some(json!(yapi_project_base_info)),
            })
            // match fetch_project_cat_menu(
            //     yapi_project_base_info._id,
            //     token.to_string(),
            //     source_path.to_string(),
            //     &app_handle,
            // )
            // .await
            // {
            //     Ok(cat_menu) => {
            //         for cat in cat_menu {
            //             match get_cat_interface_list(
            //                 app_handle.clone(),
            //                 token,
            //                 cat._id,
            //                 source_path,
            //             )
            //             .await
            //             {
            //                 Ok(res) => {
            //                     log(&app_handle, res.message);
            //                 }
            //                 Err(err) => {
            //                     notification(
            //                         &app_handle,
            //                         NotificationDesc::Error,
            //                         &format!("类目{}更新失败", cat.name),
            //                     );
            //                     log(&app_handle, err);
            //                 }
            //             }
            //         }
            //         return Ok(WebResponse {
            //             message: format!(
            //                 "已将项目{}的所有接口加入任务队列",
            //                 yapi_project_base_info.name
            //             ),
            //             data: None,
            //         });
            //     }
            //     Err(err) => log_error(&app_handle, err.to_string()),
            // }
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
    match fetch_project_cat_menu(
        project_id,
        token.to_string(),
        source_path.to_string(),
        &app_handle,
    )
    .await
    {
        Ok(res) => Ok(WebResponse {
            message: "获取yapi项目分类菜单成功".to_string(),
            data: Some(json!(res)),
        }),
        Err(err) => log_error(&app_handle, err.to_string()),
    }
}
