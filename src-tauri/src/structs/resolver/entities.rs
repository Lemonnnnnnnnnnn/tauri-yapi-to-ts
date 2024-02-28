use crate::utils::capitalize_first_letter;
use ts_bridge_derive::TsBridge;

use super::enums::{JsonValue, ObjectType};

pub trait TsBridge {
    fn get_ts_name(&self) -> String;
}

pub trait ToTs {
    fn to_ts(&self) -> String;
}

#[derive(Clone, Debug)]
pub struct Atom {
    pub value: String,
    pub required: bool,
    pub key: String,
    pub description: String,
}

impl Atom {
    pub fn new(value: String, required: bool, key: String, description: String) -> Self {
        Self {
            value,
            required,
            key,
            description,
        }
    }

    fn format_type(value: &String) -> String {
        if value.as_str() == "integer" {
            "number".to_string()
        } else {
            value.to_string()
        }
    }
}

impl ToTs for Atom {
    fn to_ts(&self) -> String {
        
        let required_symbol = if self.required { "" } else { "?" };
        format!(
            "    // {}\n    {}{}: {}\n",
            self.description.replace("\n", ""),
            self.key,
            required_symbol,
            Self::format_type(&self.value)
        )
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub interface_name: String,
    pub key: String,
    pub value: JsonValue,
    pub required: bool,
    pub description: String,
}

impl Node {
    pub fn new(
        interface_name: String,
        key: String,
        value: JsonValue,
        required: bool,
        description: String,
    ) -> Self {
        Self {
            interface_name,
            key,
            value,
            required,
            description,
        }
    }
}

#[derive(TsBridge, Clone, Debug)]
pub struct ObjectLike {
    pub interface_name: String,
    pub nodes: Vec<Node>,
    pub object_type: ObjectType,
    pub required: bool,
    pub key: String,
    pub description: String,
}

impl ObjectLike {
    pub fn new(
        interface_name: String,
        nodes: Vec<Node>,
        object_type: ObjectType,
        required: bool,
        key: String,
        description: String,
    ) -> Self {
        Self {
            interface_name,
            nodes,
            object_type,
            required,
            key,
            description,
        }
    }
}

impl ToTs for ObjectLike {
    fn to_ts(&self) -> String {
        let required_symbol = if self.required { "" } else { "?" };
        let object_name = self.get_ts_name();
        let array_symbol = if self.object_type == ObjectType::Array {
            "[]"
        } else {
            ""
        };

        let t = format!(
            "    // {}\n    {}{}: {}{}\n",
            self.description, self.key, required_symbol, object_name, array_symbol
        );

        t
    }
}
