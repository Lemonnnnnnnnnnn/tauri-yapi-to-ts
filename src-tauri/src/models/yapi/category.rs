use serde::{Deserialize, Serialize};

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
