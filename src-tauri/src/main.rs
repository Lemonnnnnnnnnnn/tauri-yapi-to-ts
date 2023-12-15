// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use serde_json::json;
use services::config::GetConfigRequest;
use structs::{context::ContextGlobal, queue::Queue};
use tauri::{api::dialog, async_runtime::spawn, CustomMenuItem, Manager, Menu, Submenu, MenuItem};
use tokio::sync::Mutex;

use crate::services::{
    category::update_categories,
    common::{execute, pause},
    config::{get_config, read_config, update_config},
    interface::update_interface,
    project::update_projects,
    request::{check_request_config, get_request_list, update_request},
};

pub mod services;
pub mod structs;
pub mod utils;

const DEAFULT_RATE_LIMIT: usize = 2;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let add = CustomMenuItem::new("add".to_string(), "添加代码库");
    let file_menu = Submenu::new("文件", Menu::new().add_item(add));
    let edit_menu = Submenu::new(
        "编辑",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll),
    );
    let menu = Menu::new()
        .add_submenu(file_menu)
        .add_submenu(edit_menu);

    tauri::Builder::default()
        .manage(ContextGlobal {
            source_path: Arc::new(Mutex::new(None)),
        })
        // .manage(Queue::new(DEAFULT_RATE_LIMIT, 0))
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            app.manage(Queue::new(DEAFULT_RATE_LIMIT, 0, main_window));
            Ok(())
        })
        .menu(menu)
        .on_menu_event(|event| {
            let window = event.window().clone();
            let handler: tauri::AppHandle = window.app_handle();
            let handler_clone = handler.clone();

            match event.menu_item_id() {
                "add" => dialog::FileDialogBuilder::default().pick_folder(move |dir_path| {
                    if let Some(dir_path) = dir_path {
                        spawn(async move {
                            match read_config(
                                json!(GetConfigRequest {
                                    dir_path: dir_path.to_str().unwrap().to_string(),
                                })
                                .to_string(),
                                handler_clone,
                            )
                            .await
                            {
                                Ok(res) => {
                                    window.emit_all("init_completed", res.message).unwrap();
                                }
                                Err(e) => {
                                    window.emit_all("missing_config", e).unwrap();
                                }
                            }
                        });
                    }
                }),
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            read_config,
            update_config,
            get_config,
            update_categories,
            update_interface,
            update_projects,
            execute,
            pause,
            get_request_list,
            check_request_config,
            update_request,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
