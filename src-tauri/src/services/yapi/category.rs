use std::io;

use tauri::AppHandle;

use crate::{
    models::{
        notification::NotificationDesc,
        yapi::{category::CategoryDataList, web_response::YapiResponse},
    },
    services::{
        notification::notification,
        reqwest::{get_data, get_reqwest_client},
    },
};

use super::config::get_project_config;

const INTERFACE_LIST_CAT_API: &str = "api/interface/list_cat";

pub async fn fetch_cat_interface_list(
    cat_id: u32,
    token: String,
    source_path: String,
    app_handle: &AppHandle,
) -> Result<CategoryDataList, io::Error> {
    let client = get_reqwest_client(&app_handle)?;
    let project_config = get_project_config(source_path)?;

    let url = format!(
        "{}{}?token={}&catid={}&limit=1000",
        project_config.base_url, INTERFACE_LIST_CAT_API, token, cat_id
    );

    // notification(
    //     &app_handle,
    //     NotificationDesc::Success,
    //     "正在获取分类下的接口列表,,,",
    // );

    match get_data::<YapiResponse<CategoryDataList>>(client, url).await {
        Ok(res) => Ok(res.data),
        Err(err) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("获取分类下的接口列表失败: {}", err.to_string()),
        )),
    }
}
