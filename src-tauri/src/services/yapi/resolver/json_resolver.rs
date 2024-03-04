use serde_json::Value;

use crate::models::yapi::interface::{
    Atom, InterfaceData, JsonType, JsonValue, Node, ObjectLike, ObjectType, Root, WebType,
};

use super::common::{get_desc, get_legal_name, get_path_arr};

// 生成 ts 字符串

pub fn get_ts_string(
    web_type: WebType,
    interface_data: &InterfaceData,
    json_value: &Value,
) -> String {
    let root = generate_root(web_type, interface_data, json_value);
    get_root_ts(root)
}

fn get_root_ts(root: Root) -> String {
    let ts_name = get_ts_interface_name(&root.interface_name, &root.key);
    let header = format!("// {}\n", root.interface_desc);
    let mut res_string = String::from(header);

    if let Some(nodes) = root.children {
        res_string = res_string + resolve_root_interfaces(&nodes, &ts_name).as_str();
    } else {
        res_string = res_string + resolve_root_interfaces(&vec![], &ts_name).as_str();
    }

    res_string
}

fn resolve_root_interfaces(nodes: &Vec<JsonValue>, ts_name: &str) -> String {
    let mut sub_list = Vec::new();
    let mut res_string = format!("export interface {} {{\n", ts_name);

    for node in nodes {
        let mut t = String::new();
        if let JsonValue::Atom(atom) = node {
            t = get_atom_ts(atom);
        } else if let JsonValue::ObjectLike(object_like) = node {
            t = get_object_like_ts(object_like);
            sub_list.push(object_like);
        }

        res_string = res_string + t.as_str();
    }

    res_string = res_string + "}\n";

    // 递归解析子interface
    let resolved_sub_list = resolve_root_interfaces_sub_list(sub_list);

    for (json_values, ts_name) in resolved_sub_list {
        res_string = res_string + &resolve_root_interfaces(&json_values, &ts_name);
    }

    res_string
}

fn resolve_root_interfaces_sub_list(sub_list: Vec<&ObjectLike>) -> Vec<(Vec<JsonValue>, String)> {
    sub_list
        .iter()
        .map(|object_like| {
            let ts_name = get_ts_interface_name(&object_like.interface_name, &object_like.key);
            let nodes = &object_like.nodes;
            let json_values: Vec<_> = nodes.iter().map(|node| node.value.clone()).collect();
            (json_values, ts_name)
        })
        .collect()
}

fn get_atom_ts(atom: &Atom) -> String {
    let required_symbol = if atom.required { "" } else { "?" };
    format!(
        "    // {}\n    {}{}: {}\n",
        atom.description.replace("\n", ""),
        atom.key,
        required_symbol,
        format_atom_type(&atom.value)
    )
}

fn format_atom_type(value: &String) -> String {
    if value.as_str() == "integer" {
        "number".to_string()
    } else {
        value.to_string()
    }
}

fn get_object_like_ts(object_like: &ObjectLike) -> String {
    let required_symbol = if object_like.required { "" } else { "?" };
    let object_name = get_ts_interface_name(&object_like.interface_name, &object_like.key);
    let array_symbol = if object_like.object_type == ObjectType::Array {
        "[]"
    } else {
        ""
    };

    format!(
        "    // {}\n    {}{}: {}{}\n",
        object_like.description, object_like.key, required_symbol, object_name, array_symbol
    )
}

// -------------- 生成模型

// 生成根节点模型
fn generate_root(web_type: WebType, interface_data: &InterfaceData, json_value: &Value) -> Root {
    let root_key = if web_type == WebType::Request {
        String::from("request")
    } else {
        String::from("response")
    };
    let interface_name = get_model_interface_name(interface_data);
    let interface_desc = interface_data.title.clone();
    let mut children = None;

    if is_object(json_value) {
        children = Some(get_root_children(interface_name.clone(), json_value));
    }
    Root {
        interface_name,
        interface_desc,
        key: root_key,
        children,
    }
}

