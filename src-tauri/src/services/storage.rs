use std::path::PathBuf;
use tauri::AppHandle;

pub fn get_app_config_dir(app_handle: &AppHandle) -> Option<PathBuf> {
    app_handle.path_resolver().app_config_dir()
}

pub fn get_local_data_dir(app_handle: &AppHandle) -> Option<PathBuf> {
    app_handle.path_resolver().app_local_data_dir()
}

pub fn get_app_log_dir(app_handle: &AppHandle) -> Option<PathBuf> {
    app_handle.path_resolver().app_log_dir()
}
