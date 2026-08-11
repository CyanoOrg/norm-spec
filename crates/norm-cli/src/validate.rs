//! Filesystem adapter and presentation for the `validate` command.

use std::{
    fs,
    io::{self, ErrorKind},
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::Args;
use norm_spec_core::{
    Diagnostic, ErrorDetail, ParseOptions, ReferenceResolver, ReferenceStatus, SchemaBundle,
    ValidateResponse, ValidationOptions, ValidationResult, ValidationStatus, parse_norm,
    project_path, validate_frontmatter,
};

use crate::{
    output::{EXIT_OPERATION, EXIT_USAGE, emit_error, emit_json},
    paths::portable_path,
    schemas,
};

#[derive(Debug, Args)]
pub(crate) struct ValidateArgs {
    /// .norm file or directory containing one.
    path: Option<PathBuf>,
    /// Recursively validate every .norm under --root.
    #[arg(long)]
    all: bool,
    /// Project root used for containment and output paths.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Explicit directory containing norm-schema.json and profiles/.
    #[arg(long)]
    schema_dir: Option<PathBuf>,
    /// Override metadata-based profile selection.
    #[arg(long)]
    profile: Option<String>,
    #[command(flatten)]
    behavior: ValidateBehaviorArgs,
    #[command(flatten)]
    output: ValidateOutputArgs,
}

#[derive(Debug, Args)]
struct ValidateBehaviorArgs {
    /// Return exit 1 when warnings are present.
    #[arg(long)]
    strict: bool,
    /// Accept the pre-A1 `# Title` followed by YAML format.
    #[arg(long)]
    legacy_format: bool,
    /// Downgrade unknown top-level keys to warnings.
    #[arg(long)]
    compat_keys: bool,
}

#[derive(Debug, Args)]
struct ValidateOutputArgs {
    /// Emit the versioned machine response instead of human output.
    #[arg(long)]
    json: bool,
    /// Pretty-print the JSON response; requires --json.
    #[arg(long)]
    pretty: bool,
}

struct ValidationRequest {
    root: PathBuf,
    files: Vec<PathBuf>,
    schemas: SchemaBundle,
}

struct ValidateFailure {
    error: ErrorDetail,
    exit_code: u8,
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

pub(crate) fn run(args: &ValidateArgs) -> ExitCode {
    if let Some(failure) = validate_arguments(args) {
        return emit_failure(args, failure);
    }
    let request = match resolve_request(args) {
        Ok(request) => request,
        Err(failure) => return emit_failure(args, failure),
    };
    let response = match evaluate(&request, args) {
        Ok(response) => response,
        Err(failure) => return emit_failure(args, failure),
    };
    let failed =
        response.summary.errors > 0 || (args.behavior.strict && response.summary.warnings > 0);
    let exit_code = if failed { EXIT_OPERATION } else { 0 };
    if args.output.json {
        emit_json(&response, args.output.pretty, exit_code)
    } else {
        emit_human(&response);
        ExitCode::from(exit_code)
    }
}

fn validate_arguments(args: &ValidateArgs) -> Option<ValidateFailure> {
    if args.path.is_some() && args.all {
        return Some(ValidateFailure {
            error: ErrorDetail::new(
                "norm/usage/conflicting-arguments",
                "The positional path conflicts with --all.",
            )
            .with_field("path|--all"),
            exit_code: EXIT_USAGE,
        });
    }
    if args.output.pretty && !args.output.json {
        return Some(ValidateFailure {
            error: ErrorDetail::new(
                "norm/usage/conflicting-arguments",
                "--pretty requires --json.",
            )
            .with_field("--pretty|--json"),
            exit_code: EXIT_USAGE,
        });
    }
    if args.path.is_none() && !args.all {
        return Some(ValidateFailure {
            error: ErrorDetail::new(
                "norm/usage/missing-argument",
                "The validate command requires a path or --all.",
            )
            .with_field("path|--all"),
            exit_code: EXIT_USAGE,
        });
    }
    None
}

fn resolve_request(args: &ValidateArgs) -> Result<ValidationRequest, ValidateFailure> {
    let root = canonical_root(&args.root)?;
    let files = if args.all {
        discover_norms(&root)?
    } else {
        vec![resolve_input(
            &root,
            args.path.as_deref().ok_or_else(|| ValidateFailure {
                error: ErrorDetail::new(
                    "norm/usage/missing-argument",
                    "The validate command requires a path or --all.",
                )
                .with_field("path|--all"),
                exit_code: EXIT_USAGE,
            })?,
        )?]
    };
    let schemas = schemas::load(args.schema_dir.as_deref()).map_err(|error| {
        let display_path = error
            .path()
            .map(portable_path)
            .or_else(|| args.schema_dir.as_deref().map(portable_path));
        let mut detail = ErrorDetail::new(
            "norm/usage/schema-not-found",
            format!("The schema bundle is unavailable: {error}"),
        );
        if let Some(path) = display_path {
            detail = detail.with_path(path);
        }
        ValidateFailure {
            error: detail,
            exit_code: EXIT_USAGE,
        }
    })?;
    Ok(ValidationRequest {
        root,
        files,
        schemas,
    })
}

fn canonical_root(input: &Path) -> Result<PathBuf, ValidateFailure> {
    let root = fs::canonicalize(input).map_err(|error| ValidateFailure {
        error: path_error(input, error.kind(), "validation root", &error.to_string()),
        exit_code: EXIT_OPERATION,
    })?;
    if !root.is_dir() {
        return Err(ValidateFailure {
            error: ErrorDetail::new(
                "norm/path/not-directory",
                "The validation root is not a directory.",
            )
            .with_path(portable_path(&root)),
            exit_code: EXIT_OPERATION,
        });
    }
    Ok(root)
}

fn resolve_input(root: &Path, input: &Path) -> Result<PathBuf, ValidateFailure> {
    let metadata = fs::symlink_metadata(input).map_err(|error| ValidateFailure {
        error: path_error(input, error.kind(), "validation input", &error.to_string()),
        exit_code: EXIT_OPERATION,
    })?;
    let candidate = if metadata.is_dir() {
        input.join(".norm")
    } else {
        input.to_path_buf()
    };
    reject_norm_symlink(root, &candidate)?;
    let canonical = fs::canonicalize(&candidate).map_err(|error| ValidateFailure {
        error: path_error(
            &candidate,
            error.kind(),
            "validation input",
            &error.to_string(),
        ),
        exit_code: EXIT_OPERATION,
    })?;
    if !canonical.starts_with(root) {
        return Err(ValidateFailure {
            error: ErrorDetail::new(
                "norm/path/outside-root",
                "The validation input resolves outside the project root.",
            )
            .with_path(portable_path(&canonical)),
            exit_code: EXIT_USAGE,
        });
    }
    Ok(canonical)
}

fn reject_norm_symlink(root: &Path, path: &Path) -> Result<(), ValidateFailure> {
    let metadata = fs::symlink_metadata(path).map_err(|error| ValidateFailure {
        error: path_error(path, error.kind(), "validation input", &error.to_string()),
        exit_code: EXIT_OPERATION,
    })?;
    if metadata.file_type().is_symlink() {
        let display = path
            .strip_prefix(root)
            .map_or_else(|_| portable_path(path), project_path);
        return Err(ValidateFailure {
            error: ErrorDetail::new(
                "norm/path/symlink-norm",
                "A .norm convention file must not be a symbolic link.",
            )
            .with_path(display),
            exit_code: EXIT_OPERATION,
        });
    }
    Ok(())
}

fn discover_norms(root: &Path) -> Result<Vec<PathBuf>, ValidateFailure> {
    let mut files = Vec::new();
    discover_directory(root, root, &mut files)?;
    files.sort();
    Ok(files)
}

fn discover_directory(
    root: &Path,
    directory: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), ValidateFailure> {
    let mut entries = fs::read_dir(directory)
        .map_err(|error| ValidateFailure {
            error: path_error(directory, error.kind(), "directory", &error.to_string()),
            exit_code: EXIT_OPERATION,
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| ValidateFailure {
            error: path_error(
                directory,
                error.kind(),
                "directory entry",
                &error.to_string(),
            ),
            exit_code: EXIT_OPERATION,
        })?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        let file_type = entry.file_type().map_err(|error| ValidateFailure {
            error: path_error(&path, error.kind(), "directory entry", &error.to_string()),
            exit_code: EXIT_OPERATION,
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

fn evaluate(
    request: &ValidationRequest,
    args: &ValidateArgs,
) -> Result<ValidateResponse, ValidateFailure> {
    let mut results = Vec::with_capacity(request.files.len());
    for file in &request.files {
        let relative = file
            .strip_prefix(&request.root)
            .map_err(|error| ValidateFailure {
                error: ErrorDetail::new(
                    "norm/path/outside-root",
                    format!("The validation file is not relative to its root: {error}"),
                )
                .with_path(portable_path(file)),
                exit_code: EXIT_USAGE,
            })?;
        let contents = fs::read_to_string(file).map_err(|error| ValidateFailure {
            error: path_error(
                relative,
                error.kind(),
                "convention file",
                &error.to_string(),
            ),
            exit_code: EXIT_OPERATION,
        })?;
        let parsed = match parse_norm(
            &contents,
            ParseOptions {
                legacy_format: args.behavior.legacy_format,
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
                compat_keys: args.behavior.compat_keys,
                profile: args.profile.as_deref(),
            },
            &resolver,
        )
        .map_err(|error| ValidateFailure {
            error: ErrorDetail::new(
                "norm/usage/schema-not-found",
                format!("The selected schema could not be compiled: {error}"),
            ),
            exit_code: EXIT_USAGE,
        })?;
        results.push(ValidationResult::new(
            project_path(relative),
            diagnostics.errors,
            diagnostics.warnings,
        ));
    }
    Ok(ValidateResponse::new(results))
}

fn canonicalize_allow_missing(path: &Path) -> io::Result<PathBuf> {
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

fn path_error(path: &Path, kind: ErrorKind, subject: &str, detail: &str) -> ErrorDetail {
    ErrorDetail::new(
        if kind == ErrorKind::NotFound {
            "norm/path/not-found"
        } else {
            "norm/path/read"
        },
        format!("The {subject} is unavailable: {detail}"),
    )
    .with_path(portable_path(path))
}

fn emit_failure(args: &ValidateArgs, failure: ValidateFailure) -> ExitCode {
    if args.output.json {
        emit_error(
            "validate",
            failure.error,
            args.output.pretty,
            failure.exit_code,
        )
    } else {
        eprintln!("ERROR [{}]", failure.error.code);
        if let Some(field) = failure.error.field {
            for item in field.split('|') {
                eprintln!("{item}");
            }
        }
        if let Some(path) = failure.error.path {
            eprintln!("{path}");
        }
        eprintln!("{}", failure.error.message);
        ExitCode::from(failure.exit_code)
    }
}

fn emit_human(response: &ValidateResponse) {
    for result in &response.results {
        match result.status {
            ValidationStatus::Ok => println!("OK {}", result.path),
            ValidationStatus::Warning => println!("WARN {}", result.path),
            ValidationStatus::Error => println!("ERROR {}", result.path),
        }
        for diagnostic in &result.errors {
            emit_human_diagnostic("ERROR", diagnostic);
        }
        for diagnostic in &result.warnings {
            emit_human_diagnostic("WARN", diagnostic);
        }
    }
    println!(
        "{} file(s) checked: {} error(s), {} warning(s)",
        response.summary.files, response.summary.errors, response.summary.warnings
    );
}

fn emit_human_diagnostic(level: &str, diagnostic: &Diagnostic) {
    let field = diagnostic
        .field
        .as_deref()
        .map(|field| format!(" {field}:"))
        .unwrap_or_default();
    println!(
        "  {level} [{}]{field} {}",
        diagnostic.code, diagnostic.message
    );
}
