use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use tauri::{Manager, State};

use crate::structs::{
    context::ContextGlobal,
    request::{Request, RequestNode},
};

use super::common::{CustomResponse, SearchRequest};

#[tauri::command]
pub async fn check_request_config(handle: tauri::AppHandle) -> Result<CustomResponse, String> {
    let context_global: State<'_, ContextGlobal> = handle.state();
    let source_path = context_global.source_path.lock().await.clone().unwrap();

    let request = Request::from_source_path(source_path);

    if request.request_full_path.is_none()
        || request.header_template.is_none()
        || request.request_template.is_none()
        || request.type_import_template.is_none()
    {
        return Err("请初始化数据".to_string());
    }

    Ok(CustomResponse {
        message: "已读取配置".to_string(),
        data: None,
    })
}

#[tauri::command]
pub async fn get_request_list(handle: tauri::AppHandle) -> Result<CustomResponse, String> {
    let context_global: State<'_, ContextGlobal> = handle.state();
    let source_path = context_global.source_path.lock().await.clone().unwrap();

    let request = Request::from_source_path(source_path);

    let list = request.get_node_list(None);

    let root_node = RequestNode {
        full_path: request.context.types_full_path.clone().unwrap(),
        name: "root".to_string(),
        children: list,
    };

    Ok(CustomResponse {
        message: "获取成功".to_string(),
        data: Some(json!(Vec::from([root_node]))),
    })
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateRequest {
    full_path: Vec<String>,
}

#[tauri::command]
pub async fn update_request(
    data: String,
    handle: tauri::AppHandle,
) -> Result<CustomResponse, String> {
    let form = from_str::<UpdateRequest>(&data).unwrap();
    let context_global: State<'_, ContextGlobal> = handle.state();
    let source_path = context_global.source_path.lock().await.clone().unwrap();

    let request = Request::from_source_path(source_path);

    for fp in form.full_path {
        let path = PathBuf::from(fp);
        request.write_ts(&path);
    }

    Ok(CustomResponse {
        message: "已成功写入ts文件".to_string(),
        data: None,
    })
}

//  ----------------------------

// 获取 type 文件的相对路径
fn get_type_relative_path(&self, path: &PathBuf) -> Option<PathBuf> {
    let types_root_path = self.context.types_full_path.clone().unwrap();
    match path == &types_root_path {
        true => None,
        false => Some(path.strip_prefix(&types_root_path).unwrap().to_path_buf()),
    }
}

// 获取写入 service 文件的路径
fn get_write_path(&self, sub_path: &Option<PathBuf>) -> PathBuf {
    match &sub_path {
        Some(sub_path) => {
            let mut write_path = self.request_full_path.clone().unwrap().join(&sub_path);

            if let Some(template) = self.file_name_template.clone() {
                if let Some(file_name) = write_path.file_name() {
                    let real_file_name = template.replace("$1", file_name.to_str().unwrap());
                    write_path.set_file_name(real_file_name);
                }
            }

            write_path
        }
        None => self.request_full_path.clone().unwrap().join("index"),
    }
}

// 检查生成service的 type 文件是否有 Request/Response interface
fn check_file(&self, file_path: &PathBuf, file_name_without_ext: &String) -> bool {
    let req = format!("{}Request", get_legal_name(file_name_without_ext));
    let resp = format!("{}Response", get_legal_name(file_name_without_ext));

    if is_string_in_file(&file_path, &req) && is_string_in_file(&file_path, &resp) {
        return true;
    }
    false
}

// 把import ts 定义字符串添加进import_list
fn op_import_list(
    type_import_template: String,
    sub_path: &Option<PathBuf>,
    file_name: &String,
) -> Vec<String> {
    let mut import_list = vec![];

    let sub_path_unix = get_sub_path_unix(sub_path);

    let import_string = type_import_template
        .replace("$1", &format!("{}Request", get_legal_name(&file_name)))
        .replace("$2", &format!("{}Response", get_legal_name(&file_name)))
        .replace("$3", &format!("{}/{}", sub_path_unix, file_name))
        + "\n";

    import_list.push(import_string);

    import_list
}

// 把export ts 定义字符串添加进export_list
fn get_export_list(
    request_template: String,
    file_path: &PathBuf,
    sub_path: &Option<PathBuf>,
) -> Vec<String> {
    let mut export_list = vec![];

    let comment = get_comment(file_path.clone());

    let sub_path_unix = get_sub_path_unix(sub_path);

    let file_name = get_file_name_without_ext(&file_path);

    let export_string = request_template
        .replace("$1", &file_name)
        .replace("$2", &format!("{}Request", get_legal_name(&file_name)))
        .replace("$3", &format!("{}Response", get_legal_name(&file_name)))
        .replace("$4", &format!("{}/{}", sub_path_unix.as_str(), &file_name))
        + "\n";

    let export_string_with_comment = format!("{}\n{}", comment, export_string);

    export_list.push(export_string_with_comment);

    export_list
}

fn get_sub_path_unix(sub_path: &Option<PathBuf>) -> String {
    match sub_path {
        Some(sub_path) => format!(
            "/{}",
            sub_path
                .to_str()
                .unwrap()
                .replace("\\", "/")
                .replace("//", "/")
        ),
        None => "".to_string(),
    }
}

fn get_comment(ts_file: PathBuf) -> String {
    let file = File::open(ts_file).unwrap();
    let reader = BufReader::new(file);

    let mut iteror = reader.lines().into_iter();
    let first_line = iteror.next().unwrap().unwrap();

    if first_line.starts_with("//") {
        return first_line;
    }

    String::from("")
}

fn get_file_name_without_ext(file_path: &PathBuf) -> String {
    let file_name_osstr = Path::file_stem(&file_path).unwrap();
    file_name_osstr.to_os_string().into_string().unwrap()
}
