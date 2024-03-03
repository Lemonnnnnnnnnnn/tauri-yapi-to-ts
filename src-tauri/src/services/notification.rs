use tauri::{AppHandle, Manager};

use crate::models::notification::{Notification, NotificationDesc};

pub fn notification(app_handle: &AppHandle, desc: NotificationDesc, message: &str) {
    app_handle
        .emit_all(
            "notification",
            Notification {
                desc,
                message: message.to_string(),
            },
        )
        .unwrap()
}
