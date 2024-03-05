use std::{fs, io, path::PathBuf};

use tauri::AppHandle;

use crate::{
    models::yapi::{
        interface::{FormType, InterfaceData, InterfaceFetchParams, WebType},
        web_response::YapiResponse,
    },
    services::reqwest::{get_data, get_reqwest_client},
};

use super::{
    config::get_project_config,
    resolver::{
        common::{get_json, get_path_arr, get_req_body_type, get_request_json},
        form_resolver, json_resolver,
    },
};

const ADD_INTERFACE_TASK_API: &str = "api/interface/get";

// 获取接口详情
pub async fn fetch_interface_detail(
    fetch_interface_params: InterfaceFetchParams,
    app_handle: &AppHandle,
) -> Result<InterfaceData, io::Error> {
    let client = get_reqwest_client(&app_handle)?;
    let project_config = get_project_config(&fetch_interface_params.source_path)?;

    let url = format!(
        "{}{}?token={}&id={}",
        project_config.base_url,
        ADD_INTERFACE_TASK_API,
        fetch_interface_params.token,
        fetch_interface_params.interface_id
    );

    match get_data::<YapiResponse<InterfaceData>>(client, url).await {
        Ok(res) => Ok(res.data),
        Err(err) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("获取接口详情失败: {}", err.to_string()),
        )),
    }
}

// 接口转ts字符串
pub fn get_interface_ts_string(data: &InterfaceData) -> Result<String, String> {
    if let Err(e) = is_legal(data) {
        return Err(e);
    }

    let resp_ts_string = json_resolver::get_ts_string(
        WebType::Response,
        data,
        &get_json(data.res_body.clone().unwrap_or("".to_string())),
    );
    let req_ts_string;

    let request_json = get_json(get_request_json(data));

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

pub fn write_content_to_interface_path(
    path: String,
    source_path: &str,
    content: String,
) -> Result<(), io::Error> {
    let project_config = get_project_config(source_path)?;
    let path_arr = get_path_arr(path);
    let mut file_path = PathBuf::from(source_path).join(project_config.types_path);

    for p in path_arr {
        file_path.push(p);
    }

    let dir_path = file_path.parent().unwrap();

    fs::create_dir_all(dir_path).unwrap();

    let file_full_name = format!("{}.ts", file_path.to_str().unwrap().to_string());

    fs::write(file_full_name, content).unwrap();

    Ok(())
}
