use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use tauri::{Manager, State};

use crate::structs::{
    category::Category,
    config::Config,
    context::{Context, ContextGlobal},
    interface::Interface,
    queue::Queue,
};

use super::{common::CustomResponse, interface::add_interfaces_task};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CategoryList {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateCategories {
    pub categories: Vec<CategoryList>,
    pub token: String,
    pub is_full_update: bool,
}

// 更新分类
#[tauri::command]
pub async fn update_categories(
    data: String,
    handle: tauri::AppHandle,
) -> Result<CustomResponse, String> {
    let handle_clone = handle.clone();
    let context_global: State<'_, ContextGlobal> = handle.state();
    let queue: State<'_, Queue> = handle.state();

    let data = from_str::<UpdateCategories>(&data).unwrap();
    let categories = data.categories;
    let token = data.token;
    let source_path = context_global.source_path.lock().await.clone().unwrap();
    let context = Context::from_source_path(&source_path);

    let categories: Vec<Category> = categories
        .iter()
        .map(|cat| {
            let id = cat.id.to_string();
            let name = cat.name.clone();
            Category::new(token.clone(), context.clone(), id, name)
        })
        .collect();

    if let Err(e) =
        add_categories_task(&queue, categories, &data.is_full_update, &source_path).await
    {
        return Err(e);
    }

    tauri::async_runtime::spawn(async move {
        let queue: State<'_, Queue> = handle_clone.state();
        queue.start_task().await;
    });

    return Ok(CustomResponse {
        message: String::from("开始执行任务序列"),
        data: Some(json!(queue.total_count.lock().await.clone())),
    });
}

pub async fn add_categories_task(
    queue: &State<'_, Queue>,
    categories: Vec<Category>,
    is_full_update: &bool,
    source_path: &PathBuf,
) -> Result<(), String> {
    for category in categories {
        // 待解析的interfaces
        let result = category.fetch_interfaces().await;

        match result {
            Ok(interfaces) => {
                let tasks = match is_full_update {
                    true => interfaces,
                    false => filter_new_interfaces(interfaces, source_path, category.id),
                };
                add_interfaces_task(&queue, tasks).await;
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(())
}

fn filter_new_interfaces(
    interfaces: Vec<Interface>,
    source_path: &PathBuf,
    category_id: String,
) -> Vec<Interface> {
    let config = Config::new(source_path);
    let config_json = config.read_config().unwrap();

    let mut config_interfaces = vec![];
    let mut find_config_interfaces = false;

    for project in config_json.project_list {
        if find_config_interfaces == true {
            break;
        }
        for category in project.categories {
            if find_config_interfaces == true {
                break;
            }
            if category.id == category_id {
                config_interfaces = category.interfaces;
                find_config_interfaces = true;
            }
        }
    }

    let filtered_interfaces = interfaces
        .into_iter()
        .filter(|interface| {
            if config_interfaces.len() == 0 {
                return true;
            }

            for config_interface in &config_interfaces {
                if config_interface.id == interface.id {
                    return false;
                }
            }

            true
        })
        .collect::<Vec<_>>();

    filtered_interfaces
}
