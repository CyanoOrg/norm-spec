//! Forward workflow evidence for the canonical norm-spec Skill.

use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

use serde_json::Value;

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct Project {
    path: PathBuf,
}

impl Project {
    fn new(label: &str) -> Self {
        for _ in 0..100 {
            let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "norm-spec-skill-workflow-{label}-{}-{sequence}",
                std::process::id()
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Self { path },
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => panic!("failed to create Skill workflow project: {error}"),
            }
        }
        panic!("failed to allocate a unique Skill workflow project")
    }

    fn write(&self, relative: &str, contents: &str) {
        let path = self.path.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|error| {
                panic!("Skill workflow fixture directory should be created: {error}")
            });
        }
        fs::write(path, contents)
            .unwrap_or_else(|error| panic!("Skill workflow fixture should be written: {error}"));
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        match fs::remove_dir_all(&self.path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => eprintln!("failed to remove Skill workflow project: {error}"),
        }
    }
}

fn root_norm() -> &'static str {
    "---\nmetadata: {layer: project, scope: ./, version: \"1.0\"}\nagent_rules:\n  update_order: [\"Update the project source first\"]\n---\n\n# Project\n"
}

fn docs_norm() -> &'static str {
    "---\nmetadata: {layer: documentation, scope: docs/, version: \"1.0\"}\nagent_rules:\n  update_order: [\"Update documentation after project sources\"]\n---\n\n# Documentation\n"
}

fn run_norm(current_dir: &Path, args: &[&str], expected_exit: i32) -> (Output, Value) {
    let output = Command::new(env!("CARGO_BIN_EXE_norm"))
        .current_dir(current_dir)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("Skill workflow command should execute: {error}"));
    assert_eq!(
        output.status.code(),
        Some(expected_exit),
        "command {args:?} returned unexpected exit; stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty(), "machine command wrote to stderr");
    let response = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("Skill workflow command returned invalid JSON: {error}"));
    (output, response)
}

fn assert_compatible(project: &Path) {
    let (_, response) = run_norm(project, &["compatibility"], 0);
    assert_eq!(
        response.get("apiVersion").and_then(Value::as_str),
        Some("norm-spec/compatibility/v1")
    );
    assert_eq!(
        response.pointer("/formats/0").and_then(Value::as_str),
        Some("norm-spec/a1")
    );
}

fn skill_text() -> String {
    fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills/norm-spec/SKILL.md"),
    )
    .unwrap_or_else(|error| panic!("canonical Skill should be readable: {error}"))
}

#[test]
fn nested_collection_is_most_specific_first_without_discarding_outer_rules() {
    let project = Project::new("collect");
    project.write(".norm", root_norm());
    project.write("docs/.norm", docs_norm());
    project.write("docs/guide.md", "# Guide\n");
    assert_compatible(&project.path);

    let root = project.path.to_string_lossy();
    let target = project.path.join("docs");
    let target = target.to_string_lossy();
    let (_, response) = run_norm(
        &project.path,
        &["collect", "--root", &root, "--target", &target, "--pretty"],
        0,
    );
    assert_eq!(
        response.get("apiVersion").and_then(Value::as_str),
        Some("norm-spec/collect/v1")
    );
    let paths = response
        .get("norms")
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("collect response should contain norms"))
        .iter()
        .map(|norm| {
            norm.get("path")
                .and_then(Value::as_str)
                .unwrap_or_else(|| panic!("collected convention should contain a path"))
        })
        .collect::<Vec<_>>();
    assert_eq!(paths, ["docs/.norm", ".norm"]);
}

#[test]
fn authoring_and_invalid_reference_repair_use_the_engine_end_to_end() {
    let project = Project::new("author-repair");
    project.write(".norm", root_norm());
    project.write("docs/conventions.md", "# Conventions\n");
    project.write("docs/status.md", "# Status\n");
    assert_compatible(&project.path);

    let root = project.path.to_string_lossy();
    let norm_file = project.path.join("docs/.norm");
    let norm_file_arg = norm_file.to_string_lossy();
    let (_, initialized) = run_norm(
        &project.path,
        &[
            "init",
            "--profile",
            "convention",
            "--output",
            &norm_file_arg,
            "--json",
            "--pretty",
        ],
        0,
    );
    assert_eq!(
        initialized.get("apiVersion").and_then(Value::as_str),
        Some("norm-spec/init/v1")
    );

    let (_, parsed) = run_norm(&project.path, &["parse", &norm_file_arg, "--pretty"], 0);
    assert_eq!(
        parsed.get("apiVersion").and_then(Value::as_str),
        Some("norm-spec/parse/v1")
    );
    let (_, valid) = run_norm(
        &project.path,
        &[
            "validate",
            &norm_file_arg,
            "--root",
            &root,
            "--strict",
            "--json",
            "--pretty",
        ],
        0,
    );
    assert_eq!(
        valid.pointer("/summary/errors").and_then(Value::as_u64),
        Some(0)
    );
    assert_eq!(
        valid.pointer("/summary/warnings").and_then(Value::as_u64),
        Some(0)
    );

    let invalid_fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/adoption/invalid-reference.norm.invalid");
    fs::copy(&invalid_fixture, &norm_file)
        .unwrap_or_else(|error| panic!("invalid reference fixture should be installed: {error}"));
    let (_, invalid) = run_norm(
        &project.path,
        &[
            "validate", "--all", "--root", &root, "--strict", "--json", "--pretty",
        ],
        1,
    );
    assert_eq!(
        invalid.pointer("/results/1/path").and_then(Value::as_str),
        Some("docs/.norm")
    );
    assert_eq!(
        invalid
            .pointer("/results/1/errors/0/code")
            .and_then(Value::as_str),
        Some("norm/reference/not-found")
    );

    project.write("docs/missing.md", "# Repaired reference\n");
    let (_, repaired) = run_norm(
        &project.path,
        &[
            "validate", "--all", "--root", &root, "--strict", "--json", "--pretty",
        ],
        0,
    );
    assert_eq!(
        repaired.pointer("/summary/errors").and_then(Value::as_u64),
        Some(0)
    );
    assert_eq!(
        repaired
            .pointer("/summary/warnings")
            .and_then(Value::as_u64),
        Some(0)
    );
}

#[test]
fn missing_cli_is_an_explicit_stop_without_generated_fallback_state() {
    let project = Project::new("missing-cli");
    let missing_candidate = project
        .path
        .join(if cfg!(windows) { "norm.exe" } else { "norm" });
    let result = Command::new(&missing_candidate)
        .arg("compatibility")
        .current_dir(&project.path)
        .output();
    let Err(error) = result else {
        panic!("missing candidate must not execute successfully");
    };
    assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    assert!(
        fs::read_dir(&project.path)
            .unwrap_or_else(|read_error| panic!(
                "missing-CLI project should be readable: {read_error}"
            ))
            .next()
            .is_none(),
        "missing engine must not generate a fallback convention or ruleset"
    );

    let normalized = skill_text()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(normalized.contains("If the executable is missing"));
    assert!(normalized.contains("stop the affected workflow"));
    assert!(normalized.contains("Do not continue by interpreting `.norm` yourself"));
    assert!(
        normalized
            .contains("compatibility or executable failure: repair the installation and stop")
    );
}
