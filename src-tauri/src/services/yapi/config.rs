use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
};

use serde_json::{from_str, json};

use crate::{models::yapi::config::YapiConfig, services::conversion::string_to_path_buf};

pub const PROJECT_CONFIG_NAME: &str = "yapi.json";

pub fn init_project_config(source_path: String) -> Result<(), io::Error> {
    let yapi_config = YapiConfig::default();
    let yapi_config_path = string_to_path_buf(source_path).join(PROJECT_CONFIG_NAME);

    if yapi_config_path.exists() {
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(yapi_config_path)?;
    file.write_all(json!(yapi_config).to_string().as_bytes())?;
    Ok(())
}

pub fn get_project_config(source_path: &str) -> Result<YapiConfig, io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(string_to_path_buf(source_path.to_string()).join(PROJECT_CONFIG_NAME))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(from_str(&contents)?)
}

pub fn write_project_config(source_path: String, yapi_config: YapiConfig) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .open(string_to_path_buf(source_path).join(PROJECT_CONFIG_NAME))?;
    file.write_all(json!(yapi_config).to_string().as_bytes())?;
    Ok(())
}
