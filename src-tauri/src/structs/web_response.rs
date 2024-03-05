use serde::{Deserialize, Serialize};
use serde_json::Value;

// 公共返回参数
#[derive(Debug, Deserialize, Serialize)]
pub struct CommonResponse<T> {
    pub errcode : u32,
    pub errmsg : String,
    pub data : T
} 

// 分类菜单列表项
#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryMenuItem { 
    pub _id: u32,
    pub name: String,
    pub interfaces : Option<CategoryDataList>
}

// 分类详情列表
#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryDataList {
    pub count : u32,
    pub total : u32,
    pub list : Vec<InterfaceDataItem>
}

// 分类详情
#[derive(Debug, Deserialize, Serialize , Clone)]
pub struct InterfaceDataItem {
    pub _id : u32,
    pub catid : u32,
    pub title : String,
    pub path : String,
}

// 接口详情
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceData {
    pub _id : u32,
    pub path : String,
    pub project_id: u32,
    pub title : String,
    pub catid : u32,
    pub req_body_other: Option<String>,
    pub req_query: Option<Vec<Value>>,
    pub req_params : Option<Vec<Value>>,
    pub req_body_form : Option<Vec<Value>>,
    pub req_body_type : Option<String>,
    pub res_body : Option<String>,
    pub method: String,
}


