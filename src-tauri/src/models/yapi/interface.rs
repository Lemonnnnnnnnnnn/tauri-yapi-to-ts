use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::AppHandle;

// 接口详情
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InterfaceData {
    pub _id: u32,
    pub path: String,
    pub project_id: u32,
    pub title: String,
    pub catid: u32,
    pub req_body_other: Option<String>,
    pub req_query: Option<Vec<Value>>,
    pub req_params: Option<Vec<Value>>,
    pub req_body_form: Option<Vec<Value>>,
    pub req_body_type: Option<String>,
    pub res_body: Option<String>,
    pub method: String,
}

#[derive(Debug, Deserialize)]
pub struct InterfaceFetchParams {
    pub interface_id: u32,
    pub token: String,
    pub source_path: String,
}

#[derive(PartialEq, Debug)]
pub enum JsonType {
    Object,
    Array,
    Atom,
    Unknown,
}

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub enum FormType {
    Form,
    Json,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ObjectType {
    Object,
    Array,
}

#[derive(PartialEq, Debug)]
pub enum WebType {
    Request,
    Response,
}

#[derive(Clone, Debug)]
pub enum JsonValue {
    ObjectLike(ObjectLike),
    Atom(Atom),
    Null,
}

#[derive(Clone, Debug)]
pub struct Atom {
    pub value: String,
    pub required: bool,
    pub key: String,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub interface_name: String,
    pub key: String,
    pub value: JsonValue,
    pub required: bool,
    pub description: String,
}

#[derive(Clone, Debug)]
pub struct ObjectLike {
    pub interface_name: String,
    pub nodes: Vec<Node>,
    pub object_type: ObjectType,
    pub required: bool,
    pub key: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Root {
    pub interface_name: String,
    pub interface_desc: String,
    pub key: String,
    pub children: Option<Vec<JsonValue>>,
}
