//! Executable contract for versioned compatibility discovery.

use std::{fs, path::PathBuf, process::Command};

use serde_json::Value;

fn expected_response() -> Value {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/compatibility/expected.json");
    let expected = fs::read_to_string(&path)
        .unwrap_or_else(|error| {
            panic!(
                "compatibility fixture {} is unavailable: {error}",
                path.display()
            )
        })
        .replace("{version}", env!("CARGO_PKG_VERSION"));
    serde_json::from_str(&expected)
        .unwrap_or_else(|error| panic!("compatibility fixture is invalid JSON: {error}"))
}

fn run_compatibility(pretty: bool) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_norm"));
    command.arg("compatibility");
    if pretty {
        command.arg("--pretty");
    }
    command
        .output()
        .unwrap_or_else(|error| panic!("failed to execute compatibility discovery: {error}"))
}

#[test]
fn compatibility_discovery_is_exact_and_machine_only() {
    for pretty in [false, true] {
        let output = run_compatibility(pretty);
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stderr.is_empty());
        let actual: Value = serde_json::from_slice(&output.stdout)
            .unwrap_or_else(|error| panic!("compatibility stdout is invalid JSON: {error}"));
        assert_eq!(actual, expected_response());
        if pretty {
            assert!(String::from_utf8_lossy(&output.stdout).contains("\n  \"product\""));
        } else {
            assert!(!String::from_utf8_lossy(&output.stdout).contains("\n  \"product\""));
        }
    }
}

#[test]
fn compiled_identity_matches_the_frozen_bundle_lock() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/contract/bundle.lock.json");
    let lock: Value =
        serde_json::from_slice(&fs::read(&path).unwrap_or_else(|error| {
            panic!("bundle lock {} is unavailable: {error}", path.display())
        }))
        .unwrap_or_else(|error| panic!("bundle lock is invalid JSON: {error}"));
    let compatibility = expected_response();
    assert_eq!(
        compatibility.pointer("/conformance/contractDigest"),
        lock.get("contractDigest")
    );
    assert_eq!(
        compatibility.pointer("/conformance/caseCount"),
        lock.get("caseCount")
    );
    assert_eq!(
        compatibility.pointer("/conformance/suite"),
        lock.get("suite")
    );
    assert_eq!(
        compatibility.pointer("/conformance/bundleApi"),
        lock.get("apiVersion")
    );
}
