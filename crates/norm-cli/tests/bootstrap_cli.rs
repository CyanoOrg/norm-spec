//! Focused black-box tests for the first executable CLI slice.

use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn run_norm(args: &[&str]) -> Output {
    match Command::new(env!("CARGO_BIN_EXE_norm")).args(args).output() {
        Ok(output) => output,
        Err(error) => panic!("failed to execute norm: {error}"),
    }
}

fn contract_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/contract")
        .join(path)
}

fn temporary_root(label: &str) -> PathBuf {
    for _ in 0..100 {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "norm-spec-bootstrap-{label}-{}-{sequence}",
            std::process::id()
        ));
        match fs::create_dir(&path) {
            Ok(()) => return path,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => panic!("failed to create temporary root: {error}"),
        }
    }
    panic!("failed to allocate a unique temporary root")
}

#[test]
fn version_is_available() {
    let output = run_norm(&["--version"]);
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "norm 0.1.0-rc.1\n");
}

#[test]
fn package_default_run_preserves_the_norm_development_command() {
    let manifest = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .unwrap_or_else(|error| panic!("CLI manifest should be readable: {error}"));
    assert!(
        manifest.contains("default-run = \"norm\""),
        "adding another binary must not make cargo run ambiguous"
    );
}

#[test]
fn help_lists_the_frozen_command_surface() {
    let output = run_norm(&["--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    for command in ["parse", "collect", "validate", "init", "scan"] {
        assert!(stdout.contains(command), "help omitted {command}");
    }
}

#[test]
fn parse_emits_the_versioned_response() {
    let fixture = contract_fixture("fixtures/valid/minimal.norm");
    let output = match Command::new(env!("CARGO_BIN_EXE_norm"))
        .arg("parse")
        .arg(fixture)
        .output()
    {
        Ok(output) => output,
        Err(error) => panic!("failed to execute norm: {error}"),
    };
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert!(String::from_utf8_lossy(&output.stdout).contains("norm-spec/parse/v1"));
}

#[test]
fn collect_emits_the_versioned_response() {
    let root = contract_fixture("fixtures/valid");
    let output = match Command::new(env!("CARGO_BIN_EXE_norm"))
        .arg("collect")
        .arg("--root")
        .arg(&root)
        .arg("--target")
        .arg(".")
        .output()
    {
        Ok(output) => output,
        Err(error) => panic!("failed to execute norm: {error}"),
    };
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert!(String::from_utf8_lossy(&output.stdout).contains("norm-spec/collect/v1"));
}

#[test]
fn validate_emits_the_versioned_response() {
    let root = contract_fixture("fixtures/valid");
    let fixture = root.join("minimal.norm");
    let output = match Command::new(env!("CARGO_BIN_EXE_norm"))
        .arg("validate")
        .arg(&fixture)
        .arg("--root")
        .arg(&root)
        .arg("--json")
        .output()
    {
        Ok(output) => output,
        Err(error) => panic!("failed to execute norm: {error}"),
    };
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert!(String::from_utf8_lossy(&output.stdout).contains("norm-spec/validate/v1"));
}

#[test]
fn init_writes_an_embedded_template_and_versioned_response() {
    let root = temporary_root("init");
    let output = match Command::new(env!("CARGO_BIN_EXE_norm"))
        .current_dir(&root)
        .args(["init", "--profile", "module", "--json"])
        .output()
    {
        Ok(output) => output,
        Err(error) => panic!("failed to execute norm: {error}"),
    };
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert!(String::from_utf8_lossy(&output.stdout).contains("norm-spec/init/v1"));
    let written = fs::read_to_string(root.join(".norm"))
        .unwrap_or_else(|error| panic!("init output should be readable: {error}"));
    let expected = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../templates/profiles/module.norm"),
    )
    .unwrap_or_else(|error| panic!("module template should be readable: {error}"));
    assert_eq!(written, expected);
    fs::remove_dir_all(&root)
        .unwrap_or_else(|error| panic!("temporary init root should be removable: {error}"));
}

#[test]
fn scan_emits_a_versioned_structural_response() {
    let root = temporary_root("scan");
    fs::create_dir(root.join("src"))
        .unwrap_or_else(|error| panic!("scan source directory should be created: {error}"));
    fs::write(root.join("README.md"), "readme")
        .unwrap_or_else(|error| panic!("scan root fixture should be written: {error}"));
    fs::write(root.join("src/lib.rs"), "library")
        .unwrap_or_else(|error| panic!("scan source fixture should be written: {error}"));
    let output = match Command::new(env!("CARGO_BIN_EXE_norm"))
        .arg("scan")
        .arg("--root")
        .arg(&root)
        .output()
    {
        Ok(output) => output,
        Err(error) => panic!("failed to execute norm: {error}"),
    };
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("norm-spec/scan/v1"));
    assert!(stdout.contains("\"directory_count\":2"));
    fs::remove_dir_all(&root)
        .unwrap_or_else(|error| panic!("temporary scan root should be removable: {error}"));
}

#[test]
fn missing_collect_target_is_a_machine_usage_error() {
    let output = run_norm(&["collect"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stderr.is_empty());
    assert!(String::from_utf8_lossy(&output.stdout).contains("norm/usage/missing-argument"));
}

#[test]
fn missing_parse_path_is_a_machine_usage_error() {
    let output = run_norm(&["parse"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stderr.is_empty());
    assert!(String::from_utf8_lossy(&output.stdout).contains("norm/usage/missing-argument"));
}

#[test]
fn missing_absolute_path_is_reported_relative_to_the_working_root() {
    let root = contract_fixture("");
    let missing = root.join("missing");
    let output = match Command::new(env!("CARGO_BIN_EXE_norm"))
        .current_dir(&root)
        .arg("parse")
        .arg(&missing)
        .output()
    {
        Ok(output) => output,
        Err(error) => panic!("failed to execute norm: {error}"),
    };
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"path\":\"missing\""));
    assert!(!stdout.contains(&root.to_string_lossy().into_owned()));
}
