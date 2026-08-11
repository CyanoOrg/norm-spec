use std::{
    fs,
    path::{Path, PathBuf},
};

use norm_spec_core::{
    Diagnostic, ErrorDetail, ParseOptions, ReferenceResolver, ReferenceStatus, SchemaBundle,
    ValidateResponse, ValidationOptions, ValidationResult, parse_norm, project_path,
    validate_frontmatter,
};

use crate::{
    ApiError,
    assets::{embedded_schema_bundle, schema_bundle_from_dir},
    paths::{canonical_root, canonicalize_allow_missing, display_path, path_error, requested_path},
};

#[derive(Clone, Copy, Debug)]
enum Selection<'a> {
    Path(&'a Path),
    All,
}

/// Explicit inputs for one filesystem-aware validation operation.
#[derive(Clone, Copy, Debug)]
pub struct ValidateRequest<'a> {
    root: &'a Path,
    selection: Selection<'a>,
    schema_dir: Option<&'a Path>,
    profile: Option<&'a str>,
    legacy_format: bool,
    compat_keys: bool,
}

impl<'a> ValidateRequest<'a> {
    /// Validate one `.norm` file, or the `.norm` inside one directory.
    ///
    /// A relative `input` is resolved from `root`.
    #[must_use]
    pub const fn path(root: &'a Path, input: &'a Path) -> Self {
        Self::new(root, Selection::Path(input))
    }

    /// Recursively validate every regular `.norm` file below `root`.
    #[must_use]
    pub const fn all(root: &'a Path) -> Self {
        Self::new(root, Selection::All)
    }

    const fn new(root: &'a Path, selection: Selection<'a>) -> Self {
        Self {
            root,
            selection,
            schema_dir: None,
            profile: None,
            legacy_format: false,
            compat_keys: false,
        }
    }

    /// Use an explicit Schema bundle directory instead of packaged schemas.
    ///
    /// A relative path is resolved from the validation root.
    #[must_use]
    pub const fn schema_dir(mut self, path: &'a Path) -> Self {
        self.schema_dir = Some(path);
        self
    }

    /// Override metadata-based profile selection.
    #[must_use]
    pub const fn profile(mut self, profile: &'a str) -> Self {
        self.profile = Some(profile);
        self
    }

    /// Enable or disable the explicitly supported pre-A1 input format.
    #[must_use]
    pub const fn legacy_format(mut self, enabled: bool) -> Self {
        self.legacy_format = enabled;
        self
    }

    /// Downgrade unknown top-level fields to compatibility warnings.
    #[must_use]
    pub const fn compat_keys(mut self, enabled: bool) -> Self {
        self.compat_keys = enabled;
        self
    }
}

struct ResolvedRequest<'a> {
    root: PathBuf,
    files: Vec<PathBuf>,
    schemas: SchemaBundle,
    profile: Option<&'a str>,
    legacy_format: bool,
    compat_keys: bool,
}

struct FileReferenceResolver<'a> {
    root: &'a Path,
    source_directory: &'a Path,
}

impl ReferenceResolver for FileReferenceResolver<'_> {
    fn resolve(&self, target: &str) -> ReferenceStatus {
        let requested = if let Some(relative) = target.strip_prefix('/') {
            self.root.join(relative)
        } else {
            self.source_directory.join(target)
        };
        let resolved = canonicalize_allow_missing(&requested).unwrap_or(requested);
        if !resolved.starts_with(self.root) {
            ReferenceStatus::OutsideRoot
        } else if resolved.exists() {
            ReferenceStatus::Exists
        } else {
            ReferenceStatus::Missing
        }
    }
}

/// Validate one explicit input or every convention below an explicit root.
///
/// Validation diagnostics are completed results, including when they contain
/// errors. This function returns [`ApiError`] only when path, Schema, or I/O
/// failures prevent evaluation.
///
/// # Errors
///
/// Returns [`ApiError`] for invalid containment or Schema configuration and
/// for filesystem or Schema-compilation failures that prevent evaluation.
pub fn validate(request: ValidateRequest<'_>) -> Result<ValidateResponse, ApiError> {
    let request = resolve_request(request)?;
    evaluate(&request)
}

