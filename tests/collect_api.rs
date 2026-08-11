//! Filesystem integration tests for the high-level collect API.

mod support;

use std::path::Path;

use norm_spec::{CollectRequest, FailureClass, collect};
use support::{Fixture, write_standard_fixture};

#[test]
fn external_style_collection_preserves_inheritance_order() {
    let fixture = Fixture::new().unwrap_or_else(|error| panic!("fixture should exist: {error}"));
    write_standard_fixture(&fixture);

    let collected = collect(CollectRequest::new(&fixture.path, Path::new("docs")))
        .unwrap_or_else(|error| panic!("collection should succeed: {error}"));
    assert_eq!(collected.target, "docs");
    assert_eq!(collected.norms.len(), 2);
    assert_eq!(collected.norms[0].path, "docs/.norm");
    assert_eq!(collected.norms[1].path, ".norm");
}

#[test]
fn unavailable_target_is_a_typed_operation_failure() {
    let fixture = Fixture::new().unwrap_or_else(|error| panic!("fixture should exist: {error}"));
    let result = collect(CollectRequest::new(
        &fixture.path,
        Path::new("missing-target"),
    ));
    let Err(error) = result else {
        panic!("missing target must fail");
    };
    assert_eq!(error.class(), FailureClass::Operation);
    assert_eq!(error.detail().code, "norm/path/not-found");
    assert_eq!(error.detail().path.as_deref(), Some("missing-target"));
}
