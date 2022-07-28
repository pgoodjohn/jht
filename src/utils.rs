pub fn is_markdown(path: &std::path::Path) -> bool {
    match path.extension() {
        None => false,
        Some(extension) => match extension.to_str() {
            Some("md") => true,
            _ => false,
        },
    }
}

pub fn is_stylesheet(path: &std::path::Path) -> bool {
    match path.extension() {
        None => false,
        Some(extension) => match extension.to_str() {
            Some("css") => true,
            _ => false,
        },
    }
}
