//! Filesystem adapter and presentation for the `init` command.

use std::{
    fs::{self, OpenOptions},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::Args;
use norm_spec_core::{ErrorDetail, InitAction, InitResponse};

use crate::{
    output::{EXIT_OPERATION, EXIT_USAGE, emit_error, emit_json},
    paths::portable_path,
    templates,
};

#[derive(Debug, Args)]
pub(crate) struct InitArgs {
    /// Embedded profile template to write.
    #[arg(long)]
    profile: Option<String>,
    /// Convention-file output path.
    #[arg(long, default_value = ".norm")]
    output: PathBuf,
    /// Replace an existing regular convention file.
    #[arg(long)]
    force: bool,
    /// Emit the versioned machine response instead of human output.
    #[arg(long)]
    json: bool,
    /// Pretty-print the JSON response; requires --json.
    #[arg(long)]
    pretty: bool,
}

struct InitFailure {
    error: ErrorDetail,
    exit_code: u8,
}

pub(crate) fn run(args: &InitArgs) -> ExitCode {
    if args.pretty && !args.json {
        return emit_failure(
            args,
            InitFailure {
                error: ErrorDetail::new(
                    "norm/usage/conflicting-arguments",
                    "--pretty requires --json.",
                )
                .with_field("--pretty|--json"),
                exit_code: EXIT_USAGE,
            },
        );
    }
    let Some(profile) = args.profile.as_deref() else {
        return emit_failure(
            args,
            InitFailure {
                error: ErrorDetail::new(
                    "norm/usage/missing-argument",
                    "The init command requires --profile.",
                )
                .with_field("--profile"),
                exit_code: EXIT_USAGE,
            },
        );
    };
    let Some(contents) = templates::get(profile) else {
        return emit_failure(
            args,
            InitFailure {
                error: ErrorDetail::new(
                    "norm/init/unknown-profile",
                    format!("Unknown init profile '{profile}'."),
                )
                .with_field("--profile"),
                exit_code: EXIT_OPERATION,
            },
        );
    };
    let action = match prepare_output(&args.output, args.force) {
        Ok(action) => action,
        Err(failure) => return emit_failure(args, failure),
    };
    if let Err(failure) = write_template(&args.output, contents, action) {
        return emit_failure(args, failure);
    }

    let path = portable_path(&args.output);
    if args.json {
        emit_json(&InitResponse::new(profile, path, action), args.pretty, 0)
    } else {
        println!("Wrote {path}");
        ExitCode::SUCCESS
    }
}

fn prepare_output(path: &Path, force: bool) -> Result<InitAction, InitFailure> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() {
                return Err(InitFailure {
                    error: ErrorDetail::new(
                        "norm/path/symlink-norm",
                        "An init output convention file must not be a symbolic link.",
                    )
                    .with_path(portable_path(path)),
                    exit_code: EXIT_OPERATION,
                });
            }
            if !force {
                return Err(InitFailure {
                    error: ErrorDetail::new(
                        "norm/init/output-exists",
                        "The init output already exists; use --force to replace it.",
                    )
                    .with_path(portable_path(path)),
                    exit_code: EXIT_OPERATION,
                });
            }
            Ok(InitAction::Overwritten)
        }
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(InitAction::Created),
        Err(error) => Err(path_failure(
            path,
            format!("The init output could not be inspected: {error}"),
        )),
    }
}

fn write_template(path: &Path, contents: &str, action: InitAction) -> Result<(), InitFailure> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent).map_err(|error| {
            path_failure(
                parent,
                format!("The init output directory could not be created: {error}"),
            )
        })?;
    }
    let mut options = OpenOptions::new();
    options.write(true);
    match action {
        InitAction::Created => {
            options.create_new(true);
        }
        InitAction::Overwritten => {
            options.truncate(true);
        }
    }
    let mut output = options.open(path).map_err(|error| {
        if error.kind() == ErrorKind::AlreadyExists {
            InitFailure {
                error: ErrorDetail::new(
                    "norm/init/output-exists",
                    "The init output appeared before it could be created.",
                )
                .with_path(portable_path(path)),
                exit_code: EXIT_OPERATION,
            }
        } else {
            path_failure(
                path,
                format!("The init output could not be opened: {error}"),
            )
        }
    })?;
    output.write_all(contents.as_bytes()).map_err(|error| {
        path_failure(
            path,
            format!("The init template could not be written: {error}"),
        )
    })
}

fn path_failure(path: &Path, message: String) -> InitFailure {
    InitFailure {
        error: ErrorDetail::new("norm/path/read", message).with_path(portable_path(path)),
        exit_code: EXIT_OPERATION,
    }
}

fn emit_failure(args: &InitArgs, failure: InitFailure) -> ExitCode {
    if args.json {
        emit_error("init", failure.error, args.pretty, failure.exit_code)
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
