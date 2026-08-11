//! Filesystem adapter for the `collect` command.

use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::Args;
use norm_spec_core::{
    CollectResponse, CollectedNorm, ErrorDetail, ParseOptions, collect_candidate_paths, parse_norm,
    project_path,
};

use crate::{
    output::{EXIT_OPERATION, EXIT_USAGE, emit_error, emit_json},
    paths::portable_path,
};

#[derive(Debug, Args)]
pub(crate) struct CollectArgs {
    /// Project root that contains every collected convention.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Directory or file whose inherited conventions should be collected.
    #[arg(long)]
    target: Option<PathBuf>,
    /// Accept the pre-A1 `# Title` followed by YAML format.
    #[arg(long)]
    legacy_format: bool,
    /// Pretty-print the JSON response.
    #[arg(long)]
    pretty: bool,
}

struct CollectionRequest {
    root: PathBuf,
    relative_target: PathBuf,
    target_is_file: bool,
}

struct CollectFailure {
    error: ErrorDetail,
    exit_code: u8,
}

pub(crate) fn run(args: &CollectArgs) -> ExitCode {
    let request = match resolve_request(args) {
        Ok(request) => request,
        Err(failure) => return emit_failure(failure, args.pretty),
    };
    let norms = match collect_norms(&request, args.legacy_format) {
        Ok(norms) => norms,
        Err(failure) => return emit_failure(failure, args.pretty),
    };
    emit_json(
        &CollectResponse::new(project_path(&request.relative_target), norms),
        args.pretty,
        0,
    )
}

fn resolve_request(args: &CollectArgs) -> Result<CollectionRequest, CollectFailure> {
    let target_input = args.target.as_deref().ok_or_else(|| CollectFailure {
        error: ErrorDetail::new(
            "norm/usage/missing-argument",
            "The collect command requires --target.",
        )
        .with_field("--target"),
        exit_code: EXIT_USAGE,
    })?;
    let root = canonical_root(&args.root)?;
    let target = canonical_target(&root, target_input)?;
    if !target.starts_with(&root) {
        return Err(CollectFailure {
            error: ErrorDetail::new(
                "norm/path/outside-root",
                "The collection target resolves outside the project root.",
            )
            .with_path(portable_path(&target)),
            exit_code: EXIT_USAGE,
        });
    }

    let relative_target = target.strip_prefix(&root).map_err(|error| CollectFailure {
        error: ErrorDetail::new(
            "norm/path/outside-root",
            format!("The collection target is not relative to its root: {error}"),
        )
        .with_path(portable_path(&target)),
        exit_code: EXIT_USAGE,
    })?;
    let target_is_file = target.is_file();
    if !target_is_file && !target.is_dir() {
        return Err(CollectFailure {
            error: ErrorDetail::new(
                "norm/path/not-directory",
                "The collection target is neither a file nor a directory.",
            )
            .with_path(project_path(relative_target)),
            exit_code: EXIT_OPERATION,
        });
    }

    Ok(CollectionRequest {
        root,
        relative_target: relative_target.to_path_buf(),
        target_is_file,
    })
}

fn canonical_root(input: &Path) -> Result<PathBuf, CollectFailure> {
    let root = fs::canonicalize(input).map_err(|error| CollectFailure {
        error: path_error(input, error.kind(), "project root", &error.to_string()),
        exit_code: EXIT_OPERATION,
    })?;
    if !root.is_dir() {
        return Err(CollectFailure {
            error: ErrorDetail::new(
                "norm/path/not-directory",
                "The collection root is not a directory.",
            )
            .with_path(portable_path(&root)),
            exit_code: EXIT_OPERATION,
        });
    }
    Ok(root)
}

fn canonical_target(root: &Path, input: &Path) -> Result<PathBuf, CollectFailure> {
    let requested = if input.is_absolute() {
        input.to_path_buf()
    } else {
        root.join(input)
    };
    fs::canonicalize(requested).map_err(|error| {
        let display = if input.is_absolute() {
            portable_path(input)
        } else {
            project_path(input)
        };
        CollectFailure {
            error: ErrorDetail::new(
                if error.kind() == ErrorKind::NotFound {
                    "norm/path/not-found"
                } else {
                    "norm/path/read"
                },
                format!("The collection target is unavailable: {error}"),
            )
            .with_path(display),
            exit_code: EXIT_OPERATION,
        }
    })
}

fn collect_norms(
    request: &CollectionRequest,
    legacy_format: bool,
) -> Result<Vec<CollectedNorm>, CollectFailure> {
    let candidates = collect_candidate_paths(&request.relative_target, request.target_is_file)
        .map_err(|error| CollectFailure {
            error: ErrorDetail::new("norm/path/outside-root", error.to_string())
                .with_path(project_path(&request.relative_target)),
            exit_code: EXIT_USAGE,
        })?;
    let mut norms = Vec::new();
    for candidate in candidates {
        if let Some(collected) = read_candidate(&request.root, &candidate, legacy_format)? {
            norms.push(collected);
        }
    }
    Ok(norms)
}

fn read_candidate(
    root: &Path,
    candidate: &Path,
    legacy_format: bool,
) -> Result<Option<CollectedNorm>, CollectFailure> {
    let path = root.join(candidate);
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(CollectFailure {
                error: path_error(
                    candidate,
                    error.kind(),
                    "convention file",
                    &error.to_string(),
                ),
                exit_code: EXIT_OPERATION,
            });
        }
    };
    if metadata.file_type().is_symlink() {
        return Err(CollectFailure {
            error: ErrorDetail::new(
                "norm/path/symlink-norm",
                "A .norm convention file must not be a symbolic link.",
            )
            .with_path(project_path(candidate)),
            exit_code: EXIT_OPERATION,
        });
    }

    let contents = fs::read_to_string(path).map_err(|error| CollectFailure {
        error: path_error(
            candidate,
            error.kind(),
            "convention file",
            &error.to_string(),
        ),
        exit_code: EXIT_OPERATION,
    })?;
    let parsed =
        parse_norm(&contents, ParseOptions { legacy_format }).map_err(|error| CollectFailure {
            error: ErrorDetail::new(error.code(), error.message())
                .with_path(project_path(candidate)),
            exit_code: EXIT_OPERATION,
        })?;
    Ok(Some(CollectedNorm::new(project_path(candidate), parsed)))
}

fn path_error(path: &Path, kind: ErrorKind, subject: &str, detail: &str) -> ErrorDetail {
    ErrorDetail::new(
        if kind == ErrorKind::NotFound {
            "norm/path/not-found"
        } else {
            "norm/path/read"
        },
        format!("The {subject} is unavailable: {detail}"),
    )
    .with_path(project_path(path))
}

fn emit_failure(failure: CollectFailure, pretty: bool) -> ExitCode {
    emit_error("collect", failure.error, pretty, failure.exit_code)
}
