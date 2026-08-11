//! Entry point for the `norm` command.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::{Args, Parser, Subcommand};
use norm_spec_core::{ErrorDetail, ErrorResponse, ParseOptions, ParseResponse, parse_norm};
use serde::Serialize;

const EXIT_OPERATION: u8 = 1;
const EXIT_USAGE: u8 = 2;

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
    Collect,
    /// Validate one or more .norm files.
    Validate,
    /// Create a .norm file from a profile template.
    Init,
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
        Command::Collect => unimplemented_command("collect"),
        Command::Validate => unimplemented_command("validate"),
        Command::Init => unimplemented_command("init"),
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
        return emit_error(error, args.pretty, EXIT_USAGE);
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
        return emit_error(error, args.pretty, EXIT_OPERATION);
    }

    let contents = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(error) => {
            let detail = ErrorDetail::new(
                "norm/path/read",
                format!("The input path could not be read: {error}"),
            )
            .with_path(display_path);
            return emit_error(detail, args.pretty, EXIT_OPERATION);
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
            return emit_error(detail, args.pretty, EXIT_OPERATION);
        }
    };

    emit_json(&ParseResponse::from(parsed), args.pretty, 0)
}

fn emit_error(error: ErrorDetail, pretty: bool, exit_code: u8) -> ExitCode {
    emit_json(&ErrorResponse::new("parse", error), pretty, exit_code)
}

fn emit_json(value: &impl Serialize, pretty: bool, exit_code: u8) -> ExitCode {
    let serialized = if pretty {
        serde_json::to_string_pretty(value)
    } else {
        serde_json::to_string(value)
    };

    match serialized {
        Ok(json) => {
            println!("{json}");
            ExitCode::from(exit_code)
        }
        Err(error) => {
            eprintln!("norm internal serialization failure: {error}");
            ExitCode::from(EXIT_OPERATION)
        }
    }
}

fn portable_path(path: &Path) -> String {
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

fn unimplemented_command(command: &str) -> ExitCode {
    eprintln!("norm {command} is not implemented yet");
    ExitCode::from(EXIT_USAGE)
}
