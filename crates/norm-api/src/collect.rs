use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use norm_spec_core::{
    CollectResponse, CollectedNorm, ErrorDetail, ParseOptions, collect_candidate_paths, parse_norm,
    project_path,
};

use crate::{
    ApiError,
    paths::{canonical_root, display_path, path_error, requested_path},
};

/// Explicit inputs for one inherited convention collection.
#[derive(Clone, Copy, Debug)]
pub struct CollectRequest<'a> {
    root: &'a Path,
    target: &'a Path,
    legacy_format: bool,
}

impl<'a> CollectRequest<'a> {
    /// Construct a request whose relative target is resolved from `root`.
    #[must_use]
    pub const fn new(root: &'a Path, target: &'a Path) -> Self {
        Self {
            root,
            target,
            legacy_format: false,
        }
    }

    /// Enable or disable the explicitly supported pre-A1 input format.
    #[must_use]
    pub const fn legacy_format(mut self, enabled: bool) -> Self {
        self.legacy_format = enabled;
        self
    }
}

struct ResolvedRequest {
    root: PathBuf,
    relative_target: PathBuf,
    target_is_file: bool,
    legacy_format: bool,
}

/// Collect inherited `.norm` files from most-specific target to project root.
///
/// # Errors
///
/// Returns [`ApiError`] when the explicit paths are unavailable or escape the
/// root, a convention file is a symbolic link, or a collected file cannot be
/// read or parsed.
pub fn collect(request: CollectRequest<'_>) -> Result<CollectResponse, ApiError> {
    let request = resolve_request(request)?;
    let candidates = collect_candidate_paths(&request.relative_target, request.target_is_file)
        .map_err(|error| {
            ApiError::usage(
                ErrorDetail::new("norm/path/outside-root", error.to_string())
                    .with_path(project_path(&request.relative_target)),
            )
        })?;
    let mut norms = Vec::new();
    for candidate in candidates {
        if let Some(collected) = read_candidate(&request.root, &candidate, request.legacy_format)? {
            norms.push(collected);
        }
    }
    Ok(CollectResponse::new(
        project_path(&request.relative_target),
        norms,
    ))
}

fn resolve_request(request: CollectRequest<'_>) -> Result<ResolvedRequest, ApiError> {
    let root = canonical_root(request.root, "collection root")?;
    let requested = requested_path(&root, request.target);
    let target = fs::canonicalize(&requested).map_err(|error| {
        ApiError::operation(
            ErrorDetail::new(
                if error.kind() == ErrorKind::NotFound {
                    "norm/path/not-found"
                } else {
                    "norm/path/read"
                },
                format!("The collection target is unavailable: {error}"),
            )
            .with_path(display_path(Some(&root), request.target)),
        )
    })?;
    if !target.starts_with(&root) {
        return Err(ApiError::usage(
            ErrorDetail::new(
                "norm/path/outside-root",
                "The collection target resolves outside the project root.",
            )
            .with_path(display_path(Some(&root), &target)),
        ));
    }
    let relative_target = target.strip_prefix(&root).map_err(|error| {
        ApiError::usage(
            ErrorDetail::new(
                "norm/path/outside-root",
                format!("The collection target is not relative to its root: {error}"),
            )
            .with_path(display_path(Some(&root), &target)),
        )
    })?;
    let target_is_file = target.is_file();
    if !target_is_file && !target.is_dir() {
        return Err(ApiError::operation(
            ErrorDetail::new(
                "norm/path/not-directory",
                "The collection target is neither a file nor a directory.",
            )
            .with_path(project_path(relative_target)),
        ));
    }
    Ok(ResolvedRequest {
        root,
        relative_target: relative_target.to_path_buf(),
        target_is_file,
        legacy_format: request.legacy_format,
    })
}

fn read_candidate(
    root: &Path,
    candidate: &Path,
    legacy_format: bool,
) -> Result<Option<CollectedNorm>, ApiError> {
    let path = root.join(candidate);
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(ApiError::operation(path_error(
                Some(root),
                candidate,
                error.kind(),
                "convention file",
                &error.to_string(),
            )));
        }
    };
    if metadata.file_type().is_symlink() {
        return Err(ApiError::operation(
            ErrorDetail::new(
                "norm/path/symlink-norm",
                "A .norm convention file must not be a symbolic link.",
            )
            .with_path(project_path(candidate)),
        ));
    }
    let contents = fs::read_to_string(path).map_err(|error| {
        ApiError::operation(path_error(
            Some(root),
            candidate,
            error.kind(),
            "convention file",
            &error.to_string(),
        ))
    })?;
    let parsed = parse_norm(&contents, ParseOptions { legacy_format }).map_err(|error| {
        ApiError::operation(
            ErrorDetail::new(error.code(), error.message()).with_path(project_path(candidate)),
        )
    })?;
    Ok(Some(CollectedNorm::new(project_path(candidate), parsed)))
}
