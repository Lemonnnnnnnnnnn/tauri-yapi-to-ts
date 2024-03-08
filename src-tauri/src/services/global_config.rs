use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
};

use serde_json::{from_str, json};
use tauri::AppHandle;

use crate::models::global_config::GlobalConfig;

use super::{log::log, storage::get_local_data_dir};

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
