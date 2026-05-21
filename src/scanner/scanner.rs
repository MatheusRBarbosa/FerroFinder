use std::{path::Path, time::Instant};

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

        let filename = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path().to_path_buf();

        entries.push(FileEntry {
            path,
            filename,
            size: metadata.len(),
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
