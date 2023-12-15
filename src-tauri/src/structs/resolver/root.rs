use super::{
    entities::{ObjectLike, ToTs, TsBridge},
    enums::JsonValue,
};
use crate::utils::capitalize_first_letter;
use ts_bridge_derive::TsBridge;

#[derive(TsBridge, Debug)]
pub struct Root {
    pub interface_name: String,
    pub interface_desc: String,
    pub key: String,
    pub children: Option<Vec<JsonValue>>,
}

impl Root {
    pub fn new(
        interface_name: String,
        interface_desc: String,
        key: String,
        children: Option<Vec<JsonValue>>,
    ) -> Self {
        Self {
            interface_name,
            interface_desc,
            key,
            children,
        }
    }

    pub fn to_ts(&self) -> String {
        let ts_name = self.get_ts_name();
        let header = format!("// {}\n", self.interface_desc);
        let mut res_string = String::from(header);

        if let Some(nodes) = &self.children {
            res_string = res_string + self.generate_interface(nodes, &ts_name).as_str();
        } else {
            res_string = res_string + self.generate_interface(&vec![], &ts_name).as_str();
        }

        res_string
    }

    fn generate_interface(&self, nodes: &Vec<JsonValue>, ts_name: &str) -> String {
        let mut sub_list = Vec::new();
        let mut res_string = format!("export interface {} {{\n", ts_name);

        for node in nodes {
            let mut t = String::new();
            if let JsonValue::Atom(atom) = node {
                t = atom.to_ts();
            } else if let JsonValue::ObjectLike(object_like) = node {
                t = object_like.to_ts();
                sub_list.push(object_like);
            }

            res_string = res_string + t.as_str();
        }

        res_string = res_string + "}\n";

        // 递归解析子interface
        let resolved_sub_list = self.resolve_sub_list(sub_list);

        for (json_values, ts_name) in resolved_sub_list {
            res_string = res_string + &self.generate_interface(&json_values, &ts_name);
        }

        res_string
    }

    fn resolve_sub_list(&self, sub_list: Vec<&ObjectLike>) -> Vec<(Vec<JsonValue>, String)> {
        let res: Vec<_> = sub_list
            .iter()
            .map(|object_like| {
                let ts_name = object_like.get_ts_name();
                let nodes = &object_like.nodes;
                let json_values: Vec<_> = nodes.iter().map(|node| node.value.clone()).collect();
                (json_values, ts_name)
            })
            .collect();

        res
    }
}
