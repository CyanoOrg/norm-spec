//! Shared CLI path presentation helpers.

use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub(crate) fn portable_path(path: &Path) -> String {
    let comparable_path = canonical_display_path(path);
    let current = env::current_dir()
        .ok()
        .map(|path| canonical_display_path(&path));
    let relative = current
        .as_deref()
        .and_then(|current| comparable_path.strip_prefix(current).ok())
        .unwrap_or(&comparable_path);
    let display = relative.to_string_lossy().replace('\\', "/");
    if display.is_empty() {
        ".".to_owned()
    } else {
        display
    }
}

fn canonical_display_path(path: &Path) -> PathBuf {
    if let Ok(canonical) = fs::canonicalize(path) {
        return canonical;
    }
    let Some(parent) = path.parent() else {
        return path.to_path_buf();
    };
    let Some(name) = path.file_name() else {
        return path.to_path_buf();
    };
    match fs::canonicalize(parent) {
        Ok(parent) => parent.join(name),
        Err(_) => path.to_path_buf(),
    }
}
