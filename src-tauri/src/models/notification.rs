use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum NotificationDesc {
    Success,
    Error,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Notification {
    pub message: String,
    pub desc: NotificationDesc,
}
