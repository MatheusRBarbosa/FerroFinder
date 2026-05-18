use std::path::PathBuf;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileEntry {
    pub path: PathBuf,
    pub filename: String,
    pub size: u64
}
