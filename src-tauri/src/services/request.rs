use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use crate::models::file::FileTree;

use super::yapi::{config::get_project_config, resolver::common::get_legal_name};
//  ----------------------------

pub fn get_file_tree(
    source_path: &str,
    search_path: Option<PathBuf>,
) -> Result<Box<Vec<FileTree>>, io::Error> {
    let project_config = get_project_config(source_path)?;
    let types_full_path = PathBuf::from(source_path).join(project_config.types_path);
    let search_path = match search_path {
        Some(path) => path,
        None => types_full_path,
    };
    let read_dir = fs::read_dir(search_path)?;

    let list: Vec<_> = read_dir
        .into_iter()
        .map(|dir_result| {
            let dir = dir_result.unwrap();
            let file_type = dir.file_type().unwrap();
            let file_path = dir.path();
            let file_name = dir.file_name().into_string().unwrap();

            if file_type.is_file() {
                let node = FileTree {
                    full_path: file_path.clone(),
                    name: file_name.clone(),
                    children: Box::new(vec![]),
                };
                node
            } else {
                let node = FileTree {
                    full_path: file_path.clone(),
                    name: file_name.clone(),
                    children: match get_file_tree(source_path, Some(dir.path())) {
                        Ok(children) => children,
                        Err(_) => Box::new(vec![]),
                    },
                };
                node
            }
        })
        .collect();

    Ok(Box::new(list))
}

pub fn get_request_ts_string(source_path: &str, path: &PathBuf) -> Result<String, io::Error> {
    let project_config = get_project_config(source_path)?;
    let sub_path = get_type_relative_path(source_path, project_config.types_path, path);

    let mut ts_string = format!("{}\n", project_config.header_template.clone());

    let mut import_list: Vec<String> = vec![];
    let mut export_list: Vec<String> = vec![];

    for dir in fs::read_dir(path)?.into_iter() {
        let dir = dir?;
        let file_type = dir.file_type()?;
        let file_path = dir.path();
        let file_name_without_ext = get_file_name_without_ext(&file_path);

        if file_type.is_file() {
            if check_file(&file_path, &file_name_without_ext) {
                op_import_list(
                    project_config.type_import_template.clone(),
                    &sub_path,
                    &file_name_without_ext,
                    &mut import_list,
                );
                op_export_list(
                    project_config.request_template.clone(),
                    &file_path,
                    &sub_path,
                    &mut export_list,
                );
            }
        }
    }

    if import_list.len() == 0 && export_list.len() == 0 {
        return Ok(String::new());
    }

    for str in import_list {
        ts_string += str.as_str();
    }

    for str in export_list {
        ts_string += str.as_str();
    }

    Ok(ts_string)
}

pub fn write_request_ts_file(
    source_path: &str,
    path: &PathBuf,
    content: String,
) -> Result<(), io::Error> {
    let project_config = get_project_config(source_path)?;
    let sub_path = get_type_relative_path(source_path, project_config.types_path, path);
    let write_path = get_write_path(
        project_config.request_path,
        source_path,
        project_config.file_name_template,
        &sub_path,
    );
    let parent = write_path.parent().unwrap();

    fs::create_dir_all(parent)?;

    let mut file = fs::File::create(format!("{}.ts", write_path.to_str().unwrap()))?;

    file.write_all(content.as_bytes())?;

    Ok(())
}

// 获取 type 文件的相对路径
fn get_type_relative_path(source_path: &str, type_path: String, path: &PathBuf) -> Option<PathBuf> {
    let types_root_path = PathBuf::from(source_path).join(type_path);
    match path == &types_root_path {
        true => None,
        false => Some(path.strip_prefix(&types_root_path).unwrap().to_path_buf()),
    }
}

// 获取写入 request 文件的路径
fn get_write_path(
    request_path: String,
    source_path: &str,
    file_name_template: String,
    sub_path: &Option<PathBuf>,
) -> PathBuf {
    let request_full_path = PathBuf::from(source_path).join(request_path);
    match &sub_path {
        Some(sub_path) => {
            let mut write_path = request_full_path.join(&sub_path);

            if let Some(file_name) = write_path.file_name() {
                let real_file_name = file_name_template.replace("$1", file_name.to_str().unwrap());
                write_path.set_file_name(real_file_name);
            }

            write_path
        }
        None => request_full_path.join("index"),
    }
}

// 检查用于生成 request 的 type 文件是否有 Request/Response interface
fn check_file(file_path: &PathBuf, file_name_without_ext: &String) -> bool {
    let req = format!("{}Request", get_legal_name(file_name_without_ext));
    let resp = format!("{}Response", get_legal_name(file_name_without_ext));

    if is_string_in_file(&file_path, &req) && is_string_in_file(&file_path, &resp) {
        return true;
    }
    false
}

pub fn is_string_in_file(ts_file: &PathBuf, string: &str) -> bool {
    let file = File::open(ts_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line.unwrap().contains(string) {
            return true;
        }
    }
    false
}

// 把import ts 定义字符串添加进import_list
fn op_import_list(
    type_import_template: String,
    sub_path: &Option<PathBuf>,
    file_name: &String,
    import_list: &mut Vec<String>,
) {
    let sub_path_unix = get_sub_path_unix(sub_path);

    let import_string = type_import_template
        .replace("$1", &format!("{}Request", get_legal_name(&file_name)))
        .replace("$2", &format!("{}Response", get_legal_name(&file_name)))
        .replace("$3", &format!("{}/{}", sub_path_unix, file_name))
        + "\n";

    import_list.push(import_string);
}

// 把export ts 定义字符串添加进export_list
fn op_export_list(
    request_template: String,
    file_path: &PathBuf,
    sub_path: &Option<PathBuf>,
    export_list: &mut Vec<String>,
) {
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
