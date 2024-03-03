use std::io;

use tauri::AppHandle;

use crate::{
    models::{
        notification::NotificationDesc,
        yapi::{
            category::CategoryMenuItem, project::YapiProjectBaseInfo, web_response::YapiResponse,
        },
    },
    services::{
        notification::notification,
        reqwest::{get_data, get_reqwest_client},
    },
};

use super::config::get_project_config;

const PROJECT_BASE_INFO_API: &str = "api/project/get";
const CATEGORY_MENU_API: &str = "api/interface/getCatMenu";

pub async fn fetch_project_base_info(
    token: String,
    source_path: String,
    app_handle: &AppHandle,
) -> Result<YapiProjectBaseInfo, io::Error> {
    let project_config = get_project_config(source_path)?;

    let url = format!(
        "{}{}?token={}",
        project_config.base_url, PROJECT_BASE_INFO_API, token
    );
    let client = get_reqwest_client(&app_handle)?;

    // notification(&app_handle, NotificationDesc::Success, "获取项目信息中...");

    match get_data::<YapiResponse<YapiProjectBaseInfo>>(client, url).await {
        Ok(res) => Ok(res.data),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("获取项目信息失败: {}", e.to_string()),
        )),
    }
}

pub async fn fetch_project_cat_menu(
    project_id: u32,
    token: String,
    source_path: String,
    app_handle: &AppHandle,
) -> Result<Vec<CategoryMenuItem>, io::Error> {
    let client = get_reqwest_client(&app_handle)?;
    let project_config = get_project_config(source_path)?;

    let url = format!(
        "{}{}?project_id={}&token={}",
        project_config.base_url, CATEGORY_MENU_API, project_id, token
    );

    // notification(
    //     &app_handle,
    //     NotificationDesc::Success,
    //     "正在获取分类列表...",
    // );

    match get_data::<YapiResponse<Vec<CategoryMenuItem>>>(client, url).await {
        Ok(res) => Ok(res.data),
        Err(err) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("获取分类列表失败: {}", err.to_string()),
        )),
    }
}
