// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use models::yapi::queue::Queue;
use services::{
    conversion::path_buf_to_string,
    global_config::init_config,
};
use structs::context::ContextGlobal;
use tauri::{api::dialog, CustomMenuItem, Manager, Menu, MenuItem, Submenu};
use tokio::sync::Mutex;

use crate::services::{
    category::update_categories,
    common::{execute, pause},
    global_config::read_config,
    interface::update_interface,
    project::update_projects,
    request::{check_request_config, get_request_list, update_request},
};

use crate::commands::{
    global_config::{add_project, load_latest_project, update_config},
    yapi::category::get_cat_interface_list,
    yapi::config::{load_project_config, update_project_config},
    yapi::interface::{add_interface_task, cancel_task, start_task, write_to_file},
    yapi::project::{get_yapi_project_base_info, get_yapi_project_cat_menu},
};

pub mod commands;
pub mod models;
pub mod services;
pub mod structs;
pub mod utils;

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
    let menu = Menu::new().add_submenu(file_menu).add_submenu(edit_menu);

    tauri::Builder::default()
        .manage(ContextGlobal {
            source_path: Arc::new(Mutex::new(None)),
        })
        .setup(|app| {
            let app_handle = app.handle();
            init_config(&app_handle).unwrap();

            app.manage(Queue::new(&app_handle));
            // if let Ok(source_path) = get_latest_project_source_path(&app_handle) {
            //     app.emit_all("load_project", source_path).unwrap();
            // }

            // services::storage::wrtie_project(&app_handle);
            // let main_window = app.get_window("main").unwrap();

            // app.manage(Queue::new(DEAFULT_RATE_LIMIT, 0, main_window));
            Ok(())
        })
        .menu(menu)
        .on_menu_event(|event| {
            let window = event.window().clone();
            let app_handle = window.app_handle();
            // let handler_clone = handler.clone();

            match event.menu_item_id() {
                "add" => dialog::FileDialogBuilder::default().pick_folder(move |source_path| {
                    if let Some(source_path) = source_path {
                        let source_path = path_buf_to_string(source_path);

                        match add_project(&source_path, app_handle) {
                            Ok(_) => {
                                window.emit_all("load_project", source_path).unwrap();
                            }
                            Err(e) => {
                                window.emit_all("load_project_error", e).unwrap();
                            }
                        }

                        // spawn(async move {
                        //     match read_config(
                        //         json!(GetConfigRequest {
                        //             dir_path: dir_path.to_str().unwrap().to_string(),
                        //         })
                        //         .to_string(),
                        //         handler_clone,
                        //     )
                        //     .await
                        //     {
                        //         Ok(res) => {
                        //             window.emit_all("init_completed", res.message).unwrap();
                        //         }
                        //         Err(e) => {
                        //             window.emit_all("missing_config", e).unwrap();
                        //         }
                        //     }
                        // });
                    }
                }),
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            read_config,
            update_config,
            // get_global_config,
            update_categories,
            update_interface,
            update_projects,
            execute,
            pause,
            get_request_list,
            check_request_config,
            update_request,
            add_project,
            load_latest_project,
            load_project_config,
            update_project_config,
            get_yapi_project_base_info,
            get_yapi_project_cat_menu,
            get_cat_interface_list,
            add_interface_task,
            start_task,
            cancel_task,
            write_to_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
