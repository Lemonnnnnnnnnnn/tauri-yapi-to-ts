use serde::{Deserialize, Serialize};

// 公共返回参数
#[derive(Debug, Deserialize, Serialize)]
pub struct YapiResponse<T> {
    pub errcode : u32,
    pub errmsg : String,
    pub data : T
}