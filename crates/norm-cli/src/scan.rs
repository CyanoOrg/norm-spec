//! Filesystem traversal and presentation for the `scan` command.

use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::Args;
use norm_spec_core::{
    DirectoryObservation, ErrorDetail, ScanResponse, ScanSymlink, ScanSymlinkKind,
    is_ignored_directory, project_path,
};

use crate::{
    output::{EXIT_OPERATION, EXIT_USAGE, emit_error, emit_json},
    paths::portable_path,
};

#[derive(Debug, Args)]
pub(crate) struct ScanArgs {
    /// Project directory to inspect structurally.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Pretty-print the JSON response.
    #[arg(long)]
    pretty: bool,
    /// Emit a human-readable report instead of JSON.
    #[arg(long)]
    text: bool,
}

struct ScanFailure {
    error: ErrorDetail,
    exit_code: u8,
}

pub(crate) fn run(args: &ScanArgs) -> ExitCode {
    if args.text && args.pretty {
        return emit_human_failure(ScanFailure {
            error: ErrorDetail::new(
                "norm/usage/conflicting-arguments",
                "--text conflicts with --pretty.",
            )
            .with_field("--text|--pretty"),
            exit_code: EXIT_USAGE,
        });
    }
    let response = match scan(&args.root) {
        Ok(response) => response,
        Err(failure) => {
            return if args.text {
                emit_human_failure(failure)
            } else {
                emit_error("scan", failure.error, args.pretty, failure.exit_code)
            };
        }
    };
    if args.text {
        emit_text(&response);
        ExitCode::SUCCESS
    } else {
        emit_json(&response, args.pretty, 0)
    }
}

fn scan(input: &Path) -> Result<ScanResponse, ScanFailure> {
    let root = canonical_root(input)?;
    let mut observations = Vec::new();
    let mut symlinks = Vec::new();
    scan_directory(&root, &root, &mut observations, &mut symlinks)?;
    Ok(ScanResponse::new(observations, symlinks))
}

fn canonical_root(input: &Path) -> Result<PathBuf, ScanFailure> {
    let root = fs::canonicalize(input).map_err(|error| ScanFailure {
        error: ErrorDetail::new(
            if error.kind() == ErrorKind::NotFound {
                "norm/path/not-found"
            } else {
                "norm/path/read"
            },
            format!("The scan root is unavailable: {error}"),
        )
        .with_path(portable_path(input)),
        exit_code: EXIT_OPERATION,
    })?;
    if !root.is_dir() {
        return Err(ScanFailure {
            error: ErrorDetail::new(
                "norm/path/not-directory",
                "The scan root is not a directory.",
            )
            .with_path(portable_path(&root)),
            exit_code: EXIT_OPERATION,
        });
    }
    Ok(root)
}

fn scan_directory(
    root: &Path,
    directory: &Path,
    observations: &mut Vec<DirectoryObservation>,
    symlinks: &mut Vec<ScanSymlink>,
) -> Result<(), ScanFailure> {
    let mut entries = fs::read_dir(directory)
        .map_err(|error| path_failure(directory, "scan directory", &error))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| path_failure(directory, "scan directory entry", &error))?;
    entries.sort_by_key(fs::DirEntry::file_name);

    let mut file_names = Vec::new();
    let mut child_directories = Vec::new();
    for entry in entries {
        let path = entry.path();
        let name = entry.file_name().into_string().map_err(|_| ScanFailure {
            error: ErrorDetail::new(
                "norm/path/read",
                "A scan directory entry name is not valid UTF-8.",
            )
            .with_path(portable_path(&path)),
            exit_code: EXIT_OPERATION,
        })?;
        let file_type = entry
            .file_type()
            .map_err(|error| path_failure(&path, "scan directory entry", &error))?;
        if file_type.is_symlink() {
            if name == ".norm" {
                return Err(ScanFailure {
                    error: ErrorDetail::new(
                        "norm/path/symlink-norm",
                        "A scanned .norm convention file must not be a symbolic link.",
                    )
                    .with_path(relative_path(root, &path)?),
                    exit_code: EXIT_OPERATION,
                });
            }
            let kind = fs::metadata(&path).map_or(ScanSymlinkKind::Other, |metadata| {
                if metadata.is_dir() {
                    ScanSymlinkKind::Directory
                } else if metadata.is_file() {
                    ScanSymlinkKind::File
                } else {
                    ScanSymlinkKind::Other
                }
            });
            if kind == ScanSymlinkKind::Directory && is_ignored_directory(&name) {
                continue;
            }
            symlinks.push(ScanSymlink::new(relative_path(root, &path)?, kind));
        } else if file_type.is_dir() {
            if !is_ignored_directory(&name) {
                child_directories.push(path);
            }
        } else if file_type.is_file() {
            file_names.push(name);
        }
    }

    observations.push(DirectoryObservation::new(
        relative_path(root, directory)?,
        file_names,
    ));
    for child in child_directories {
        scan_directory(root, &child, observations, symlinks)?;
    }
    Ok(())
}

fn relative_path(root: &Path, path: &Path) -> Result<String, ScanFailure> {
    path.strip_prefix(root)
        .map(project_path)
        .map_err(|error| ScanFailure {
            error: ErrorDetail::new(
                "norm/path/outside-root",
                format!("A scan path is not relative to its root: {error}"),
            )
            .with_path(portable_path(path)),
            exit_code: EXIT_USAGE,
        })
}

fn path_failure(path: &Path, subject: &str, error: &std::io::Error) -> ScanFailure {
    ScanFailure {
        error: ErrorDetail::new(
            "norm/path/read",
            format!("The {subject} is unavailable: {error}"),
        )
        .with_path(portable_path(path)),
        exit_code: EXIT_OPERATION,
    }
}

fn emit_human_failure(failure: ScanFailure) -> ExitCode {
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

fn emit_text(response: &ScanResponse) {
    println!("norm scan: {}", response.root);
    println!("directories scanned: {}", response.directory_count);
    println!();
    println!(
        ".norm coverage: {}/{} dirs ({:.0}%)",
        response.norm_coverage.dirs_with_norm,
        response.norm_coverage.total_dirs,
        response.norm_coverage.ratio * 100.0
    );
    println!();
    println!("directory naming:");
    for (style, count) in &response.naming.directories {
        println!("  {style}: {count}");
    }
    println!();
    println!("file naming:");
    for (style, count) in &response.naming.files {
        println!("  {style}: {count}");
    }
    println!();
    println!("recurring filenames (>= 2 dirs):");
    if response.recurring_filenames.is_empty() {
        println!("  (none)");
    } else {
        for item in &response.recurring_filenames {
            println!("  {}: {} dirs", item.name, item.dir_count);
        }
    }
    println!();
    println!("directory layout:");
    for directory in &response.directories {
        let marker = if directory.has_norm { " [.norm]" } else { "" };
        let indent = "  ".repeat(directory.depth);
        println!(
            "  {indent}{}{marker} ({} files)",
            directory.path, directory.file_count
        );
    }
}
