use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct YapiProjectBaseInfo { 
    pub _id: u32,
    pub name: String
}