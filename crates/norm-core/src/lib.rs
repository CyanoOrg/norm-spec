//! Canonical semantic engine for the `.norm` format.

#![forbid(unsafe_code)]

mod collector;
mod parser;
mod protocol;
mod scan;
mod schema;
mod semantics;
#[cfg(test)]
mod test_support;
mod validator;

pub use collector::{CollectPathError, collect_candidate_paths, project_path};
pub use parser::{ParseError, ParseOptions, ParsedNorm, parse_norm};
pub use protocol::{
    CollectResponse, CollectedNorm, CompatibilityConformance, CompatibilityProduct,
    CompatibilityResponse, CompatibilityRustApi, Diagnostic, ErrorDetail, ErrorResponse,
    InitAction, InitResponse, ParseResponse, ValidateResponse, ValidationResult, ValidationStatus,
    ValidationSummary,
};
pub use scan::{
    DirectoryObservation, NormCoverage, RecurringFilename, ScanNaming, ScanResponse, ScanSymlink,
    ScanSymlinkAction, ScanSymlinkKind, ScannedDirectory, classify_name, is_ignored_directory,
};
pub use schema::{SchemaCompileError, SchemaViolation, validate_draft7};
pub use semantics::{ReferenceResolver, ReferenceStatus, validate_frontmatter};
pub use validator::{SchemaBundle, ValidationDiagnostics, ValidationOptions, validate_structure};

/// Identifier for the initial `.norm` format contract.
pub const FORMAT_ID: &str = "norm-spec/a1";

/// Machine API identifier for parse responses.
pub const PARSE_API_VERSION: &str = "norm-spec/parse/v1";

/// Machine API identifier for collect responses.
pub const COLLECT_API_VERSION: &str = "norm-spec/collect/v1";

/// Machine API identifier for validate responses.
pub const VALIDATE_API_VERSION: &str = "norm-spec/validate/v1";

/// Machine API identifier for init responses.
pub const INIT_API_VERSION: &str = "norm-spec/init/v1";

/// Machine API identifier for scan responses.
pub const SCAN_API_VERSION: &str = "norm-spec/scan/v1";

/// Machine API identifier for handled CLI failures in machine mode.
pub const ERROR_API_VERSION: &str = "norm-spec/error/v1";

/// Machine API identifier for compatibility discovery responses.
pub const COMPATIBILITY_API_VERSION: &str = "norm-spec/compatibility/v1";

/// Identifier for the public high-level Rust consumer surface.
pub const RUST_API_VERSION: &str = "norm-spec/rust-api/v1";

/// Identifier for the locked contract-bundle format.
pub const CONTRACT_BUNDLE_API_VERSION: &str = "norm-spec/contract-bundle/v1";

/// Machine API identifier for arbitrary-candidate conformance reports.
pub const CONFORMANCE_API_VERSION: &str = "norm-spec/conformance/v1";

/// Identifier for the frozen initial A1 CLI conformance suite.
pub const A1_CLI_SUITE_ID: &str = "norm-spec/a1-cli/v1";

/// Number of executable cases in the frozen initial A1 CLI suite.
pub const A1_CLI_CASE_COUNT: usize = 82;

/// SHA-256 identity of the exact frozen initial A1 CLI contract bundle.
pub const A1_CLI_CONTRACT_DIGEST: &str =
    "sha256:3d94441e9cde3ef9489618bdb8fbf37f6979331bea099acadf0136b65df7e2eb";

/// Return the compiled crate version.
#[must_use]
pub const fn crate_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::{
        A1_CLI_CASE_COUNT, A1_CLI_CONTRACT_DIGEST, A1_CLI_SUITE_ID, COLLECT_API_VERSION,
        COMPATIBILITY_API_VERSION, CONFORMANCE_API_VERSION, CONTRACT_BUNDLE_API_VERSION,
        ERROR_API_VERSION, FORMAT_ID, INIT_API_VERSION, PARSE_API_VERSION, RUST_API_VERSION,
        SCAN_API_VERSION, VALIDATE_API_VERSION,
    };

    #[test]
    fn protocol_identifiers_are_explicitly_versioned() {
        assert_eq!(PARSE_API_VERSION, "norm-spec/parse/v1");
        assert_eq!(COLLECT_API_VERSION, "norm-spec/collect/v1");
        assert_eq!(VALIDATE_API_VERSION, "norm-spec/validate/v1");
        assert_eq!(INIT_API_VERSION, "norm-spec/init/v1");
        assert_eq!(SCAN_API_VERSION, "norm-spec/scan/v1");
        assert_eq!(ERROR_API_VERSION, "norm-spec/error/v1");
        assert_eq!(COMPATIBILITY_API_VERSION, "norm-spec/compatibility/v1");
        assert_eq!(RUST_API_VERSION, "norm-spec/rust-api/v1");
        assert_eq!(CONTRACT_BUNDLE_API_VERSION, "norm-spec/contract-bundle/v1");
        assert_eq!(CONFORMANCE_API_VERSION, "norm-spec/conformance/v1");
        assert_eq!(A1_CLI_SUITE_ID, "norm-spec/a1-cli/v1");
        assert_eq!(A1_CLI_CASE_COUNT, 82);
        assert_eq!(
            A1_CLI_CONTRACT_DIGEST,
            "sha256:3d94441e9cde3ef9489618bdb8fbf37f6979331bea099acadf0136b65df7e2eb"
        );
        assert_eq!(FORMAT_ID, "norm-spec/a1");
    }
}
