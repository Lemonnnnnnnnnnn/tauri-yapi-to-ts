use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use tauri::{Manager, State};

use crate::{
    structs::{
        config::{CategoryShort, Config, ConfigJson, ProjectShort},
        context::ContextGlobal,
        queue::Queue,
    },
    utils::join_path,
};

use super::common::{CustomResponse, SearchRequest};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetConfigRequest {
    pub dir_path: String,
}

// 读取配置(初始化)
#[tauri::command]
pub async fn read_config(
    data: String,
    handle: tauri::AppHandle,
) -> Result<CustomResponse, String> {
    let cx: State<'_, ContextGlobal> = handle.state();
    let params = from_str::<GetConfigRequest>(&data).unwrap();
    let queue: State<'_, Queue> = handle.state();
    let dir_path = PathBuf::from(&params.dir_path);

    let config: Config = Config::new(&dir_path);
    // 更新queue基础配置
    init_queue(&queue, &config).await;
    // 更新context基础配置
    cx.update(dir_path).await;

    match config.read_config() {
        Ok(data) => {
            if data.base_url.is_some() {
                update_queue(&queue, &data).await;
                return Ok(CustomResponse {
                    message: String::from("读取配置成功！"),
                    data: None,
                });
            } else {
                return Err("数据为空！".to_string());
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
}


// 获取配置
#[tauri::command]
pub async fn get_config(data: String, handle: tauri::AppHandle) -> Result<CustomResponse, String> {
    let params = from_str::<SearchRequest>(&data).unwrap();
    let cx: State<'_, ContextGlobal> = handle.state();
    let path = cx.source_path.lock().await.clone();

    if let Some(path) = &path {
        let config = Config::new(&path);

        match config.read_config() {
            Ok(mut data) => {
                match params.key {
                    Some(key) => {
                        if !key.is_empty() {
                            data.project_list
                                .retain_mut(|project| filter_categories(project, &key));
                        }
                    }
                    None => {}
                }
                Ok(CustomResponse {
                    message: String::from("获取配置成功！"),
                    data: Some(json!(data)),
                })
            }
            Err(_) => {
                return Err(String::from("read config error"));
            }
        }
    } else {
        return Err(String::from("path is None"));
    }
}

fn filter_categories(project: &mut ProjectShort, key: &String) -> bool {
    project.categories.retain_mut(|category| {
        if category.name.contains(key) {
            true
        } else {
            filter_interfaces(category, key)
        }
    });

    if project.categories.len() != 0 {
        true
    } else {
        false
    }
}

fn filter_interfaces(category: &mut CategoryShort, key: &String) -> bool {
    category.interfaces.retain_mut(|interface| {
        if let Some(name) = &interface.name {
            if name.contains(key) {
                return true;
            }
        }

        if let Some(path) = &interface.path {
            if path.contains(key) {
                return true;
            }
        }

        false
    });

    if category.interfaces.len() != 0 {
        true
    } else {
        false
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateConfigRequest {
    pub source_path: Option<String>,
    pub base_url: Option<String>,
    pub types_path: Option<String>,
    pub rate_limit: Option<usize>,
    pub break_seconds: Option<u64>,
    pub request_template: Option<String>,
    pub header_template: Option<String>,
    pub request_path: Option<String>,
    pub file_name_template: Option<String>
}

// 更新类型配置
#[tauri::command]
pub async fn update_config(
    data: String,
    handle: tauri::AppHandle,
) -> Result<CustomResponse, String> {
    let form = from_str::<UpdateConfigRequest>(&data).unwrap();
    let cx: State<'_, ContextGlobal> = handle.state();
    let path = cx.source_path.lock().await.clone().unwrap();
    let queue: State<'_, Queue> = handle.state();

    let config = Config::new(&path);

    let mut config_json = match config.read_config() {
        Ok(data) => data,
        Err(_) => ConfigJson::new(path.clone()),
    };

    if let Some(base_url) = &form.base_url {
        let mut url = base_url.clone();
        if url.ends_with("/") {
            url.pop();
        }
        config_json.base_url = Some(url);
    }

    if let Some(types_path) = &form.types_path {
        config_json.types_path = Some(PathBuf::from(types_path));
        config_json.types_full_path = Some(join_path(
            &config_json.source_path.clone().unwrap(),
            types_path.clone(),
        ));
    }

    if let Some(rate_limit) = &form.rate_limit {
        config_json.rate_limit = Some(rate_limit.clone());
    }

    if let Some(break_seconds) = &form.break_seconds {
        config_json.break_seconds = Some(break_seconds.clone());
    }

    if let Some(request_template) = &form.request_template {
        config_json.request_template = Some(request_template.clone());
    }

    if let Some(header_template) = &form.header_template {
        config_json.header_template = Some(header_template.clone());
    }

    if let Some(file_name_template) = &form.file_name_template {
        config_json.file_name_template = Some(file_name_template.clone());
    }

    if let Some(request_path) = &form.request_path {
        config_json.request_path = Some(PathBuf::from(request_path));
        config_json.request_full_path = Some(join_path(
            &config_json.source_path.clone().unwrap(),
            request_path.clone(),
        ));
    }

    update_queue(&queue, &config_json).await;

    config.write_config(&config_json);

    return Ok(CustomResponse {
        message: String::from("更新配置成功！"),
        data: None,
    });
}

async fn init_queue(queue: &State<'_, Queue>, config: &Config) {
    let mut queue_config = queue.config.lock().await;
    *queue_config = Some(config.clone());
}

async fn update_queue(queue: &State<'_, Queue>, config: &ConfigJson) {
    *queue.break_seconds.lock().await = match config.break_seconds {
        Some(val) => val,
        None => 2,
    };
    *queue.rate_limit.lock().await = match config.rate_limit {
        Some(val) => val,
        None => 0,
    };
}
