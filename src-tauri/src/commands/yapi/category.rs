use serde_json::json;
use tauri::{AppHandle, Manager, State};

use crate::{
    models::{
        notification::NotificationDesc,
        web_response::WebResponse,
        yapi::{interface::InterfaceFetchParams, queue::Queue},
    },
    services::{
        log::log_error, notification::notification, yapi::category::fetch_cat_interface_list,
    },
};

#[tauri::command]
pub async fn get_cat_interface_list(
    app_handle: AppHandle,
    token: &str,
    cat_id: u32,
    source_path: &str,
) -> Result<WebResponse, String> {
    // let queue: State<'_, Queue> = app_handle.state();

    // notification(
    //     &app_handle,
    //     NotificationDesc::Success,
    //     &format!("正在获取分类{}的接口信息...", cat_id),
    // );
    match fetch_cat_interface_list(
        cat_id,
        token.to_string(),
        source_path.to_string(),
        &app_handle,
    )
    .await
    {
        Ok(res) => {
            Ok(WebResponse {
                data: Some(json!(res)),
                message: format!("获取分类{}的接口信息成功", cat_id),
            })

            // for item in res.list {
            //     queue
            //         .add_task(InterfaceFetchParams {
            //             interface_id: item._id,
            //             token: token.to_string(),
            //             app_handle: app_handle.clone(),
            //             source_path: source_path.to_string(),
            //         })
            //         .await;
            // }

            // match queue.start_execute(&app_handle).await {
            //     Ok(_) => Ok(WebResponse {
            //         data: None,
            //         message: format!("已将分类{}下的所有接口加入任务队列", cat_id),
            //     }),
            //     Err(err) => log_error(&app_handle, err.to_string()),
            // }
        }
        Err(err) => log_error(&app_handle, err.to_string()),
    }
}
