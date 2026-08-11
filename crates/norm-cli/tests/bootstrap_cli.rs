//! Focused black-box tests for the first executable CLI slice.

use std::{
    path::PathBuf,
    process::{Command, Output},
};

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

#[test]
fn version_is_available() {
    let output = run_norm(&["--version"]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "norm 0.1.0-alpha.1\n"
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

#[test]
fn remaining_commands_fail_explicitly() {
    for command in ["validate", "init", "scan"] {
        let output = run_norm(&[command]);
        assert_eq!(output.status.code(), Some(2));
        assert!(String::from_utf8_lossy(&output.stderr).contains("not implemented yet"));
    }
}
