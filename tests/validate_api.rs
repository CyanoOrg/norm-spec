//! Filesystem integration tests for the high-level validation API.

mod support;

use std::path::Path;

use norm_spec::{FailureClass, ValidateRequest, validate};
use support::{Fixture, root_norm, write_standard_fixture};

#[test]
fn packaged_schemas_validate_one_or_all_conventions() {
    let fixture = Fixture::new().unwrap_or_else(|error| panic!("fixture should exist: {error}"));
    write_standard_fixture(&fixture);

    let one = validate(ValidateRequest::path(&fixture.path, Path::new("docs")))
        .unwrap_or_else(|error| panic!("single validation should succeed: {error}"));
    assert_eq!(one.summary.files, 1);
    assert_eq!(one.summary.errors, 0);

    let all = validate(ValidateRequest::all(&fixture.path))
        .unwrap_or_else(|error| panic!("recursive validation should succeed: {error}"));
    assert_eq!(all.summary.files, 2);
    assert_eq!(all.summary.errors, 0);
}

#[test]
fn validation_errors_remain_completed_results() {
    let fixture = Fixture::new().unwrap_or_else(|error| panic!("fixture should exist: {error}"));
    fixture
        .write(
            ".norm",
            "---\nmetadata:\n  layer: root\n  scope: ./\n  version: \"1.0\"\nagent_rules:\n  reference_policy:\n    target_must_exist: true\n    allow_external: false\nstrong_references:\n  - type: source\n    target: ../outside.md\n    sync: manual\n    validation: strict\n---\n\n# Root\n",
        )
        .unwrap_or_else(|error| panic!("invalid convention should be written: {error}"));
    let response = validate(ValidateRequest::path(&fixture.path, Path::new(".norm")))
        .unwrap_or_else(|error| panic!("evaluation should complete: {error}"));
    assert_eq!(response.summary.errors, 1);
    assert_eq!(
        response.results[0].errors[0].code,
        "norm/reference/outside-root"
    );
}

#[test]
fn unavailable_schema_is_a_typed_usage_failure() {
    let fixture = Fixture::new().unwrap_or_else(|error| panic!("fixture should exist: {error}"));
    fixture
        .write(".norm", root_norm())
        .unwrap_or_else(|error| panic!("root convention should be written: {error}"));
    let result = validate(
        ValidateRequest::path(&fixture.path, Path::new(".norm"))
            .schema_dir(Path::new("missing-schema")),
    );
    let Err(error) = result else {
        panic!("missing schema must fail");
    };
    assert_eq!(error.class(), FailureClass::Usage);
    assert_eq!(error.detail().code, "norm/usage/schema-not-found");
    assert_eq!(error.detail().path.as_deref(), Some("missing-schema"));
}
