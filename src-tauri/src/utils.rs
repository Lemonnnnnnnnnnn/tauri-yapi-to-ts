use std::{env, path::PathBuf, fs::File, io::{BufReader, BufRead}};

use serde_json::Value;

pub fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

pub fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn get_err(value: &Value) -> (i64, String) {
    let err_code = value.get("errcode").unwrap().as_i64().unwrap();
    let err_msg = value.get("errmsg").unwrap().as_str().unwrap().to_string();

    (err_code, err_msg)
}

pub fn join_path(source : &PathBuf , sub_path : String) -> PathBuf{
    let path_arr : Vec<_> = sub_path.split("/").collect();
    let mut path = source.clone();
    for sub in path_arr{
        path = path.join(sub);
    }

    path
}

pub fn is_string_in_file(ts_file: &PathBuf, string: &str) -> bool {
    let file = File::open(ts_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line.unwrap().contains(string) {
            return true;
        }
    }
    return false;
}


// #[test]
// fn test() {
// }
