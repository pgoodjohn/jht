pub fn is_stylesheet(path: &std::path::Path) -> bool {
    match path.extension() {
        None => false,
        Some(extension) => match extension.to_str() {
            Some("css") => true,
            _ => false,
        },
    }
}

pub fn is_plaintext_file(path: &std::path::Path) -> bool {
    match path.extension() {
        None => false,
        Some(extension) => match extension.to_str() {
            Some("md") => true,
            Some("txt") => true,
            _ => false,
        },
    }
}
