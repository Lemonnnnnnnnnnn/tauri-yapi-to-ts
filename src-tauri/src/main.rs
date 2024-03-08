// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use models::yapi::queue::Queue;
use services::{conversion::path_buf_to_string, global_config::init_config};
use tauri::{api::dialog, CustomMenuItem, Manager, Menu, MenuItem, Submenu};

use crate::commands::{
    global_config::{add_project, load_global_config, load_latest_project, update_global_config},
    yapi::category::get_cat_interface_list,
    yapi::config::{load_project_config, update_project_config},
    yapi::interface::{
        add_interface_task, cancel_task, get_interface_detail, start_task, write_to_file,
    },
    yapi::project::{get_yapi_project_base_info, get_yapi_project_cat_menu},
    yapi::request::{get_request_string, load_file_tree, write_request_to_file},
};

pub mod commands;
pub mod models;
pub mod services;
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
        .setup(|app| {
            let app_handle = app.handle();
            init_config(&app_handle).unwrap();

            app.manage(Queue::new(&app_handle));
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
                    }
                }),
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            update_global_config,
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
            write_to_file,
            load_global_config,
            load_file_tree,
            get_request_string,
            write_request_to_file,
            get_interface_detail
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
