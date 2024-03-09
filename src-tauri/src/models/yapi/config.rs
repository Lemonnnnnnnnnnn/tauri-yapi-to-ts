use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct YapiConfig {
    pub base_url: String,
    pub types_path: String,
    pub project_list: Vec<YapiProject>,
    pub request_path: String,
    pub request_template: String,
    pub header_template: String,
    pub file_name_template: String,
    pub type_import_template: String,
}

impl Default for YapiConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            types_path: String::new(),
            project_list: Vec::new(),
            request_path: String::new(),
            request_template: String::new(),
            header_template: String::new(),
            file_name_template: String::new(),
            type_import_template: String::new(),
        }
    }
}

impl Into<Value> for YapiConfig {
    fn into(self) -> Value {
        json!(self)
    }
}

impl YapiConfig {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn merge_from_request(&mut self, yapi_config_request: YapiConfigRequest) {
        // if yapi_config_request has some value , merge this property to YapiConfig
        if let Some(base_url) = yapi_config_request.base_url {
            self.base_url = base_url;
        }
        if let Some(types_path) = yapi_config_request.types_path {
            self.types_path = types_path;
        }
        if let Some(request_path) = yapi_config_request.request_path {
            self.request_path = request_path;
        }
        if let Some(request_template) = yapi_config_request.request_template {
            self.request_template = request_template;
        }
        if let Some(header_template) = yapi_config_request.header_template {
            self.header_template = header_template;
        }
        if let Some(file_name_template) = yapi_config_request.file_name_template {
            self.file_name_template = file_name_template;
        }
        if let Some(type_import_template) = yapi_config_request.type_import_template {
            self.type_import_template = type_import_template;
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct YapiConfigRequest {
    pub base_url: Option<String>,
    pub types_path: Option<String>,
    pub request_path: Option<String>,
    pub request_template: Option<String>,
    pub header_template: Option<String>,
    pub file_name_template: Option<String>,
    pub type_import_template: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct YapiProject {
    pub token: String,
    pub project_id: String,
    pub project_name: Option<String>,
    pub categories: Vec<YapiCategory>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct YapiCategory {
    pub id: String,
    pub name: String,
    pub interfaces: Vec<YapiInterface>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct YapiInterface {
    pub id: String,
    pub name: String,
    pub path: Option<String>,
    pub lock: Option<bool>
}
