use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use tauri::{AppHandle, Manager, State};

use crate::{
    models::global_config::{self, GlobalConfig},
    structs::{
        config::{CategoryShort, Config, ConfigJson, ProjectShort},
        context::ContextGlobal,
        queue::Queue,
    },
    utils::join_path,
};

use super::{common::CustomResponse, log::log, storage::get_local_data_dir};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetConfigRequest {
    pub dir_path: String,
}

// 读取配置(初始化)
#[tauri::command]
pub async fn read_config(data: String, handle: tauri::AppHandle) -> Result<CustomResponse, String> {
    let cx: State<'_, ContextGlobal> = handle.state();
    let params = from_str::<GetConfigRequest>(&data).unwrap();
    let queue: State<'_, Queue> = handle.state();
    let dir_path = PathBuf::from(&params.dir_path);

    let config: Config = Config::new(&dir_path);
    // 更新queue基础配置
    init_queue(&queue, &config).await;
    // 更新context基础配置
    cx.update(dir_path.clone()).await;

    match config.read_config() {
        Ok(mut data) => {
            if data.base_url.is_some() {
                // 更新配置文件 source_path
                data.source_path = Some(dir_path.clone());
                config.write_config(&data);

                update_queue(&queue, &data).await;
                return Ok(CustomResponse {
                    message: String::from("读取配置成功！"),
                    data: None,
                });
            }
            return Err("数据为空！".to_string());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

// 获取配置
// #[tauri::command]
// pub async fn get_global_config(data: String, handle: tauri::AppHandle) -> Result<CustomResponse, String> {
//     let params = from_str::<SearchRequest>(&data).unwrap();
//     let cx: State<'_, ContextGlobal> = handle.state();
//     let path = cx.source_path.lock().await.clone();

//     if let Some(path) = &path {
//         let config = Config::new(&path);

//         match config.read_config() {
//             Ok(mut data) => {
//                 match params.key {
//                     Some(key) => {
//                         if !key.is_empty() {
//                             data.project_list
//                                 .retain_mut(|project| filter_categories(project, &key));
//                         }
//                     }
//                     None => {}
//                 }
//                 Ok(CustomResponse {
//                     message: String::from("获取配置成功！"),
//                     data: Some(json!(data)),
//                 })
//             }
//             Err(_) => {
//                 return Err(String::from("read config error"));
//             }
//         }
//     } else {
//         return Err(String::from("path is None"));
//     }
// }

// fn filter_categories(project: &mut ProjectShort, key: &String) -> bool {
//     project.categories.retain_mut(|category| {
//         if category.name.contains(key) {
//             true
//         } else {
//             filter_interfaces(category, key)
//         }
//     });

//     if project.categories.len() != 0 {
//         true
//     } else {
//         false
//     }
// }

// fn filter_interfaces(category: &mut CategoryShort, key: &String) -> bool {
//     category.interfaces.retain_mut(|interface| {
//         if let Some(name) = &interface.name {
//             if name.contains(key) {
//                 return true;
//             }
//         }

//         if let Some(path) = &interface.path {
//             if path.contains(key) {
//                 return true;
//             }
//         }

//         false
//     });

//     if category.interfaces.len() != 0 {
//         true
//     } else {
//         false
//     }
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateConfigRequest {
    pub source_path: Option<String>,
    pub base_url: Option<String>,
    pub types_path: Option<String>,
    pub rate_limit: Option<usize>,
    pub break_seconds: Option<u64>,
    pub request_template: Option<String>,
    pub header_template: Option<String>,
    pub type_import_template: Option<String>,
    pub proxy: Option<String>,
    pub request_path: Option<String>,
    pub file_name_template: Option<String>,
}

// 更新类型配置
// #[tauri::command]
// pub async fn update_config(
//     data: String,
//     handle: tauri::AppHandle,
// ) -> Result<CustomResponse, String> {
//     let form = from_str::<UpdateConfigRequest>(&data).unwrap();
//     let cx: State<'_, ContextGlobal> = handle.state();
//     let path = cx.source_path.lock().await.clone().unwrap();
//     let queue: State<'_, Queue> = handle.state();

//     let config = Config::new(&path);

//     let mut config_json = match config.read_config() {
//         Ok(data) => data,
//         Err(_) => ConfigJson::new(path.clone()),
//     };

//     if let Some(base_url) = &form.base_url {
//         let mut url = base_url.clone();
//         if url.ends_with("/") {
//             url.pop();
//         }
//         config_json.base_url = Some(url);
//     }

//     if let Some(types_path) = &form.types_path {
//         config_json.types_path = Some(PathBuf::from(types_path));
//         config_json.types_full_path = Some(join_path(
//             &config_json.source_path.clone().unwrap(),
//             types_path.clone(),
//         ));
//     }

//     if let Some(rate_limit) = &form.rate_limit {
//         config_json.rate_limit = Some(rate_limit.clone());
//     }

//     if let Some(break_seconds) = &form.break_seconds {
//         config_json.break_seconds = Some(break_seconds.clone());
//     }

//     if let Some(request_template) = &form.request_template {
//         config_json.request_template = Some(request_template.clone());
//     }

//     if let Some(header_template) = &form.header_template {
//         config_json.header_template = Some(header_template.clone());
//     }

//     if let Some(type_import_template) = &form.type_import_template {
//         config_json.type_import_template = Some(type_import_template.clone());
//     }

//     if let Some(file_name_template) = &form.file_name_template {
//         config_json.file_name_template = Some(file_name_template.clone());
//     }

//     if let Some(request_path) = &form.request_path {
//         config_json.request_path = Some(PathBuf::from(request_path));
//         config_json.request_full_path = Some(join_path(
//             &config_json.source_path.clone().unwrap(),
//             request_path.clone(),
//         ));
//     }

//     if let Some(proxy) = &form.proxy {
//         config_json.proxy = Some(proxy.clone());
//     }

//     update_queue(&queue, &config_json).await;

//     config.write_config(&config_json);

//     return Ok(CustomResponse {
//         message: String::from("更新配置成功！"),
//         data: None,
//     });
// }

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

// ----------------------------

pub const CONFIG_NAME: &str = "config.json";

pub fn init_config(app_handle: &AppHandle) -> Result<(), io::Error> {
    if get_local_data_dir(app_handle)
        .unwrap()
        .join(CONFIG_NAME)
        .exists()
    {
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(get_local_data_dir(app_handle).unwrap().join(CONFIG_NAME))?;
    file.write_all(json!(GlobalConfig::default()).to_string().as_bytes())?;
    Ok(())
}

pub fn get_global_config(app_handle: &AppHandle) -> Result<GlobalConfig, io::Error> {
    let local_data_dir = get_local_data_dir(app_handle);
    if let Some(local_dir) = local_data_dir {
        let mut file = OpenOptions::new()
            .read(true)
            .open(local_dir.join(CONFIG_NAME))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config = from_str::<GlobalConfig>(&contents.to_string()).unwrap();
        Ok(config)
    } else {
        log(app_handle, "local_data_dir is None".to_string());
        Err(io::Error::new(
            io::ErrorKind::Other,
            "local_data_dir is None",
        ))
    }
}

pub fn write_config(app_handle: &AppHandle, contents: &GlobalConfig) -> Result<(), io::Error> {
    let local_data_dir = get_local_data_dir(app_handle);
    if let Some(local_dir) = local_data_dir {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(local_dir.join(CONFIG_NAME))?;
        file.write_all(json!(contents).to_string().as_bytes())?;
        Ok(())
    } else {
        log(app_handle, "local_data_dir is None".to_string());
        Err(io::Error::new(
            io::ErrorKind::Other,
            "local_data_dir is None",
        ))
    }
}

pub fn get_latest_project_source_path(app_handle: &AppHandle) -> Result<String, io::Error> {
    let config = get_global_config(app_handle)?;
    let latest_project_source_path = config.projects.first();
    if let Some(latest_project_source_path) = latest_project_source_path {
        Ok(latest_project_source_path.to_string())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "未找到任何工程项目"))
    }
}

pub fn update_project(
    app_handle: &AppHandle,
    source_path: &String,
    global_config: &mut GlobalConfig,
) -> Result<(), io::Error> {
    if global_config.projects.contains(source_path) {
        // move to first
        let index = global_config
            .projects
            .iter()
            .position(|x| x == source_path)
            .unwrap();
        global_config.projects.remove(index);
        global_config.projects.insert(0, source_path.clone());
        write_config(app_handle, global_config)
    } else {
        // insert to first
        global_config.projects.insert(0, source_path.clone());
        write_config(app_handle, global_config)
    }
}
