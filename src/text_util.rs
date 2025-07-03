pub fn strip_format(file_name: &str) -> String {
    if let Some(dot_index) = file_name.rfind('.') {
        file_name[..dot_index].to_string()
    } else {
        file_name.to_string()
    }
}