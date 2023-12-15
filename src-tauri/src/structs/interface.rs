use std::fs;
use std::path::PathBuf;
// use std::time::Duration;

use crate::utils::get_err;

use super::context::Context;
use super::resolver::enums::{FormType, WebType};
use super::resolver::form_resolver::FormResolver;
use super::resolver::json_resolver::JsonResolver;
use super::web_response::{CommonResponse, InterfaceData};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, Value};
// use tokio::time::sleep;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Interface {
    token: String,
    context: Context,
    pub id: String,
    pub resolved_interface: Option<ResolvedInterface>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResolvedInterface {
    pub request_json: String,
    pub request_form_type: Option<FormType>,
    pub response_json: Option<String>,
    pub id: String,
    pub category_id: String,
    pub path: String,
    pub title: String,
    pub method: String,
}

impl Interface {
    pub fn new(token: String, context: Context, interface_id: String) -> Self {
        Interface {
            token,
            context,
            id: interface_id,
            resolved_interface: None,
        }
    }

    pub fn source_path(token: String, interface_id: String, source_path: PathBuf) -> Self {
        let context = Context::from_source_path(&source_path);
        Interface {
            token,
            context,
            id: interface_id,
            resolved_interface: None,
        }
    }

    pub async fn fetch_detail(&self) -> Result<ResolvedInterface, String> {
        let api_url = format!(
            "{}/api/interface/get?token={}&id={}",
            self.context.base_url.clone().unwrap(),
            self.token.clone(),
            self.id
        );

        let client = reqwest::Client::new();
        let result = client
            .get(api_url.clone())
            .header("ACCEPT", "application/json")
            .send()
            .await;

        match result {
            Ok(response) => {
                match response.json::<Value>().await {
                    Ok(value) => {
                        // println!("{:#?}", value);
                        let (err_code, err_msg) = get_err(&value);

                        if err_code != 0 {
                            panic!("{err_msg}");
                        }

                        let raw_data = from_value::<CommonResponse<InterfaceData>>(value).unwrap();

                        let ri = Self::format(self, raw_data);

                        Ok(ri)
                    }
                    Err(_) => Err(format!(
                        "interface {} tranfer to json error",
                        self.id.clone()
                    )),
                }
            }
            Err(_) => Err(format!("interface {} fetch error", self.id.clone())),
        }
    }

    fn format(&self, raw_data: CommonResponse<InterfaceData>) -> ResolvedInterface {
        let request_json = Self::get_request_json(&raw_data);
        let request_form_type = Self::get_req_body_type(&raw_data);
        let response_json = raw_data.data.res_body;
        let id = raw_data.data._id.to_string();
        let category_id = raw_data.data.catid.to_string();
        let path = raw_data.data.path;
        let title = raw_data.data.title;
        let method = raw_data.data.method;

        ResolvedInterface {
            request_json,
            response_json,
            id,
            category_id,
            path,
            title,
            request_form_type,
            method,
        }
    }

    pub fn write_ts(&self, ri: &ResolvedInterface) -> Result<(), String> {
        // 生成全部文件夹
        let path = ri.path.clone();
        let path_arr = Self::get_path_arr(path);
        let mut file_path = self.context.types_full_path.clone().unwrap();

        for p in path_arr {
            file_path.push(p);
        }

        let interface_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
        let dir_path = file_path.parent().unwrap();

        fs::create_dir_all(dir_path).unwrap();

        // 生成并写入ts
        let req = &ri.request_json;
        let resp = &ri.response_json;
        let interface_desc = &ri.title;
        let req_ts_string: String;

        match Self::is_legal(ri) {
            Ok(_) => {
                // request
                if ri.method == "POST" {
                    let request_form_type = ri.request_form_type.clone();
                    if let FormType::Form = request_form_type.unwrap() {
                        req_ts_string = FormResolver::new(
                            WebType::Request,
                            interface_name.clone(),
                            interface_desc.clone(),
                            req.to_string(),
                        )
                        .to_ts();
                    } else {
                        req_ts_string = JsonResolver::new(
                            WebType::Request,
                            interface_name.clone(),
                            interface_desc.clone(),
                            req.to_string(),
                        )
                        .generate_modal()
                        .to_ts();
                    }
                } else {
                    req_ts_string = FormResolver::new(
                        WebType::Request,
                        interface_name.clone(),
                        interface_desc.clone(),
                        req.to_string(),
                    )
                    .to_ts();
                }

                // response
                let resp_ts_string = JsonResolver::new(
                    WebType::Response,
                    interface_name.clone(),
                    interface_desc.clone(),
                    resp.clone().unwrap(),
                )
                .generate_modal()
                .to_ts();

                let ts_string = format!("{}\n{}", req_ts_string, resp_ts_string);
                let file_name = format!("{}.ts", file_path.to_str().unwrap().to_string());

                fs::write(file_name, ts_string).unwrap();

                Ok(())
            }
            Err(e) => {
                return Err(format!("接口 {}: {}", ri.title, e));
            }
        }
    }

    fn is_legal(ri: &ResolvedInterface) -> Result<(), String> {
        // if ri.request_form_type.is_none() {
        //     return Err("request form type is none".to_string());
        // }
        if ri.response_json.is_none() {
            return Err("response json is none".to_string());
        }
        Ok(())
    }

    fn get_path_arr(raw_path: String) -> Vec<String> {
        let path_arr: Vec<_> = raw_path
            .split("/")
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .collect();
        path_arr
    }

    fn get_request_json(raw_data: &CommonResponse<InterfaceData>) -> String {
        match Self::get_req_body_type(raw_data) {
            Some(req_body_type) => {
                if let FormType::Form = req_body_type {
                    let form = &raw_data.data.req_body_form;

                    if let Some(res) = form {
                        if res.len() != 0 {
                            return json!(res).to_string();
                        }
                    }

                    let req_query = &raw_data.data.req_query;

                    if let Some(res) = req_query {
                        if res.len() != 0 {
                            return json!(res).to_string();
                        }
                    }

                    let req_params = &raw_data.data.req_params;
                    if let Some(res) = req_params {
                        if res.len() != 0 {
                            return json!(res).to_string();
                        }
                    }

                    String::from("")
                } else {
                    let json = &raw_data.data.req_body_other;

                    if let Some(res) = json {
                        return res.clone();
                    } else {
                        return String::from("");
                    }
                }
            }
            None => "".to_string(),
        }
    }

    fn get_req_body_type(raw_data: &CommonResponse<InterfaceData>) -> Option<FormType> {
        match &raw_data.data.req_body_type {
            Some(req_body_type) => {
                if req_body_type.as_str() == "form" {
                    return Some(FormType::Form);
                } else {
                    Some(FormType::Json)
                }
            }
            None => Some(FormType::Form),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::structs::project::TEMP_TOKEN;

//     use super::super::context::get_test_context;
//     use super::*;

//     #[tokio::test]
//     async fn test_json() -> Result<(), Box<dyn std::error::Error>> {
//         let context = get_test_context();
//         let interface_id = String::from("15610");

//         // Interface::new(TEMP_TOKEN.into(), context, interface_id)
//         //     .fetch_detail()
//         //     .await
//         //     .write_ts();

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_form() -> Result<(), Box<dyn std::error::Error>> {
//         let context = get_test_context();
//         let interface_id = String::from("25384");

//         // Interface::new(TEMP_TOKEN.into(), context, interface_id)
//         //     .fetch_detail()
//         //     .await
//         //     .write_ts();

//         Ok(())
//     }
// }
