use std::path::PathBuf;

pub fn path_buf_to_string(input: PathBuf) -> String {
    input.to_string_lossy().to_string()
}

pub fn string_to_path_buf(input: String) -> PathBuf {
    PathBuf::from(input)
}

pub fn string_to_u8_slice(input: &String) -> &[u8] {
    input.as_bytes()
}