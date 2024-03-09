use std::{fs, io::Write};

use tauri::AppHandle;

use super::storage::get_app_log_dir;

const LOG_NAME: &str = "log.txt";

pub fn log_error<T>(app_handle: &AppHandle, contents: String) -> Result<T, std::string::String> {
    log(app_handle, format!("Error: {}", contents));
    Err(contents)
}

pub fn log(app_handle: &AppHandle, contents: String) {
    let app_log_dir = get_app_log_dir(app_handle);

    if let Some(log_dir) = app_log_dir {
        if let Err(e) = fs::create_dir_all(&log_dir) {
            println!("create_dir_all error: {:?}", e);
        };

        // add time
        let contents = format!(
            "{} {}\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            contents
        );

        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_dir.join(LOG_NAME));

        match file {
            Ok(mut file) => {
                if let Err(e) = file.write_all(contents.as_bytes()) {
                    println!("write log error: {:?}", e);
                }
            }
            Err(e) => {
                println!("open log error: {:?}", e);
            }
        }
    }
}
