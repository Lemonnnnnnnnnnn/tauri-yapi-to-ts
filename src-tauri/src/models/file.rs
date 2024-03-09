use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileTree {
    pub full_path: PathBuf,
    pub name: String,
    pub children: Box<Vec<FileTree>>,
}