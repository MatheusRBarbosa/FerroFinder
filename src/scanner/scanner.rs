use std::{path::Path, time::Instant};

use chrono::{DateTime, Local};
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::{models::file_entry::FileEntry, platforms};

pub fn scan() -> Vec<FileEntry> {
    platforms::roots()
        .par_iter()
        .flat_map_iter(|root| scan_root(root))
        .collect()
}

fn scan_root(root: &Path) -> Vec<FileEntry> {
    let start = Instant::now();
    let mut entries = Vec::new();

    let Some(p) = root.to_str() else {
        return entries;
    };

    for entry in WalkDir::new(p) {
        let Ok(entry) = entry else { continue };

        if !entry.file_type().is_file() {
            continue;
        }

        let Ok(metadata) = entry.metadata() else {
            continue;
        };

        let path = entry.path().to_path_buf();
        let filename = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        let modified_at: DateTime<Local> = metadata
            .modified()
            .map(DateTime::<Local>::from)
            .unwrap_or(Local::now());

        let created_at = metadata
            .created()
            .map(DateTime::<Local>::from)
            .unwrap_or(Local::now());

        let last_access_at = metadata
            .accessed()
            .map(DateTime::<Local>::from)
            .unwrap_or(Local::now());

        entries.push(FileEntry {
            path,
            filename,
            extension,
            size: metadata.len(),
            modified_at,
            created_at,
            last_access_at,
        });
    }

    let duration = start.elapsed();
    println!(
        "{:?} Took {:?}, scanned {} files",
        root,
        duration,
        entries.len()
    );
    entries
}
