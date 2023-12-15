use serde_json::Value;

pub fn get_legal_name(raw_name: &str) -> String {
    let chars = raw_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<Vec<_>>();

    String::from_iter(chars)
}

pub fn get_legal_desc(raw_desc: &str) -> String {
    raw_desc.replace("\n", "").to_string()
}

pub fn get_desc(value: &Value , key: &str) -> String {
    let desc = match value.get(key) {
        Some(desc) => {
            match desc.as_str() {
                Some(desc_str) => get_legal_desc(desc_str),
                None => String::from("无注释")
            }
        },
        None => String::from("无注释")
    };

    desc
}


pub fn get_json(json_str: String) -> serde_json::Value {
    match serde_json::from_str(&json_str) {
        Ok(json) => json,
        Err(_) => serde_json::Value::Null,
    }
}
