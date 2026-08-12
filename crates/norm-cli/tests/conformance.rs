//! Black-box fail-closed coverage for arbitrary-candidate conformance.

use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

use serde_json::{Value, json};

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct TemporaryRoot {
    path: PathBuf,
}

impl TemporaryRoot {
    fn new(label: &str) -> Self {
        for _ in 0..100 {
            let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "norm-spec-conformance-test-{label}-{}-{sequence}",
                std::process::id()
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Self { path },
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => panic!("failed to create temporary root: {error}"),
            }
        }
        panic!("failed to allocate a temporary root")
    }
}

impl Drop for TemporaryRoot {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path)
            .unwrap_or_else(|error| panic!("failed to remove temporary root: {error}"));
    }
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn export_bundle(root: &Path) -> PathBuf {
    let repository = repository_root();
    let contract = repository.join("tests/contract");
    let destination = root.join("bundle");
    fs::create_dir(&destination)
        .unwrap_or_else(|error| panic!("failed to create bundle root: {error}"));
    let lock_bytes = fs::read(contract.join("bundle.lock.json"))
        .unwrap_or_else(|error| panic!("failed to read bundle lock: {error}"));
    let lock: Value = serde_json::from_slice(&lock_bytes)
        .unwrap_or_else(|error| panic!("bundle lock is invalid JSON: {error}"));
    let files = lock["files"]
        .as_array()
        .unwrap_or_else(|| panic!("bundle lock omitted files"));
    for file in files {
        let relative = file["path"]
            .as_str()
            .unwrap_or_else(|| panic!("bundle file omitted its path"));
        let source = if relative.starts_with("schema/") {
            repository.join(relative)
        } else {
            contract.join(relative)
        };
        let target = join_portable(&destination, relative);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .unwrap_or_else(|error| panic!("failed to create bundle directory: {error}"));
        }
        fs::copy(&source, &target).unwrap_or_else(|error| {
            panic!(
                "failed to copy bundle file {} to {}: {error}",
                source.display(),
                target.display()
            )
        });
    }
    fs::write(destination.join("bundle.lock.json"), lock_bytes)
        .unwrap_or_else(|error| panic!("failed to write bundle lock: {error}"));
    destination
}

fn join_portable(root: &Path, value: &str) -> PathBuf {
    value
        .split('/')
        .fold(root.to_path_buf(), |path, component| path.join(component))
}

fn run(candidate: &Path, bundle: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_norm-spec-conformance"))
        .arg("--candidate")
        .arg(candidate)
        .arg("--contract-dir")
        .arg(bundle)
        .output()
        .unwrap_or_else(|error| panic!("failed to execute conformance runner: {error}"))
}

fn report(output: &Output) -> Value {
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("conformance stdout is invalid JSON: {error}"))
}

fn assert_summary(value: &Value, executed: u64, passed: u64, failed: u64, not_executed: u64) {
    assert_eq!(value["summary"]["declared"], 82);
    assert_eq!(value["summary"]["executed"], executed);
    assert_eq!(value["summary"]["passed"], passed);
    assert_eq!(value["summary"]["failed"], failed);
    assert_eq!(value["summary"]["notExecuted"], not_executed);
    assert_eq!(executed, passed + failed);
    assert_eq!(82, executed + not_executed);
}

#[test]
fn current_candidate_executes_the_exact_complete_suite() {
    let root = TemporaryRoot::new("pass");
    let bundle = export_bundle(&root.path);
    let output = run(Path::new(env!("CARGO_BIN_EXE_norm")), &bundle);
    assert_eq!(output.status.code(), Some(0));
    let value = report(&output);
    assert_eq!(value["apiVersion"], "norm-spec/conformance/v1");
    assert_eq!(value["status"], "pass");
    assert_eq!(value["complete"], true);
    assert_summary(&value, 82, 82, 0, 0);
    assert_eq!(value["issues"], json!([]));
    assert_eq!(value["failures"], json!([]));
}

#[test]
fn missing_candidate_is_an_incomplete_machine_error() {
    let root = TemporaryRoot::new("missing-candidate");
    let bundle = export_bundle(&root.path);
    let output = run(&root.path.join("missing-norm"), &bundle);
    assert_eq!(output.status.code(), Some(2));
    let value = report(&output);
    assert_eq!(value["status"], "error");
    assert_eq!(value["complete"], false);
    assert_summary(&value, 0, 0, 0, 82);
    assert_eq!(
        value["issues"][0]["code"],
        "norm/conformance/candidate-unavailable"
    );
}

