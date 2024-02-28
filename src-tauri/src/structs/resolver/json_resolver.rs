use serde_json::Value;

use super::{
    entities::{Atom, Node, ObjectLike},
    enums::{JsonType, JsonValue, ObjectType, WebType},
    root::Root,
    utils::{get_desc, get_json, get_legal_name},
};

#[derive(Debug)]
pub struct JsonResolver {
    interface_name: String,
    interface_desc: String,
    root: Option<Root>,
    web_type: WebType,
    json_value: Value,
}

impl JsonResolver {
    pub fn new(
        web_type: WebType,
        interface_name: String,
        interface_desc: String,
        json_string: String,
    ) -> Self {
        Self {
            interface_name: Self::get_name(&interface_name),
            interface_desc,
            json_value: get_json(json_string),
            root: None,
            web_type,
        }
    }

    pub fn to_ts(&self) -> String {
        if let Some(root) = &self.root {
            root.to_ts()
        } else {
            panic!("root is None");
        }
    }

    pub fn generate_modal(&mut self) -> &Self {
        let root_key = if self.web_type == WebType::Request {
            String::from("request")
        } else {
            String::from("response")
        };
        let interface_name = self.interface_name.clone();

        if Self::is_object(&self.json_value) {
            let root = self.generate_root(interface_name, root_key);
            self.root = Some(root);
        } else {
            let root = Root::new(interface_name, self.interface_desc.clone(), root_key, None);
            self.root = Some(root);
        };

        self
    }

    fn generate_root(&self, interface_name: String, root_key: String) -> Root {
        let properties = self.json_value.get("properties").unwrap();
        let properties_iterator = properties.as_object().unwrap().iter();
        let required_list = self.json_value.get("required");

        let json_values: Vec<_> = properties_iterator
            .map(|(key, value)| {
                let node_type = Self::get_type(value);
                let raw_type = Self::get_ts_type(value);
                let required = Self::is_required(key, required_list);
                let description = get_desc(value, "description");
                let key = Self::get_name(&key);

                match node_type {
                    JsonType::Object => {
                        let nodes = self.generate_nodes(value);
                        let object = ObjectLike {
                            interface_name: self.interface_name.clone(),
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
                        let nodes = self.generate_nodes(items);
                        let array = ObjectLike {
                            interface_name: self.interface_name.clone(),
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

        let root = Root::new(
            interface_name,
            self.interface_desc.clone(),
            root_key,
            Some(json_values),
        );

        root
    }

    fn generate_nodes(&self, json_value: &Value) -> Vec<Node> {
        match json_value.get("properties") {
            Some(properties) => {
                let properties_iterator = properties.as_object().unwrap().iter();
                let required_list = json_value.get("required");

                let nodes: Vec<_> = properties_iterator
                    .map(|(key, value)| {
                        let node_type = Self::get_type(value);
                        let required = Self::is_required(key, required_list);
                        let raw_type = Self::get_ts_type(value);
                        let key = Self::get_name(&key);
                        let description = get_desc(value, "description");

                        let value = match node_type {
                            JsonType::Object => {
                                let nodes = self.generate_nodes(value);
                                JsonValue::ObjectLike(ObjectLike {
                                    interface_name: self.interface_name.clone(),
                                    nodes,
                                    object_type: ObjectType::Object,
                                    required,
                                    description: description.clone(),
                                    key: key.clone(),
                                })
                            }
                            JsonType::Array => {
                                let items = value.get("items").unwrap();
                                let nodes = self.generate_nodes(items);
                                JsonValue::ObjectLike(ObjectLike {
                                    interface_name: self.interface_name.clone(),
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

                        Node::new(
                            self.interface_name.clone(),
                            key.clone(),
                            value,
                            required,
                            description.clone(),
                        )
                    })
                    .collect();

                nodes
            }
            None => {
                vec![]
            }
        }
    }

    fn is_object(value: &Value) -> bool {
        if let JsonType::Object = Self::get_type(value) {
            return true;
        }

        false
    }

    fn is_required(key: &String, list: Option<&Value>) -> bool {
        if let Some(required_list) = list {
            let required_key_list = required_list.as_array().unwrap();

            if required_key_list.iter().any(|x| x.to_string() == format!("\"{}\"", *key)) {
                return true;
            }
            false
        } else {
            false
        }
    }

    fn get_type(value: &Value) -> JsonType {
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
            get_legal_name(raw_name)
        }
    }
}

// #[cfg(test)]
// mod test {
//     use std::fs;

//     use super::*;

//     #[test]
//     fn test_to_ts() {
//         let interface_name = "exportAgent";
//         let str = "{\"type\":\"object\",\"title\":\"empty object\",\"properties\":{\"code\":{\"type\":\"string\",\"description\":\"响应码\"},\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"orderId\":{\"type\":\"string\",\"description\":\"业务订单号\"},\"title\":{\"type\":\"string\",\"description\":\"标题\"},\"customerName\":{\"type\":\"string\",\"description\":\"客户名称\"},\"projectName\":{\"type\":\"string\",\"description\":\"项目名称\"},\"productList\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"productName\":{\"type\":\"string\",\"description\":\"商品名称\"},\"orderNum\":{\"type\":\"string\",\"description\":\"商品数量\"}},\"required\":[\"productName\",\"orderNum\"]},\"description\":\"订单商品列表\"},\"orderTotalNum\":{\"type\":\"string\",\"description\":\"订单总数量\"},\"approvalStatus\":{\"type\":\"string\",\"description\":\"审批状态0-待提交 1-审批中 2-审批通过 3-已退回\"},\"orderStatus\":{\"type\":\"string\",\"description\":\"订单状态 1-未发货 2-部分发货 3-全部发货 4-部分退货 5-全部退货 6-已签收\"},\"applicantName\":{\"type\":\"string\",\"description\":\"申请人\"},\"applicant\":{\"type\":\"string\",\"description\":\"申请人id\"},\"businessPersonId\":{\"type\":\"string\",\"description\":\"业务归属人id\"},\"businessPersonName\":{\"type\":\"string\",\"description\":\"归属业务人名称\"},\"applyDate\":{\"type\":\"string\",\"description\":\"申请日期 yyyyMMdd\"},\"updateTime\":{\"type\":\"string\",\"description\":\"操作时间\"}},\"required\":[\"orderId\",\"title\",\"customerName\",\"projectName\",\"productList\",\"orderTotalNum\",\"approvalStatus\",\"applicantName\",\"applyDate\",\"updateTime\",\"applicant\",\"businessPersonId\",\"businessPersonName\"]}},\"pager\":{\"type\":\"object\",\"properties\":{\"currentNum\":{\"type\":\"string\",\"description\":\"当前页\"},\"pageSize\":{\"type\":\"string\",\"description\":\"页大小\"},\"totalPages\":{\"type\":\"string\",\"description\":\"总页数\"},\"totalSize\":{\"type\":\"string\",\"description\":\"总大小\"}},\"description\":\"分页对象\",\"required\":[\"currentNum\",\"totalSize\",\"totalPages\",\"pageSize\"]},\"msg\":{\"type\":\"string\",\"description\":\"响应信息\"},\"timestamp\":{\"type\":\"string\",\"description\":\"响应时间\"}},\"required\":[\"code\",\"data\",\"pager\",\"msg\",\"timestamp\"]}";
//         let web_type = WebType::Request;

//         let res = JsonResolver::new(web_type, interface_name.to_string(), str.to_string())
//             .generate_modal()
//             .to_ts();

//         let _ = fs::write("./test.ts", res);
//     }
// }
