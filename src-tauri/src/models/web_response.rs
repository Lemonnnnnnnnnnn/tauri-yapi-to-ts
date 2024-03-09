#[derive(serde::Serialize)]
pub struct WebResponse {
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl Default for WebResponse {
    fn default() -> Self {
        Self {
            message: "".to_string(),
            data: None,
        }
    }
}