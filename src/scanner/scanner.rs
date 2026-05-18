use walkdir::WalkDir;
use crate::models::file_entry::FileEntry;

pub fn scan_directory(path: &str) -> Vec<FileEntry> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(path) {
       let entry = match entry {
           Ok(e) => e,
           Err(_) => continue,
       };

       if !entry.file_type().is_file() {
           continue;
       }

       let metadata = match entry.metadata() {
         Ok(m) => m,
         Err(_) => continue,
       };

       entries.push(FileEntry {
           path: entry.path().to_path_buf(),
           filename: entry.file_name().to_string_lossy().to_string(),
           size: metadata.len(),
       });
    }

    entries
}
