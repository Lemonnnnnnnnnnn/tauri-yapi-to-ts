use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, Value};
use tauri::{AppHandle, Manager, State};

use crate::structs::{
    category::Category,
    config::{CategoryShort, Config, ConfigJson, InterfaceShort, ProjectShort},
    context::{Context, ContextGlobal},
    project::Project,
    queue::Queue,
};

use super::{
    category::add_categories_task,
    common::CustomResponse,
    conversion::string_to_path_buf,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectList {
    pub id: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateProjects {
    pub projects: Vec<ProjectList>,
    pub is_full_update: bool,
}

// 更新项目
#[tauri::command]
pub async fn update_projects(
    data: String,
    handle: tauri::AppHandle,
) -> Result<CustomResponse, String> {
    let handle_clone = handle.clone();
    let context_global: State<'_, ContextGlobal> = handle.state();
    let queue: State<'_, Queue> = handle.state();

    let source_path = context_global.source_path.lock().await.clone().unwrap();
    let context = Context::from_source_path(&source_path);
    let data = from_str::<UpdateProjects>(&data).unwrap();

    let projects: Vec<_> = generate_modal(&data, &context);

    for project in projects {
        let result = project.fetch_categories().await;

        match result {
            Ok(categories) => {
                merge_to_config(&context_global, project, &categories).await;

                if let Err(e) =
                    add_categories_task(&queue, categories, &data.is_full_update, &source_path)
                        .await
                {
                    return Err(e);
                }
            }
            Err(_) => {
                return Err(String::from("fetch categories error"));
            }
        }
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

fn generate_modal(data: &UpdateProjects, context: &Context) -> Vec<Project> {
    let projects: Vec<_> = data
        .projects
        .iter()
        .map(|project| {
            let context_clone = context.clone();
            let id = project.id.clone();

            Project::new(id, project.token.clone(), context_clone)
        })
        .collect();

    projects
}

async fn merge_to_config(
    context_global: &ContextGlobal,
    project: Project,
    categories: &Vec<Category>,
) {
    let path = context_global.source_path.lock().await.clone().unwrap();
    let config = Config::new(&path);
    let new_project = init_short_project(&project.id, &project.token, categories);

    let mut config_json = config.read_config().unwrap();

    merge_projects(&new_project, &mut config_json);

    config.write_config(&config_json);
}

fn init_short_project(
    project_id: &String,
    token: &String,
    categories: &Vec<Category>,
) -> ProjectShort {
    let short_categories: Vec<_> = categories
        .into_iter()
        .map(|category| CategoryShort {
            id: category.id.clone(),
            name: category.name.clone(),
            interfaces: category
                .interfaces
                .iter()
                .map(|interface| InterfaceShort {
                    id: interface.id.clone(),
                    name: None,
                    path: None,
                })
                .collect::<Vec<InterfaceShort>>(),
        })
        .collect();

    ProjectShort {
        project_id: project_id.clone(),
        token: token.clone(),
        categories: short_categories,
    }
}

fn merge_projects(new_project: &ProjectShort, old_config: &mut ConfigJson) {
    let old_project = old_config
        .project_list
        .iter_mut()
        .find(|cp| cp.project_id == new_project.project_id);

    match old_project {
        Some(op) => {
            merge_categories(new_project, op);
        }
        None => {
            old_config.project_list.push(new_project.clone());
        }
    }
}

fn merge_categories(new_project: &ProjectShort, old_project: &mut ProjectShort) {
    for category in &new_project.categories {
        if old_project.categories.iter().any(|oc| oc.id == category.id) {
            continue;
        }
        old_project.categories.push(category.clone());
    }
}

//  -----------
