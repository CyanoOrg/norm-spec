//! CLI arguments and presentation for the `validate` command.

use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::Args;
use norm_spec::{
    ApiError, Diagnostic, ErrorDetail, FailureClass, ValidateRequest, ValidateResponse,
    ValidationStatus, validate,
};

use crate::{
    output::{EXIT_OPERATION, EXIT_USAGE, emit_error, emit_json},
    paths::portable_path,
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

struct ValidateFailure {
    error: ErrorDetail,
    exit_code: u8,
}

pub(crate) fn run(args: &ValidateArgs) -> ExitCode {
    if let Some(failure) = validate_arguments(args) {
        return emit_failure(args, failure);
    }
    let input = match args.path.as_deref().map(absolute_path).transpose() {
        Ok(path) => path,
        Err(failure) => return emit_failure(args, failure),
    };
    let schema_dir = match args.schema_dir.as_deref().map(absolute_path).transpose() {
        Ok(path) => path,
        Err(failure) => return emit_failure(args, failure),
    };
    let mut request = if args.all {
        ValidateRequest::all(&args.root)
    } else {
        let Some(input) = input.as_deref() else {
            return emit_failure(args, missing_input());
        };
        ValidateRequest::path(&args.root, input)
    };
    if let Some(path) = schema_dir.as_deref() {
        request = request.schema_dir(path);
    }
    if let Some(profile) = args.profile.as_deref() {
        request = request.profile(profile);
    }
    request = request
        .legacy_format(args.behavior.legacy_format)
        .compat_keys(args.behavior.compat_keys);
    let response = match validate(request) {
        Ok(response) => response,
        Err(error) => return emit_failure(args, api_failure(error)),
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
        return Some(missing_input());
    }
    None
}

fn missing_input() -> ValidateFailure {
    ValidateFailure {
        error: ErrorDetail::new(
            "norm/usage/missing-argument",
            "The validate command requires a path or --all.",
        )
        .with_field("path|--all"),
        exit_code: EXIT_USAGE,
    }
}

fn absolute_path(path: &Path) -> Result<PathBuf, ValidateFailure> {
    std::path::absolute(path).map_err(|error| ValidateFailure {
        error: ErrorDetail::new(
            "norm/path/read",
            format!("The input path could not be resolved: {error}"),
        )
        .with_path(portable_path(path)),
        exit_code: EXIT_OPERATION,
    })
}

fn api_failure(error: ApiError) -> ValidateFailure {
    let exit_code = match error.class() {
        FailureClass::Usage => EXIT_USAGE,
        FailureClass::Operation => EXIT_OPERATION,
    };
    ValidateFailure {
        error: error.into_detail(),
        exit_code,
    }
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
