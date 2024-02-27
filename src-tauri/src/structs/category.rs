use std::path::PathBuf;

use crate::utils::get_err;

use super::context::Context;
use super::interface::Interface;
use super::web_response::{CategoryDataList, CommonResponse};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
type Response = CommonResponse<CategoryDataList>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    token: String,
    context: Context,
    pub id: String,
    pub name: String,
    pub interfaces: Vec<Interface>,
}

impl Category {
    pub fn new(token: String, context: Context, id: String, name: String) -> Self {
        Category {
            token,
            context,
            id,
            name,
            interfaces: Vec::new(),
        }
    }

    pub fn from_source_path(token: String, id: String, name: String, source_path: PathBuf) -> Self {
        let context = Context::from_source_path(&source_path);
        Category {
            token,
            context,
            id,
            name,
            interfaces: Vec::new(),
        }
    }

    // 获取 interfaces
    pub async fn fetch_interfaces(&self) -> Result<Vec<Interface>, String> {
        let interface_list_res = self.get_interface_list().await;

        match interface_list_res {
            Ok(interface_list) => {
                let interfaces: Vec<_> = interface_list
                    .list
                    .iter()
                    .map(|item| {
                        Interface::new(
                            self.token.clone(),
                            self.context.clone(),
                            item._id.to_string(),
                        )
                    })
                    .collect();

                return Ok(interfaces);
            }
            Err(e) => return Err(e),
        }
    }

    // 获取分类下的所有接口
    async fn get_interface_list(&self) -> Result<CategoryDataList, String> {
        let api_url = format!(
            "{}/api/interface/list_cat?token={}&catid={}&limit=1000",
            self.context.base_url.clone().unwrap(),
            self.token.clone(),
            self.id
        );

        let proxy = if let Some(proxy) = self.context.proxy.clone() {
            if !proxy.is_empty() {
                Some(reqwest::Proxy::all(proxy).unwrap())
            } else {
                None
            }
        } else {
            None
        };
        let client = if let Some(proxy) = proxy.clone() {
            reqwest::Client::builder().proxy(proxy).build().unwrap()
        } else {
            reqwest::Client::new()
        };

        let res = client
            .get(api_url.clone())
            .header("ACCEPT", "application/json")
            .send()
            .await;

        match res {
            Ok(response) => {
                let to_json = response.json::<Value>().await;

                match to_json {
                    Ok(json) => {
                        let (err_code, err_msg) = get_err(&json);

                        if err_code != 0 {
                            panic!("{err_msg}");
                        }

                        let raw_data = from_value::<Response>(json).unwrap();

                        return Ok(raw_data.data);
                    }
                    Err(_) => {
                        return Err(format!(
                            "category {} transform to json error",
                            self.id.clone()
                        ))
                    }
                }
            }
            Err(e) => {
                return Err(format!(
                    "fetch category {} error , detail: {}",
                    self.id.clone(),
                    e.to_string()
                ));
            }
        }
    }
}

// #[cfg(test)]
// mod test {
//     // use crate::structs::project::TEMP_TOKEN;

//     // use super::super::context::get_test_context;
//     // use super::*;

//     #[tokio::test]
//     #[ignore]
//     async fn test() -> Result<(), Box<dyn std::error::Error>> {
//         // let category_id = String::from("4796");
//         // let context = get_test_context();
//         // let name = String::from("a");

//         // Category::new(TEMP_TOKEN.into(), context, category_id, name)
//         //     .fetch_interfaces()
//         //     .await
//         //     .fetch_interface_detail()
//         //     .await
//         //     .write_ts_file();

//         Ok(())
//     }
// }
