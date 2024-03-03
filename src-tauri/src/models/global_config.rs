use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    // 项目工程
    pub projects: Vec<String>,
    // 请求间隔
    pub break_seconds: u64,
    // 并行请求数
    pub rate_limit: usize,
    // 代理
    pub proxy: Option<String>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        GlobalConfig {
            projects: Vec::new(),
            break_seconds: 3,
            rate_limit: 5,
            proxy: None,
        }
    }
}

impl GlobalConfig {
    pub fn merge_from_request(&mut self, update: GlobalConfigRequest) {
        if let Some(rate_limit) = update.rate_limit {
            self.rate_limit = rate_limit;
        }
        if let Some(break_seconds) = update.break_seconds {
            self.break_seconds = break_seconds;
        }
        if let Some(proxy) = update.proxy {
            self.proxy = Some(proxy);
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalConfigRequest {
    pub rate_limit: Option<usize>,
    pub break_seconds: Option<u64>,
    pub proxy: Option<String>,
}