fn resolve_request(request: ValidateRequest<'_>) -> Result<ResolvedRequest<'_>, ApiError> {
    let root = canonical_root(request.root, "validation root")?;
    let files = match request.selection {
        Selection::Path(input) => vec![resolve_input(&root, input)?],
        Selection::All => discover_norms(&root)?,
    };
    let schema_directory = request.schema_dir.map(|path| requested_path(&root, path));
    let schemas = schema_directory
        .as_deref()
        .map_or_else(embedded_schema_bundle, schema_bundle_from_dir)
        .map_err(|error| {
            let path = error
                .path()
                .map(|path| display_path(Some(&root), path))
                .or_else(|| {
                    request
                        .schema_dir
                        .map(|path| display_path(Some(&root), path))
                });
            let mut detail = ErrorDetail::new(
                "norm/usage/schema-not-found",
                format!("The schema bundle is unavailable: {error}"),
            );
            if let Some(path) = path {
                detail = detail.with_path(path);
            }
            ApiError::usage(detail)
        })?;
    Ok(ResolvedRequest {
        root,
        files,
        schemas,
        profile: request.profile,
        legacy_format: request.legacy_format,
        compat_keys: request.compat_keys,
    })
}

fn resolve_input(root: &Path, input: &Path) -> Result<PathBuf, ApiError> {
    let requested = requested_path(root, input);
    let metadata = fs::symlink_metadata(&requested).map_err(|error| {
        ApiError::operation(path_error(
            Some(root),
            input,
            error.kind(),
            "validation input",
            &error.to_string(),
        ))
    })?;
    let candidate = if metadata.is_dir() {
        requested.join(".norm")
    } else {
        requested
    };
    reject_norm_symlink(root, &candidate)?;
    let canonical = fs::canonicalize(&candidate).map_err(|error| {
        ApiError::operation(path_error(
            Some(root),
            &candidate,
            error.kind(),
            "validation input",
            &error.to_string(),
        ))
    })?;
    if !canonical.starts_with(root) {
        return Err(ApiError::usage(
            ErrorDetail::new(
                "norm/path/outside-root",
                "The validation input resolves outside the project root.",
            )
            .with_path(display_path(Some(root), &canonical)),
        ));
    }
    Ok(canonical)
}

fn reject_norm_symlink(root: &Path, path: &Path) -> Result<(), ApiError> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        ApiError::operation(path_error(
            Some(root),
            path,
            error.kind(),
            "validation input",
            &error.to_string(),
        ))
    })?;
    if metadata.file_type().is_symlink() {
        let path = path
            .strip_prefix(root)
            .map_or_else(|_| display_path(Some(root), path), project_path);
        return Err(ApiError::operation(
            ErrorDetail::new(
                "norm/path/symlink-norm",
                "A .norm convention file must not be a symbolic link.",
            )
            .with_path(path),
        ));
    }
    Ok(())
}

fn discover_norms(root: &Path) -> Result<Vec<PathBuf>, ApiError> {
    let mut files = Vec::new();
    discover_directory(root, root, &mut files)?;
    files.sort();
    Ok(files)
}

fn discover_directory(
    root: &Path,
    directory: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), ApiError> {
    let mut entries = fs::read_dir(directory)
        .map_err(|error| {
            ApiError::operation(path_error(
                Some(root),
                directory,
                error.kind(),
                "directory",
                &error.to_string(),
            ))
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            ApiError::operation(path_error(
                Some(root),
                directory,
                error.kind(),
                "directory entry",
                &error.to_string(),
            ))
        })?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        let file_type = entry.file_type().map_err(|error| {
            ApiError::operation(path_error(
                Some(root),
                &path,
                error.kind(),
                "directory entry",
                &error.to_string(),
            ))
        })?;
        if file_type.is_symlink() {
            if entry.file_name() == ".norm" {
                reject_norm_symlink(root, &path)?;
            }
            continue;
        }
        if file_type.is_dir() {
            discover_directory(root, &path, files)?;
        } else if file_type.is_file() && entry.file_name() == ".norm" {
            files.push(path);
        }
    }
    Ok(())
}

fn evaluate(request: &ResolvedRequest<'_>) -> Result<ValidateResponse, ApiError> {
    let mut results = Vec::with_capacity(request.files.len());
    for file in &request.files {
        let relative = file.strip_prefix(&request.root).map_err(|error| {
            ApiError::usage(
                ErrorDetail::new(
                    "norm/path/outside-root",
                    format!("The validation file is not relative to its root: {error}"),
                )
                .with_path(display_path(Some(&request.root), file)),
            )
        })?;
        let contents = fs::read_to_string(file).map_err(|error| {
            ApiError::operation(path_error(
                Some(&request.root),
                relative,
                error.kind(),
                "convention file",
                &error.to_string(),
            ))
        })?;
        let parsed = match parse_norm(
            &contents,
            ParseOptions {
                legacy_format: request.legacy_format,
            },
        ) {
            Ok(parsed) => parsed,
            Err(error) => {
                results.push(ValidationResult::new(
                    project_path(relative),
                    vec![Diagnostic::new(error.code(), error.message())],
                    Vec::new(),
                ));
                continue;
            }
        };
        let source_directory = file.parent().unwrap_or(&request.root);
        let resolver = FileReferenceResolver {
            root: &request.root,
            source_directory,
        };
        let diagnostics = validate_frontmatter(
            &parsed.frontmatter,
            &request.schemas,
            ValidationOptions {
                compat_keys: request.compat_keys,
                profile: request.profile,
            },
            &resolver,
        )
        .map_err(|error| {
            ApiError::usage(ErrorDetail::new(
                "norm/usage/schema-not-found",
                format!("The selected schema could not be compiled: {error}"),
            ))
        })?;
        results.push(ValidationResult::new(
            project_path(relative),
            diagnostics.errors,
            diagnostics.warnings,
        ));
    }
    Ok(ValidateResponse::new(results))
}
