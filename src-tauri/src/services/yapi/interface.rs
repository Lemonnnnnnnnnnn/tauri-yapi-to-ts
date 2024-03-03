use std::io;

use serde_json::json;
use tauri::AppHandle;

use crate::{
    models::{
        notification::NotificationDesc,
        yapi::{
            interface::{FormType, InterfaceData, InterfaceFetchParams, WebType},
            web_response::YapiResponse,
        },
    },
    services::{
        notification::notification,
        reqwest::{get_data, get_reqwest_client},
    },
};

use super::{
    config::get_project_config,
    resolver::{
        common::{get_req_body_type, get_request_json},
        form_resolver, json_resolver,
    },
};

const add_interface_task_API: &str = "api/interface/get";

pub async fn fetch_interface_detail(
    fetch_interface_params: InterfaceFetchParams,
    app_handle: &AppHandle,
) -> Result<InterfaceData, io::Error> {
    let client = get_reqwest_client(&app_handle)?;
    let project_config = get_project_config(fetch_interface_params.source_path)?;

    let url = format!(
        "{}{}?token={}&id={}",
        project_config.base_url,
        add_interface_task_API,
        fetch_interface_params.token,
        fetch_interface_params.interface_id
    );

    // notification(
    //     &fetch_interface_params.app_handle,
    //     NotificationDesc::Success,
    //     "正在获取接口详情...",
    // );

    match get_data::<YapiResponse<InterfaceData>>(client, url).await {
        Ok(res) => Ok(res.data),
        Err(err) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("获取接口详情失败: {}", err.to_string()),
        )),
    }
}

pub fn get_interface_ts_string(data: &InterfaceData) -> Result<String, String> {
    if let Err(e) = is_legal(data) {
        return Err(e);
    }

    let resp_ts_string = json_resolver::get_ts_string(
        WebType::Response,
        data,
        &json!(data.res_body.clone().unwrap_or("".to_string())),
    );
    let req_ts_string;

    let request_json = json!(get_request_json(data));

    if data.method == "POST" {
        // post请求可能是json也可能是form
        match get_req_body_type(data) {
            FormType::Form => {
                req_ts_string = form_resolver::get_ts_string(WebType::Request, data, &request_json)
            }
            FormType::Json => {
                req_ts_string = json_resolver::get_ts_string(WebType::Request, data, &request_json)
            }
        };
    } else {
        req_ts_string = form_resolver::get_ts_string(WebType::Request, data, &request_json)
    }

    Ok(format!("{}\n{}", req_ts_string, resp_ts_string))
}

fn is_legal(data: &InterfaceData) -> Result<(), String> {
    if data.res_body.is_none() {
        return Err("接口响应体为空".to_string());
    }
    Ok(())
}
