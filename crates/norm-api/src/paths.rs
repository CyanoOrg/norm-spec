use std::{
    fs, io,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use norm_spec_core::{ErrorDetail, project_path};

use crate::ApiError;

pub(crate) fn canonical_root(input: &Path, subject: &str) -> Result<PathBuf, ApiError> {
    let root = fs::canonicalize(input).map_err(|error| {
        ApiError::operation(path_error(
            None,
            input,
            error.kind(),
            subject,
            &error.to_string(),
        ))
    })?;
    if !root.is_dir() {
        return Err(ApiError::operation(
            ErrorDetail::new(
                "norm/path/not-directory",
                format!("The {subject} is not a directory."),
            )
            .with_path(display_path(None, input)),
        ));
    }
    Ok(root)
}

pub(crate) fn requested_path(root: &Path, input: &Path) -> PathBuf {
    if input.is_absolute() {
        input.to_path_buf()
    } else {
        root.join(input)
    }
}

pub(crate) fn display_path(root: Option<&Path>, input: &Path) -> String {
    if root.is_none() && input.is_relative() {
        return project_path(input);
    }
    let requested = root.map_or_else(|| input.to_path_buf(), |root| requested_path(root, input));
    let comparable = canonicalize_allow_missing(&requested).unwrap_or(requested);
    root.and_then(|root| comparable.strip_prefix(root).ok())
        .map_or_else(|| project_path(&comparable), project_path)
}

pub(crate) fn path_error(
    root: Option<&Path>,
    path: &Path,
    kind: ErrorKind,
    subject: &str,
    detail: &str,
) -> ErrorDetail {
    ErrorDetail::new(
        if kind == ErrorKind::NotFound {
            "norm/path/not-found"
        } else {
            "norm/path/read"
        },
        format!("The {subject} is unavailable: {detail}"),
    )
    .with_path(display_path(root, path))
}

pub(crate) fn canonicalize_allow_missing(path: &Path) -> io::Result<PathBuf> {
    let mut current = path;
    let mut suffix = Vec::new();
    loop {
        match fs::canonicalize(current) {
            Ok(mut canonical) => {
                for component in suffix.iter().rev() {
                    canonical.push(component);
                }
                return Ok(canonical);
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {
                let Some(name) = current.file_name() else {
                    return Err(error);
                };
                suffix.push(name.to_os_string());
                let Some(parent) = current.parent() else {
                    return Err(error);
                };
                current = parent;
            }
            Err(error) => return Err(error),
        }
    }
}
