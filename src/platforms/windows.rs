use std::path::PathBuf;

use windows::core::PCWSTR;
use windows::Win32::Storage::FileSystem::{GetDriveTypeW, GetLogicalDrives};

const DRIVE_FIXED: u32 = 3;

pub fn roots() -> Vec<PathBuf> {
    let mounted = unsafe { GetLogicalDrives() };
    let mut out = Vec::new();

    for i in 0u8..26 {
        let is_mounted = mounted & (1 << i) != 0;
        if !is_mounted {
            continue;
        }

        let letter = (b'A' + i) as char;
        let root = format!("{letter}:\\");

        if drive_type(&root) == DRIVE_FIXED {
            out.push(PathBuf::from(root));
        }
    }

    out
}

fn drive_type(root: &str) -> u32 {
    let wide = to_wide_nul(root);
    unsafe { GetDriveTypeW(PCWSTR(wide.as_ptr())) }
}

fn to_wide_nul(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
