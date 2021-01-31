use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Image {
    pub path: PathBuf,
    pub extension: String,
}
