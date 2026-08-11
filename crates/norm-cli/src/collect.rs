//! CLI arguments and presentation for the `collect` command.

use std::{path::PathBuf, process::ExitCode};

use clap::Args;
use norm_spec::{ApiError, CollectRequest, ErrorDetail, FailureClass, collect};

use crate::output::{EXIT_OPERATION, EXIT_USAGE, emit_error, emit_json};

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

pub(crate) fn run(args: &CollectArgs) -> ExitCode {
    let Some(target) = args.target.as_deref() else {
        return emit_error(
            "collect",
            ErrorDetail::new(
                "norm/usage/missing-argument",
                "The collect command requires --target.",
            )
            .with_field("--target"),
            args.pretty,
            EXIT_USAGE,
        );
    };
    match collect(CollectRequest::new(&args.root, target).legacy_format(args.legacy_format)) {
        Ok(response) => emit_json(&response, args.pretty, 0),
        Err(error) => emit_api_error("collect", error, args.pretty),
    }
}

fn emit_api_error(command: &str, error: ApiError, pretty: bool) -> ExitCode {
    let exit_code = match error.class() {
        FailureClass::Usage => EXIT_USAGE,
        FailureClass::Operation => EXIT_OPERATION,
    };
    emit_error(command, error.into_detail(), pretty, exit_code)
}
