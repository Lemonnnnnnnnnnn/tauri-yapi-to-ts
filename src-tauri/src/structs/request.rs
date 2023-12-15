use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::utils::is_string_in_file;

use super::{config::Config, context::Context};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestNode {
    pub full_path: PathBuf,
    pub name: String,
    pub children: Box<Vec<RequestNode>>,
}

pub struct Request {
    pub context: Context,
    /**
     * example:
     * export const getProjectList = (params: GetProjectListRequest) =>
     * request<GetProjectListResponse>('/api/project/getProjectList', { method: 'post', data: params });
     *
     * request_template:
     *
     * export const $1 = (params: $2) =>
     * request<$3>('/api/$4' , { method: 'post' , data: params});
     *
     * replace $1,$2,$3,$4 and get complete string;  
     */
    pub request_template: Option<String>,
    pub header_template: Option<String>,
    pub request_full_path: Option<PathBuf>,
}

impl Request {
    pub fn from_source_path(source_path: PathBuf) -> Self {
        let context = Context::from_source_path(&source_path);
        let config = Config::new(&source_path);
        let config_json = config.read_config().unwrap();

        Self {
            context,
            request_template: config_json.request_template,
            header_template: config_json.header_template,
            request_full_path: config_json.request_full_path,
        }
    }

    pub fn get_node_list(&self, search_path: Option<PathBuf>) -> Box<Vec<RequestNode>> {
        let search_path = match search_path {
            Some(path) => path,
            None => self.context.types_full_path.clone().unwrap(),
        };
        let read_dir = fs::read_dir(search_path).unwrap();

        let list: Vec<_> = read_dir
            .into_iter()
            .map(|dir_result| {
                let dir = dir_result.unwrap();
                let file_type = dir.file_type().unwrap();
                let file_path = dir.path();
                let file_name = dir.file_name().into_string().unwrap();

                if file_type.is_file() {
                    let node = RequestNode {
                        full_path: file_path.clone(),
                        name: file_name.clone(),
                        children: Box::new(vec![]),
                    };
                    node
                } else {
                    let node = RequestNode {
                        full_path: file_path.clone(),
                        name: file_name.clone(),
                        children: self.get_node_list(Some(dir.path())),
                    };
                    node
                }
            })
            .collect();

        Box::new(list)
    }

    pub fn write_ts(&self, path: &PathBuf) {
        let types_root_path = self.context.types_full_path.clone().unwrap();
        let sub_path = match path == &types_root_path {
            true => None,
            false => Some(path.strip_prefix(&types_root_path).unwrap().to_path_buf()),
        };

        println!("sub_path: {:?}", sub_path);

        let read_res = fs::read_dir(path).unwrap();

        let mut ts_string = match &self.header_template {
            Some(template) => format!("{}\n", template.clone()),
            None => String::new(),
        };

        let mut import_list: Vec<String> = vec![];
        let mut export_list: Vec<String> = vec![];

        let write_path = match &sub_path {
            Some(sub_path) => {
                self.request_full_path.clone().unwrap().join(&sub_path)
            },
            None => self.request_full_path.clone().unwrap().join("index"),
        };

        for dirs in read_res.into_iter() {
            let dir = dirs.unwrap();
            let file_type = dir.file_type().unwrap();
            let file_path = dir.path();
            let file_name_with_ext = dir.file_name().into_string().unwrap();
            let file_name_osstr = Path::file_stem(&file_path).unwrap();
            let file_name_without_ext = file_name_osstr.to_os_string().into_string().unwrap();

            if file_type.is_file() {
                if self.check_file(&file_path, &file_name_without_ext) {
                    self.op_import_list(&sub_path, &file_name_without_ext, &mut import_list);
                    self.op_export_list(
                        &file_path,
                        &sub_path,
                        &file_name_without_ext,
                        &mut export_list,
                    );
                }
            } else if file_type.is_dir() {
                let new_dir = path.clone().join(file_name_with_ext);

                self.write_ts(&new_dir);
            }
        }

        if import_list.len() == 0 && export_list.len() == 0 {
            return;
        }

        for str in import_list {
            ts_string += str.as_str();
        }

        for str in export_list {
            ts_string += str.as_str();
        }

        let parent = write_path.parent().unwrap();

        println!("{:#?}", write_path);

        fs::create_dir_all(parent).unwrap();
        fs::write(format!("{}.ts", write_path.to_str().unwrap()), ts_string).unwrap();
    }

    fn check_file(&self, file_path: &PathBuf, file_name_without_ext: &String) -> bool {
        let req = format!("{}Request", file_name_without_ext);
        let resp = format!("{}Response", file_name_without_ext);

        if is_string_in_file(&file_path, &req) && is_string_in_file(&file_path, &resp) {
            return true;
        }
        return false;
    }

    // 把import ts 定义字符串添加进import_list
    fn op_import_list(
        &self,
        sub_path: &Option<PathBuf>,
        file_name_string: &String,
        import_list: &mut Vec<String>,
    ) {
        let sub_path_unix = Self::get_sub_path_unix(sub_path);

        let types_path_pathbuf = self.context.types_path.clone().unwrap();

        let import_string = format!(
            "import {{ {}Request , {}Response }} from \"@/{}{}/{}\"\n",
            file_name_string,
            file_name_string,
            types_path_pathbuf.to_str().unwrap(),
            sub_path_unix,
            file_name_string
        );

        import_list.push(import_string);
    }

    // 把export ts 定义字符串添加进export_list
    fn op_export_list(
        &self,
        file_path: &PathBuf,
        sub_path: &Option<PathBuf>,
        file_name_string: &String,
        export_list: &mut Vec<String>,
    ) {
        let comment = Self::get_comment(file_path.clone());

        let sub_path_unix = Self::get_sub_path_unix(sub_path);

        let export_string = self
            .request_template
            .clone()
            .unwrap()
            .replace("$1", &file_name_string)
            .replace("$2", &format!("{}Request", &file_name_string))
            .replace("$3", &format!("{}Response", &file_name_string))
            .replace(
                "$4",
                &format!("{}{}", sub_path_unix.as_str(), &file_name_string),
            )
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
}

// #[test]
// fn test() {
//     let request = Request::from_source_path(PathBuf::from("D:\\work\\tmp"));
//     let path = PathBuf::from("D:\\work\\tmp\\src\\types\\policy");
//     request.write_ts(&path);
// }
