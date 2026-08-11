//! Entry point for the `norm` command.

mod collect;
mod init;
mod output;
mod paths;
mod schemas;
mod templates;
mod validate;

use std::{fs, path::PathBuf, process::ExitCode};

use clap::{Args, Parser, Subcommand};
use norm_spec_core::{ErrorDetail, ParseOptions, ParseResponse, parse_norm};

use crate::{
    collect::CollectArgs,
    init::InitArgs,
    output::{EXIT_OPERATION, EXIT_USAGE, emit_error, emit_json},
    paths::portable_path,
    validate::ValidateArgs,
};

#[derive(Debug, Parser)]
#[command(
    name = "norm",
    version,
    about = "Read and validate directory conventions from .norm files",
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Parse a .norm file into a versioned JSON response.
    Parse(ParseArgs),
    /// Collect inherited .norm files for a target directory.
    Collect(CollectArgs),
    /// Validate one or more .norm files.
    Validate(ValidateArgs),
    /// Create a .norm file from a profile template.
    Init(InitArgs),
    /// Scan a directory without inferring conventions.
    Scan,
}

#[derive(Debug, Args)]
struct ParseArgs {
    /// .norm file or directory containing one.
    path: Option<PathBuf>,
    /// Accept the pre-A1 `# Title` followed by YAML format.
    #[arg(long)]
    legacy_format: bool,
    /// Pretty-print the JSON response.
    #[arg(long)]
    pretty: bool,
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Parse(args) => run_parse(args),
        Command::Collect(args) => collect::run(&args),
        Command::Validate(args) => validate::run(&args),
        Command::Init(args) => init::run(&args),
        Command::Scan => unimplemented_command("scan"),
    }
}

fn run_parse(args: ParseArgs) -> ExitCode {
    let Some(input_path) = args.path else {
        let error = ErrorDetail::new(
            "norm/usage/missing-argument",
            "The parse command requires a path.",
        )
        .with_field("path");
        return emit_error("parse", error, args.pretty, EXIT_USAGE);
    };

    let path = if input_path.is_dir() {
        input_path.join(".norm")
    } else {
        input_path
    };
    let display_path = portable_path(&path);

    if !path.exists() {
        let error = ErrorDetail::new(
            "norm/path/not-found",
            format!("The input path does not exist: {display_path}"),
        )
        .with_path(display_path);
        return emit_error("parse", error, args.pretty, EXIT_OPERATION);
    }

    let contents = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(error) => {
            let detail = ErrorDetail::new(
                "norm/path/read",
                format!("The input path could not be read: {error}"),
            )
            .with_path(display_path);
            return emit_error("parse", detail, args.pretty, EXIT_OPERATION);
        }
    };

    let parsed = match parse_norm(
        &contents,
        ParseOptions {
            legacy_format: args.legacy_format,
        },
    ) {
        Ok(parsed) => parsed,
        Err(error) => {
            let detail = ErrorDetail::new(error.code(), error.message()).with_path(display_path);
            return emit_error("parse", detail, args.pretty, EXIT_OPERATION);
        }
    };

    emit_json(&ParseResponse::from(parsed), args.pretty, 0)
}

fn unimplemented_command(command: &str) -> ExitCode {
    eprintln!("norm {command} is not implemented yet");
    ExitCode::from(EXIT_USAGE)
}