fn get_root_children(interface_name: String, json_value: &Value) -> Vec<JsonValue> {
    let properties = json_value.get("properties").unwrap();
    let properties_iterator = properties.as_object().unwrap().iter();
    let required_list = json_value.get("required");

    let json_values: Vec<_> = properties_iterator
        .map(|(key, value)| {
            let node_type = get_json_type(value);
            let raw_type = get_ts_type(value);
            let required = is_required(key, required_list);
            let description = get_desc(value, "description");
            let key = get_name(&key);

            match node_type {
                JsonType::Object => {
                    let nodes = generate_nodes(value, &interface_name);
                    let object = ObjectLike {
                        interface_name: interface_name.to_string(),
                        nodes,
                        object_type: ObjectType::Object,
                        required,
                        description,
                        key,
                    };

                    JsonValue::ObjectLike(object)
                }
                JsonType::Atom => {
                    let atom = Atom {
                        value: raw_type,
                        required,
                        key,
                        description,
                    };
                    JsonValue::Atom(atom)
                }
                JsonType::Array => {
                    let items = value.get("items").unwrap();
                    let nodes = generate_nodes(items, &interface_name);
                    let array = ObjectLike {
                        interface_name: interface_name.to_string(),
                        nodes,
                        object_type: ObjectType::Array,
                        required,
                        description,
                        key,
                    };

                    JsonValue::ObjectLike(array)
                }
                _ => JsonValue::Null,
            }
        })
        .collect();

    json_values
}

fn generate_nodes(json_value: &Value, interface_name: &str) -> Vec<Node> {
    match json_value.get("properties") {
        Some(properties) => {
            let properties_iterator = properties.as_object().unwrap().iter();
            let required_list = json_value.get("required");

            let nodes: Vec<_> = properties_iterator
                .map(|(key, value)| {
                    let node_type = get_json_type(value);
                    let required = is_required(key, required_list);
                    let raw_type = get_ts_type(value);
                    let key = get_name(&key);
                    let description = get_desc(value, "description");

                    let value = match node_type {
                        JsonType::Object => {
                            let nodes = generate_nodes(value, interface_name);
                            JsonValue::ObjectLike(ObjectLike {
                                interface_name: interface_name.to_string(),
                                nodes,
                                object_type: ObjectType::Object,
                                required,
                                description: description.clone(),
                                key: key.clone(),
                            })
                        }
                        JsonType::Array => {
                            let items = value.get("items").unwrap();
                            let nodes = generate_nodes(items, interface_name);
                            JsonValue::ObjectLike(ObjectLike {
                                interface_name: interface_name.to_string(),
                                nodes,
                                object_type: ObjectType::Array,
                                required,
                                description: description.clone(),
                                key: key.clone(),
                            })
                        }
                        JsonType::Atom => {
                            let atom = Atom {
                                value: raw_type,
                                required,
                                key: key.clone(),
                                description: description.clone(),
                            };
                            JsonValue::Atom(atom)
                        }
                        _ => JsonValue::Null,
                    };

                    Node {
                        interface_name: interface_name.to_string(),
                        key: key.clone(),
                        value,
                        required,
                        description: description.clone(),
                    }
                })
                .collect();

            nodes
        }
        None => {
            vec![]
        }
    }
}

// -------------- 可组合方法

fn is_object(value: &Value) -> bool {
    if let JsonType::Object = get_json_type(value) {
        return true;
    }

    false
}

fn is_required(key: &String, list: Option<&Value>) -> bool {
    if let Some(required_list) = list {
        let required_key_list = required_list.as_array().unwrap();

        if required_key_list
            .iter()
            .any(|x| x.to_string() == format!("\"{}\"", *key))
        {
            return true;
        }
        false
    } else {
        false
    }
}

fn get_json_type(value: &Value) -> JsonType {
    match value.get("type") {
        Some(type_value) => match type_value.as_str() {
            Some(type_string) => {
                if type_string == "object" {
                    JsonType::Object
                } else if type_string == "array" {
                    JsonType::Array
                } else {
                    JsonType::Atom
                }
            }
            None => JsonType::Unknown,
        },
        None => JsonType::Unknown,
    }
}

fn get_ts_type(value: &Value) -> String {
    match value.get("type") {
        Some(type_value) => match type_value.as_str() {
            Some(type_value_str) => type_value_str.to_string(),
            None => String::from("any"),
        },
        None => String::from("any"),
    }
}

fn get_name(raw_name: &str) -> String {
    if raw_name.is_empty() {
        String::from("unknownName")
    } else {
        raw_name.to_string()
    }
}

fn get_model_interface_name(interface_data: &InterfaceData) -> String {
    let file_name = get_path_arr(interface_data.path.clone())
        .last()
        .unwrap_or(&"unknowFileName".to_string())
        .clone();
    file_name
}

// 拼接生成ts接口名字
fn get_ts_interface_name(interface_name: &str, key: &str) -> String {
    format!(
        "{}{}",
        get_legal_name(interface_name),
        capitalize_first_letter(&get_legal_name(key))
    )
}

// 大写第一个字符
fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
