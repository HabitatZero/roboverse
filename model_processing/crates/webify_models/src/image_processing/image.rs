//! Structure that represents an image reference

use std::path::PathBuf;
#[derive(Debug, Clone)]
pub struct Image {
    /// File path of the image
    pub path: PathBuf,
    /// File extension of the image
    pub extension: String,
}