#[test]
fn tampered_bundle_is_rejected_before_case_execution() {
    let root = TemporaryRoot::new("tampered-bundle");
    let bundle = export_bundle(&root.path);
    fs::write(bundle.join("expected/parse-minimal.json"), "{}\n")
        .unwrap_or_else(|error| panic!("failed to tamper with bundle: {error}"));
    let output = run(Path::new(env!("CARGO_BIN_EXE_norm")), &bundle);
    assert_eq!(output.status.code(), Some(2));
    let value = report(&output);
    assert_eq!(value["complete"], false);
    assert_summary(&value, 0, 0, 0, 82);
    assert_eq!(
        value["issues"][0]["code"],
        "norm/conformance/bundle-mismatch"
    );
}

#[test]
fn unsafe_locked_path_is_rejected_explicitly() {
    let root = TemporaryRoot::new("unsafe-bundle");
    let bundle = export_bundle(&root.path);
    let lock_path = bundle.join("bundle.lock.json");
    let mut lock: Value = serde_json::from_slice(
        &fs::read(&lock_path).unwrap_or_else(|error| panic!("failed to read lock: {error}")),
    )
    .unwrap_or_else(|error| panic!("lock is invalid JSON: {error}"));
    lock["files"][0]["path"] = json!("../escape");
    fs::write(
        &lock_path,
        serde_json::to_vec_pretty(&lock)
            .unwrap_or_else(|error| panic!("failed to serialize changed lock: {error}")),
    )
    .unwrap_or_else(|error| panic!("failed to write changed lock: {error}"));
    let output = run(Path::new(env!("CARGO_BIN_EXE_norm")), &bundle);
    assert_eq!(output.status.code(), Some(2));
    let value = report(&output);
    assert_eq!(
        value["issues"][0]["code"],
        "norm/conformance/bundle-unsafe-path"
    );
}

#[test]
fn compatible_but_wrong_candidate_runs_every_case_and_fails_complete() {
    let root = TemporaryRoot::new("wrong-candidate");
    let bundle = export_bundle(&root.path);
    let candidate = compile_fake_candidate(&root.path, true);
    let output = run(&candidate, &bundle);
    assert_eq!(output.status.code(), Some(1));
    let value = report(&output);
    assert_eq!(value["candidate"]["compatibility"], "compatible");
    assert_eq!(value["status"], "fail");
    assert_eq!(value["complete"], true);
    assert_eq!(value["summary"]["executed"], 82);
    assert_eq!(value["summary"]["notExecuted"], 0);
    assert!(
        value["summary"]["failed"]
            .as_u64()
            .is_some_and(|count| count > 0)
    );
}

#[test]
fn missing_compatibility_does_not_hide_complete_behavioral_evidence() {
    let root = TemporaryRoot::new("missing-compatibility");
    let bundle = export_bundle(&root.path);
    let candidate = compile_fake_candidate(&root.path, false);
    let output = run(&candidate, &bundle);
    assert_eq!(output.status.code(), Some(1));
    let value = report(&output);
    assert_eq!(value["candidate"]["compatibility"], "unavailable");
    assert_eq!(value["status"], "fail");
    assert_eq!(value["complete"], true);
    assert_eq!(value["summary"]["executed"], 82);
    assert_eq!(value["summary"]["notExecuted"], 0);
    assert_eq!(
        value["issues"][0]["code"],
        "norm/conformance/compatibility-unavailable"
    );
}

fn compile_fake_candidate(root: &Path, compatible: bool) -> PathBuf {
    let source = root.join("fake-candidate.rs");
    let executable = root.join(if cfg!(windows) {
        "fake-candidate.exe"
    } else {
        "fake-candidate"
    });
    let compatibility =
        fs::read_to_string(repository_root().join("tests/compatibility/expected.json"))
            .unwrap_or_else(|error| panic!("failed to read compatibility fixture: {error}"))
            .replace("{version}", "9.9.9-test");
    let compact: Value = serde_json::from_str(&compatibility)
        .unwrap_or_else(|error| panic!("compatibility fixture is invalid: {error}"));
    let compact = serde_json::to_string(&compact)
        .unwrap_or_else(|error| panic!("failed to compact compatibility fixture: {error}"));
    let body = if compatible {
        format!(
            "fn main() {{ if std::env::args().nth(1).as_deref() == Some(\"compatibility\") {{ println!(\"{{}}\", {compact:?}); }} }}\n"
        )
    } else {
        "fn main() { if std::env::args().nth(1).as_deref() == Some(\"compatibility\") { std::process::exit(1); } }\n".to_owned()
    };
    fs::write(&source, body)
        .unwrap_or_else(|error| panic!("failed to write fake candidate source: {error}"));
    let output = Command::new("rustc")
        .arg(&source)
        .arg("-o")
        .arg(&executable)
        .output()
        .unwrap_or_else(|error| panic!("failed to execute rustc: {error}"));
    assert!(
        output.status.success(),
        "fake candidate compilation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    executable
}
