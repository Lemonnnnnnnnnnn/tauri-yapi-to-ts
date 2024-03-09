use serde_json::{json, Value};

use crate::models::yapi::interface::{FormType, InterfaceData};


pub fn get_legal_name(raw_name: &str) -> String {
    let chars = raw_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<Vec<_>>();

    String::from_iter(chars)
}

pub fn get_legal_desc(raw_desc: &str) -> String {
    raw_desc.replace("\n", "").to_string()
}

pub fn get_desc(value: &Value, key: &str) -> String {
    let desc = match value.get(key) {
        Some(desc) => match desc.as_str() {
            Some(desc_str) => get_legal_desc(desc_str),
            None => String::from("无注释"),
        },
        None => String::from("无注释"),
    };

    desc
}

pub fn get_json(json_str: String) -> serde_json::Value {
    match serde_json::from_str(&json_str) {
        Ok(json) => json,
        Err(_) => serde_json::Value::Null,
    }
}

// 解析请求体json字符串
pub fn get_request_json(interface_data: &InterfaceData) -> String {
    match get_req_body_type(interface_data) {
        FormType::Form => {
            let form = &interface_data.req_body_form;

            if let Some(res) = form {
                if res.len() != 0 {
                    return json!(res).to_string();
                }
            }

            let req_query = &interface_data.req_query;

            if let Some(res) = req_query {
                if res.len() != 0 {
                    return json!(res).to_string();
                }
            }

            let req_params = &interface_data.req_params;
            if let Some(res) = req_params {
                if res.len() != 0 {
                    return json!(res).to_string();
                }
            }

            String::from("")
        }
        FormType::Json => {
            let json = &interface_data.req_body_other;

            if let Some(res) = json {
                res.clone()
            } else {
                String::from("")
            }
        }
    }
}

// 解析请求体类型
pub fn get_req_body_type(interface_data: &InterfaceData) -> FormType {
    match &interface_data.req_body_type {
        Some(req_body_type) => {
            if req_body_type.as_str() == "form" {
                FormType::Form
            } else {
                FormType::Json
            }
        }
        None => FormType::Form,
    }
}

// 将接口路径转换为数组形式
pub fn get_path_arr(raw_path: String) -> Vec<String> {
    let path_arr: Vec<_> = raw_path
        .split("/")
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();
    path_arr
}

