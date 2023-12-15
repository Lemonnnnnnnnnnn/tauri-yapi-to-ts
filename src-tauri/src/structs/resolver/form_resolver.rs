use serde_json::Value;

use super::{
    enums::WebType,
    utils::{get_desc, get_json, get_legal_name},
};

#[derive(Debug)]
pub struct FormResolver {
    web_type: WebType,
    interface_name: String,
    interface_desc: String,
    form_value: Value,
}

impl FormResolver {
    pub fn new(
        web_type: WebType,
        interface_name: String,
        interface_desc: String,
        json_string: String,
    ) -> Self {
        Self {
            web_type,
            interface_name: get_legal_name(&interface_name),
            interface_desc,
            form_value: get_json(json_string),
        }
    }

    pub fn to_ts(&self) -> String {
        let header = format!("// {}", self.interface_desc);
        let interface_ts_name = self.get_interface_ts_name();
        let mut res_string = format!("{}\nexport interface {} {{\n", header, interface_ts_name);

        match self.form_value.is_array() {
            true => {
                for value in self.form_value.as_array().unwrap() {
                    let name = Self::get_name(&value);
                    let t = Self::get_type(&value);
                    let desc = get_desc(&value, "desc");
                    let required = Self::get_required(value);
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

    fn get_interface_ts_name(&self) -> String {
        match self.web_type {
            WebType::Request => format!("{}Request", self.interface_name),
            WebType::Response => format!("{}Response", self.interface_name),
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

    fn get_required(value: &Value) -> bool {
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
                        return String::from("string");
                    } else {
                        return String::from("any");
                    }
                }
                None => return String::from("string"),
            },
            None => {
                return String::from("string");
            }
        };
    }
}
