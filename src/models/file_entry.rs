use std::path::PathBuf;

use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileEntry {
    pub path: PathBuf,
    pub filename: String,
    pub size: u64,
    pub extension: String,
    pub modified_at: DateTime<Local>,
    pub created_at: DateTime<Local>,
    pub last_access_at: DateTime<Local>,
}
