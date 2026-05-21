use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::platforms;

pub struct IgnoreRules {
    absolute_prefixes: Vec<PathBuf>,
    leaf_names: Vec<OsString>,
}

const DEFAULT_LEAF_NAMES: &[&str] = &[
    "$Recycle.Bin",
    "System Volume Information",
    "pagefile.sys",
    "hiberfil.sys",
    "swapfile.sys",
    ".git",
    ".svn",
    ".hg",
    "node_modules",
    "npm-cache",
    ".pnpm-store",
    ".yarn",
    ".next",
    ".nuxt",
    ".bun",
    ".parcel-cache",
    ".turbo",
    "target",
    ".cargo",
    "venv",
    ".venv",
    ".pyenv",
    ".vscode",
    "__pycache__",
    ".mypy_cache",
    ".pytest_cache",
    ".tox",
    ".gradle",
    ".jest-cache",
    ".mvn",
    "build",
    "out",
    "cmake-build-debug",
    "cmake-build-release",
    ".cache",
    "tmp",
    "temp",
    "logs",
    "log",
    "AppData",
    "go",
];

impl IgnoreRules {
    pub fn defaults() -> Self {
        Self {
            absolute_prefixes: platforms::ignored_paths(),
            leaf_names: DEFAULT_LEAF_NAMES.iter().map(OsString::from).collect(),
        }
    }

    pub fn with_extra_paths(mut self, extras: impl IntoIterator<Item = PathBuf>) -> Self {
        self.absolute_prefixes.extend(extras);
        self
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        if self.absolute_prefixes.iter().any(|p| path.starts_with(p)) {
            return true;
        }
        path.file_name()
            .is_some_and(|n| self.leaf_names.iter().any(|x| x.as_os_str() == n))
    }
}
