use std::{path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use super::config::{ConfigJson, Config};


#[derive(Debug, Clone)]
pub struct ContextGlobal {
    pub source_path: Arc<Mutex<Option<PathBuf>>>,
}

impl ContextGlobal {
    pub async fn update(&self, source_path : PathBuf) {
        *self.source_path.lock().await = Some(source_path); 
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Context {
    pub source_path: Option<PathBuf>,
    pub base_url: Option<String>,
    pub rate_limit: Option<usize>,
    pub break_seconds: Option<u64>,
    pub types_path: Option<PathBuf>,
    pub types_full_path: Option<PathBuf>,
    pub proxy: Option<String>
}

impl Context {

    pub fn from_source_path(source: &PathBuf) -> Self {
        let config = Config::new(&source.clone());
        let config_json = config.read_config().unwrap();
        Self::from_json(config_json)
    }

    pub fn from_json(json : ConfigJson) -> Self {
        Self {
            source_path: json.source_path.clone(),
            base_url: json.base_url.clone(),
            rate_limit: json.rate_limit.clone(),
            break_seconds: json.break_seconds.clone(),
            types_path: json.types_path.clone(),
            types_full_path: json.types_full_path.clone(),
            proxy: json.proxy.clone(),
        }
    }
    
}
