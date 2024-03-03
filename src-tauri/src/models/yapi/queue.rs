use std::{
    collections::VecDeque,
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use serde::{Deserialize, Serialize};
use tauri::{async_runtime::Mutex, AppHandle, Manager};
use tokio::{sync::Semaphore, time::sleep};

use crate::services::{
    global_config::get_global_config,
    yapi::interface::{fetch_interface_detail, get_interface_ts_string},
};

use super::interface::{InterfaceData, InterfaceFetchParams};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResolvedInterface {
    pub interface: InterfaceData,
    pub ts_string: String,
}

#[derive(Debug, Clone)]
pub struct Queue {
    pub semaphore: Arc<Semaphore>,
    pub waiting_queue: Arc<Mutex<VecDeque<InterfaceFetchParams>>>,
    pub total_count: Arc<Mutex<usize>>,
    pub running: Arc<AtomicBool>,
    pub result_queue: Arc<Mutex<ResultQueue>>,
    pub app_handle: Arc<Mutex<AppHandle>>,
}

#[derive(Debug, Clone)]
pub struct ResultQueue {
    pub list: Vec<ResolvedInterface>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmitMessage {
    pub msg: String,
    pub is_success: bool, // 0:success,1:fail
    pub resolved_interface: Option<ResolvedInterface>,
}

impl Queue {
    pub fn new(app_handle: &AppHandle) -> Queue {
        let global_config = get_global_config(app_handle).unwrap();

        Queue {
            semaphore: Arc::new(Semaphore::new(global_config.rate_limit)),
            waiting_queue: Arc::new(Mutex::new(VecDeque::new())),
            running: Arc::new(AtomicBool::new(true)),
            total_count: Arc::new(Mutex::new(0)),
            result_queue: Arc::new(Mutex::new(ResultQueue {
                list: vec![],
            })),
            app_handle: Arc::new(Mutex::new(app_handle.clone())),
        }
    }

    pub async fn add_task(&self, interface_fetch_params: InterfaceFetchParams) {
        self.waiting_queue
            .lock()
            .await
            .push_back(interface_fetch_params);
    }

    pub async fn cancel_execute(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    pub async fn clear(&self) {
        *self.result_queue.lock().await = ResultQueue {
            list: vec![],
        };
        *self.total_count.lock().await = 0;
    }

    pub async fn start_execute(&self, app_handle: &AppHandle) {
        let global_config = get_global_config(app_handle).unwrap();

        while self.running.load(Ordering::Relaxed) {
            // 消耗一个令牌
            self.acquire().await;

            let interface_data = {
                let mut queue = self.waiting_queue.lock().await;
                queue.pop_front()
            };
            let app_handle = Arc::clone(&self.app_handle);
            let result_queue = Arc::clone(&self.result_queue);
            let sem = self.semaphore.clone();

            match interface_data {
                Some(fetch_interface_params) => {
                    tokio::spawn(async move {
                        let app_handle = app_handle.lock().await.clone();

                        match fetch_interface_detail(fetch_interface_params, &app_handle).await {
                            Ok(detail) => match get_interface_ts_string(&detail) {
                                Ok(ts_string) => {
                                    let title = detail.title.clone();
                                    queue_log(
                                        &app_handle,
                                        Some(ResolvedInterface {
                                            interface: detail,
                                            ts_string,
                                        }),
                                        format!("接口 {} 已完成！", title),
                                        true,
                                    )
                                }
                                Err(e) => {
                                    queue_log(
                                        &app_handle,
                                        None,
                                        format!("接口转换失败: {}", e.to_string()),
                                        false,
                                    );
                                }
                            },
                            Err(e) => {
                                queue_log(
                                    &app_handle,
                                    None,
                                    format!("接口请求失败：{}", e.to_string()),
                                    false,
                                );
                            }
                        }

                        sleep(Duration::from_secs(global_config.break_seconds)).await;
                        // 补充一个令牌
                        sem.add_permits(1);
                    });
                }
                None => {
                    self.running.store(false, Ordering::Relaxed);
                    self.clear().await;
                }
            }
        }
    }

    async fn acquire(&self) {
        let permit = self.semaphore.acquire().await.unwrap();
        permit.forget();
    }
}

fn queue_log(
    app_handle: &AppHandle,
    resolved_interface: Option<ResolvedInterface>,
    msg: String,
    is_success: bool,
) {
    app_handle
        .emit_all(
            "queue_log",
            EmitMessage {
                msg,
                is_success,
                resolved_interface,
            },
        )
        .unwrap();
}
