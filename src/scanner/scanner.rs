use walkdir::WalkDir;

use crate::{models::file_entry::FileEntry, platforms};

pub fn scan() -> Vec<FileEntry> {
    let mut entries = Vec::new();
    let root_paths = platforms::roots();
    for root in root_paths {
        let p = match root.to_str() {
            Some(path) => path,
            None => continue
        };

        for entry in WalkDir::new(p) {
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

        println!("Scanned ==> {:?}/{}", entry.path().to_path_buf(), entry.file_name().to_string_lossy().to_string());
        entries.push(FileEntry {
            path: entry.path().to_path_buf(),
            filename: entry.file_name().to_string_lossy().to_string(),
            size: metadata.len(),
        });
        }
    }

    entries
}
