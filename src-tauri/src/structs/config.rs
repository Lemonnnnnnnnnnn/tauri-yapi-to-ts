use std::{fs::File, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "yapi.json";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectShort {
    pub token: String,
    pub project_id: String,
    pub categories: Vec<CategoryShort>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CategoryShort {
    pub id: String,
    pub name: String,
    pub interfaces: Vec<InterfaceShort>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InterfaceShort {
    pub id: String,
    pub name: Option<String>,
    pub path : Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigJson {
    pub source_path: Option<PathBuf>,
    pub base_url: Option<String>,
    pub rate_limit: Option<usize>,
    pub types_path: Option<PathBuf>,
    pub types_full_path: Option<PathBuf>,
    pub break_seconds: Option<u64>,
    pub project_list: Vec<ProjectShort>,
    pub request_path: Option<PathBuf>,
    pub request_full_path: Option<PathBuf>,
    pub request_template: Option<String>,
    pub header_template: Option<String>,
}

impl ConfigJson {
    pub fn new(source_path : PathBuf) -> Self{
        Self{
            source_path: Some(source_path),
            base_url: None,
            rate_limit: None,
            types_path: None,
            types_full_path: None,
            break_seconds: None,
            project_list: vec![],
            request_path: None,
            request_full_path:None,
            request_template: None,
            header_template: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub config_path: PathBuf,
}

impl Config {
    pub fn new(source_path: &PathBuf) -> Self {
        let config_path = source_path.clone().join(CONFIG_PATH);
        Self { config_path }
    }

    pub fn read_config(&self) -> Result<ConfigJson, String> {
        let config_path = self.config_path.to_str().unwrap();
        let f = File::open(config_path);

        match f {
            Ok(file) => {
                let read_result = serde_json::from_reader(file);
                match read_result {
                    Ok(config_json) => {
                        let config_json: ConfigJson = config_json;
                        Ok(config_json)
                    }
                    Err(e) => return Err(e.to_string()),
                }
            }
            Err(_) => Err("未找到配置文件".to_owned()),
        }
    }

    pub fn write_config(&self, config_json: &ConfigJson) {
        let config_path = self.config_path.to_str().unwrap();
        let mut file = File::create(config_path).unwrap();

        let json = serde_json::to_string(&config_json).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
