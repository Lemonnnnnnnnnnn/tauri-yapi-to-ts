use std::{collections::VecDeque, sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use tauri::{async_runtime::Mutex, Window};
use tokio::time::sleep;

use crate::structs::config::InterfaceShort;

use super::{
    config::Config,
    interface::{Interface, ResolvedInterface},
};

#[derive(Debug, Clone)]
pub struct Queue {
    pub waiting_queue: Arc<Mutex<VecDeque<Interface>>>,
    pub total_count: Arc<Mutex<usize>>,
    pub rate_limit: Arc<Mutex<usize>>,
    pub break_seconds: Arc<Mutex<u64>>,
    pub running: Arc<Mutex<bool>>,
    pub result_queue: Arc<Mutex<ResultQueue>>,
    pub config: Arc<Mutex<Option<Config>>>,
    pub window: Arc<Mutex<Window>>,
}

#[derive(Debug, Clone)]
pub struct ResultQueue {
    pub list: Vec<ResolvedInterface>,
    pub processing_count: usize,
    pub over_count: usize,
    pub over_count_group: usize, // 用来做任务分组
}

#[derive(Serialize, Deserialize, Clone)]
struct EmitMessage {
    pub msg: String,
    pub success_number: usize,
    pub is_success: bool, // 0:success,1:fail
}

impl Queue {
    pub fn new(rate_limit: usize, break_seconds: u64, window: Window) -> Queue {
        Queue {
            waiting_queue: Arc::new(Mutex::new(VecDeque::new())),
            rate_limit: Arc::new(Mutex::new(rate_limit)),
            break_seconds: Arc::new(Mutex::new(break_seconds)),
            running: Arc::new(Mutex::new(false)),
            total_count: Arc::new(Mutex::new(0)),
            result_queue: Arc::new(Mutex::new(ResultQueue {
                list: vec![],
                over_count: 0,
                processing_count: 0,
                over_count_group: 0,
            })),
            config: Arc::new(Mutex::new(None)),
            window: Arc::new(Mutex::new(window)),
        }
    }

    pub async fn add_task(&self, interface: Interface) {
        let mut waiting_queue = self.waiting_queue.lock().await;

        waiting_queue.push_back(interface);

        let mut total_count = self.total_count.lock().await;

        *total_count = *total_count + 1;
    }

    pub async fn start_task(&self) {
        let running_arc = Arc::clone(&self.running);
        let mut running = running_arc.lock().await;
        *running = true;

        drop(running);

        self.start_execute().await;
    }

    pub async fn stop_task(&self) {
        let mut running = self.running.lock().await;
        println!("stop!");
        *running = false;
        self.clear().await;
    }

    async fn start_execute(&self) {
        loop {
            let result_queue = &self.result_queue;
            let total_count = self.total_count.lock().await.clone();
            let rate_limit = self.rate_limit.lock().await.clone();
            let break_seconds = self.break_seconds.lock().await.clone();
            let mut running = self.running.lock().await;

            if *running == false {
                return;
            }

            let over_count = result_queue.lock().await.over_count;
            if total_count == over_count {
                *running = false;
                println!("over!");
                self.post_task().await;
                self.clear().await;
                let window = self.window.lock().await;
                window.emit("task_completed", {}).unwrap();
                return;
            }

            // 如果分组处理完毕，则开始新一批任务
            if result_queue.lock().await.processing_count != 0
                && result_queue.lock().await.over_count_group >= rate_limit
                && result_queue.lock().await.processing_count >= rate_limit
            {
                result_queue.lock().await.processing_count -= rate_limit;
                // 如果任务没有全部完成，等待一段时间再进行下一批任务
                if result_queue.lock().await.over_count != total_count {
                    sleep(Duration::from_secs(break_seconds.clone())).await;
                }
            }

            // 分组尚未处理完毕，循环等待
            if result_queue.lock().await.processing_count >= rate_limit {
                continue;
            }

            let mut waiting_queue = self.waiting_queue.lock().await;
            let task = waiting_queue.pop_front();

            match task {
                None => {
                    continue;
                }
                Some(t) => {
                    let result_queue_clone = Arc::clone(result_queue);
                    result_queue_clone.lock().await.processing_count += 1;
                    let window = Arc::clone(&self.window);

                    // 请求接口
                    tokio::spawn(async move {
                        let result = t.fetch_detail().await;
                        let interface_id = t.id.clone();

                        let window_lock = window.lock().await;

                        match result {
                            Ok(ri) => {
                                match t.write_ts(&ri) {
                                    Ok(_) => {
                                        let title = ri.title.clone();
                                        result_queue_clone.lock().await.list.push(ri);
                                        result_queue_clone.lock().await.over_count += 1;
                                        result_queue_clone.lock().await.over_count_group += 1;
                                        window_lock
                                            .emit(
                                                "log",
                                                EmitMessage {
                                                    msg: format!("interface {} completed", title),
                                                    success_number: result_queue_clone
                                                        .lock()
                                                        .await
                                                        .over_count,
                                                    is_success: true,
                                                },
                                            )
                                            .unwrap();
                                    }
                                    Err(e) => {
                                        result_queue_clone.lock().await.list.push(ri);
                                        result_queue_clone.lock().await.over_count += 1;
                                        result_queue_clone.lock().await.over_count_group += 1;

                                        window_lock
                                            .emit(
                                                "log",
                                                EmitMessage {
                                                    msg: e,
                                                    success_number: result_queue_clone
                                                        .lock()
                                                        .await
                                                        .over_count,
                                                    is_success: false,
                                                },
                                            )
                                            .unwrap();
                                    }
                                };
                            }
                            Err(_) => {
                                result_queue_clone.lock().await.over_count += 1;
                                result_queue_clone.lock().await.over_count_group += 1;
                                window_lock
                                    .emit(
                                        "log",
                                        EmitMessage {
                                            msg: format!("fetch interface {} error!", interface_id),
                                            success_number: result_queue_clone
                                                .lock()
                                                .await
                                                .over_count,
                                            is_success: false,
                                        },
                                    )
                                    .unwrap();
                            }
                        }
                    });
                }
            }
        }
    }

    pub async fn post_task(&self) {
        let config_mutex = self.config.lock().await;
        let config = config_mutex.as_ref();
        let result_queue_mutex = self.result_queue.lock().await;
        let result_queue = &result_queue_mutex.list;

        match config {
            Some(config) => {
                let config_json_result = config.read_config();

                match config_json_result {
                    Ok(mut config_json) => {
                        for ri in result_queue {
                            let insert_node_id = &ri.category_id;
                            let mut find = false;

                            for project in &mut config_json.project_list {
                                if find {
                                    break;
                                }

                                for category in &mut project.categories {
                                    if find {
                                        break;
                                    }

                                    if category.id.as_str() == insert_node_id.as_str() {
                                        for interface in &mut category.interfaces {
                                            if interface.id == ri.id {
                                                interface.name = Some(ri.title.clone());
                                                interface.path = Some(ri.path.clone());

                                                find = true;
                                                break;
                                            }
                                        }

                                        if !find {
                                            category.interfaces.push(InterfaceShort {
                                                id: ri.id.clone(),
                                                name: Some(ri.title.clone()),
                                                path: Some(ri.path.clone())
                                            });
                                        }

                                        find = true;
                                    }
                                }
                            }
                        }

                        config.write_config(&config_json);
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            None => {
                println!("no config");
            }
        }
    }

    async fn clear(&self) {
        *self.result_queue.lock().await = ResultQueue {
            list: vec![],
            over_count: 0,
            processing_count: 0,
            over_count_group: 0,
        };
        *self.total_count.lock().await = 0;
    }
}
