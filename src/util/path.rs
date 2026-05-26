use std::path::Path;

pub fn get_filename_without_ext(path: &str) -> &str {
    Path::new(path)
    .file_stem()
    .and_then(|os_str| os_str.to_str())
    .unwrap_or("")
}

pub fn get_filename(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("")
}