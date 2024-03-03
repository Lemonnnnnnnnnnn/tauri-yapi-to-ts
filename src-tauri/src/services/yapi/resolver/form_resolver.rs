use serde_json::{json, Value};

use crate::models::yapi::interface::{InterfaceData, WebType};

use super::common::{get_desc, get_legal_name, get_path_arr, get_request_json};

pub fn get_ts_string(web_type: WebType, interface_data: &InterfaceData , form_value: &Value) -> String {
    let header = format!("// {}", interface_data.title);
    let file_name = get_path_arr(interface_data.path.clone())
        .last()
        .unwrap_or(&"unknowFileName".to_string())
        .clone();
    let interface_ts_name = get_interface_ts_name(web_type, &file_name);
    let mut res_string = format!("{}\nexport interface {} {{\n", header, interface_ts_name);

    // let form_value = json!(get_request_json(interface_data));

    match form_value.is_array() {
        true => {
            for value in form_value.as_array().unwrap() {
                let name = get_name(&value);
                let t = get_type(&value);
                let desc = get_desc(&value, "desc");
                let required = is_required(&value);
                let required_symbol = if required { "" } else { "?" };

                let type_define: String =
                    format!("    // {}\n    {}{}: {}\n", desc, name, required_symbol, t);

                res_string = res_string + type_define.as_str();
            }
        }
        false => {}
    }
    res_string = res_string + "}\n";

    res_string
}

fn get_interface_ts_name(web_type: WebType, file_name: &str) -> String {
    match web_type {
        WebType::Request => format!("{}Request", get_legal_name(file_name)),
        WebType::Response => format!("{}Response", get_legal_name(file_name)),
    }
}

fn get_name(value: &Value) -> String {
    match value.get("name") {
        Some(name) => match name.as_str() {
            Some(name_str) => get_legal_name(name_str),
            None => String::from("unknowName"),
        },
        None => String::from("unknowName"),
    }
}

fn is_required(value: &Value) -> bool {
    match value.get("required") {
        Some(required) => match required.as_str() {
            Some(required_string) => {
                if required_string == "1" {
                    true
                } else {
                    false
                }
            }
            None => false,
        },
        None => false,
    }
}

fn get_type(value: &Value) -> String {
    match value.get("type") {
        Some(t) => match t.as_str() {
            Some(t_str) => {
                if t_str == "text" {
                    String::from("string")
                } else {
                    String::from("any")
                }
            }
            None => String::from("string"),
        },
        None => String::from("string"),
    }
}
