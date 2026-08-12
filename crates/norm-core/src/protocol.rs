//! Versioned machine response models.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    A1_CLI_CASE_COUNT, A1_CLI_SUITE_ID, COLLECT_API_VERSION, COMPATIBILITY_API_VERSION,
    CONFORMANCE_API_VERSION, CONTRACT_BUNDLE_API_VERSION, ERROR_API_VERSION, FORMAT_ID,
    INIT_API_VERSION, PARSE_API_VERSION, ParsedNorm, RUST_API_VERSION, SCAN_API_VERSION,
    VALIDATE_API_VERSION, crate_version,
};

/// Exact product identity reported by compatibility discovery.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompatibilityProduct {
    /// Stable product name.
    pub name: String,
    /// Exact compiled product version.
    pub version: String,
}

/// Public Rust consumer-surface identity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompatibilityRustApi {
    /// Versioned Rust API identifier.
    pub id: String,
    /// Cargo package that provides the surface.
    pub package: String,
    /// Exact compiled package version.
    pub version: String,
}

/// Frozen conformance surface reported by a candidate.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompatibilityConformance {
    /// Versioned contract-bundle format.
    #[serde(rename = "bundleApi")]
    pub bundle_api: String,
    /// Versioned conformance-report protocol.
    #[serde(rename = "reportApi")]
    pub report_api: String,
    /// Frozen suite identifier.
    pub suite: String,
    /// Number of executable cases in the suite.
    #[serde(rename = "caseCount")]
    pub case_count: usize,
    /// SHA-256 identity of the exact contract bundle.
    #[serde(rename = "contractDigest")]
    pub contract_digest: String,
}

/// Machine-readable compatibility discovery response.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CompatibilityResponse {
    /// Version of the compatibility response protocol.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Exact product identity.
    pub product: CompatibilityProduct,
    /// Accepted format identifiers in lexical order.
    pub formats: Vec<String>,
    /// Public high-level Rust API identity.
    #[serde(rename = "rustApi")]
    pub rust_api: CompatibilityRustApi,
    /// Supported machine API identifiers in lexical order.
    #[serde(rename = "machineApis")]
    pub machine_apis: Vec<String>,
    /// Frozen conformance surface.
    pub conformance: CompatibilityConformance,
}

impl CompatibilityResponse {
    /// Construct discovery for the compiled product and an exact contract digest.
    #[must_use]
    pub fn current(contract_digest: impl Into<String>) -> Self {
        Self {
            api_version: COMPATIBILITY_API_VERSION.to_owned(),
            product: CompatibilityProduct {
                name: "norm-spec".to_owned(),
                version: crate_version().to_owned(),
            },
            formats: vec![FORMAT_ID.to_owned()],
            rust_api: CompatibilityRustApi {
                id: RUST_API_VERSION.to_owned(),
                package: "norm-spec".to_owned(),
                version: crate_version().to_owned(),
            },
            machine_apis: [
                COLLECT_API_VERSION,
                COMPATIBILITY_API_VERSION,
                ERROR_API_VERSION,
                INIT_API_VERSION,
                PARSE_API_VERSION,
                SCAN_API_VERSION,
                VALIDATE_API_VERSION,
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            conformance: CompatibilityConformance {
                bundle_api: CONTRACT_BUNDLE_API_VERSION.to_owned(),
                report_api: CONFORMANCE_API_VERSION.to_owned(),
                suite: A1_CLI_SUITE_ID.to_owned(),
                case_count: A1_CLI_CASE_COUNT,
                contract_digest: contract_digest.into(),
            },
        }
    }
}

/// Successful `parse` machine response.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ParseResponse {
    /// Version of the parse response protocol.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Parsed YAML frontmatter.
    pub frontmatter: Value,
    /// Trimmed Markdown body.
    pub body: String,
}

impl From<ParsedNorm> for ParseResponse {
    fn from(parsed: ParsedNorm) -> Self {
        Self {
            api_version: PARSE_API_VERSION.to_owned(),
            frontmatter: parsed.frontmatter,
            body: parsed.body,
        }
    }
}

/// One parsed convention file in a `collect` response.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CollectedNorm {
    /// Convention-file path relative to the collection root.
    pub path: String,
    /// Parsed YAML frontmatter.
    pub frontmatter: Value,
    /// Trimmed Markdown body.
    pub body: String,
}

impl CollectedNorm {
    /// Construct a collected convention from its portable path and parsed data.
    #[must_use]
    pub fn new(path: impl Into<String>, parsed: ParsedNorm) -> Self {
        Self {
            path: path.into(),
            frontmatter: parsed.frontmatter,
            body: parsed.body,
        }
    }
}

/// Successful `collect` machine response.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CollectResponse {
    /// Version of the collect response protocol.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Root marker for all paths in the response.
    pub root: String,
    /// Requested target path relative to the root.
    pub target: String,
    /// Parsed conventions ordered from most-specific to least-specific.
    pub norms: Vec<CollectedNorm>,
}

impl CollectResponse {
    /// Construct a versioned collection response.
    #[must_use]
    pub fn new(target: impl Into<String>, norms: Vec<CollectedNorm>) -> Self {
        Self {
            api_version: COLLECT_API_VERSION.to_owned(),
            root: ".".to_owned(),
            target: target.into(),
            norms,
        }
    }
}

/// Stable diagnostic emitted by schema, profile, or semantic validation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Diagnostic {
    /// Stable machine-readable diagnostic code.
    pub code: String,
    /// Human-readable detail; not intended for machine matching.
    pub message: String,
    /// Dot-and-index path to the affected frontmatter field, when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Machine-actionable correction hint, when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

impl Diagnostic {
    /// Construct a diagnostic with no field or suggestion.
    #[must_use]
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            field: None,
            suggestion: None,
        }
    }

    /// Attach an affected field path.
    #[must_use]
    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    /// Attach a machine-actionable correction hint.
    #[must_use]
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

/// Overall classification of one validated `.norm` file.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ValidationStatus {
    /// No errors or warnings were emitted.
    Ok,
    /// Warnings were emitted without errors.
    Warning,
    /// At least one error was emitted.
    Error,
}

/// Validation outcome for one root-relative `.norm` path.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValidationResult {
    /// Root-relative convention-file path.
    pub path: String,
    /// Highest diagnostic severity for this file.
    pub status: ValidationStatus,
    /// Deterministically ordered hard validation failures.
    pub errors: Vec<Diagnostic>,
    /// Deterministically ordered non-fatal validation findings.
    pub warnings: Vec<Diagnostic>,
}

impl ValidationResult {
    /// Construct and deterministically order a per-file validation result.
    #[must_use]
    pub fn new(
        path: impl Into<String>,
        mut errors: Vec<Diagnostic>,
        mut warnings: Vec<Diagnostic>,
    ) -> Self {
        errors.sort_by(diagnostic_order);
        warnings.sort_by(diagnostic_order);
        let status = if errors.is_empty() {
            if warnings.is_empty() {
                ValidationStatus::Ok
            } else {
                ValidationStatus::Warning
            }
        } else {
            ValidationStatus::Error
        };
        Self {
            path: path.into(),
            status,
            errors,
            warnings,
        }
    }
}

fn diagnostic_order(left: &Diagnostic, right: &Diagnostic) -> std::cmp::Ordering {
    left.field
        .cmp(&right.field)
        .then_with(|| left.code.cmp(&right.code))
        .then_with(|| left.suggestion.cmp(&right.suggestion))
        .then_with(|| left.message.cmp(&right.message))
}

/// Aggregate counts for a validation response.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValidationSummary {
    /// Number of convention files evaluated.
    pub files: usize,
    /// Total number of hard validation diagnostics.
    pub errors: usize,
    /// Total number of warning diagnostics.
    pub warnings: usize,
}

/// Versioned machine response for a completed `validate` evaluation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValidateResponse {
    /// Version of the validation response protocol.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Root marker for all paths in the response.
    pub root: String,
    /// Deterministically ordered per-file results.
    pub results: Vec<ValidationResult>,
    /// Aggregate file and diagnostic counts.
    pub summary: ValidationSummary,
}

impl ValidateResponse {
    /// Construct a validation response ordered by portable path.
    #[must_use]
    pub fn new(mut results: Vec<ValidationResult>) -> Self {
        results.sort_by(|left, right| left.path.cmp(&right.path));
        let summary = ValidationSummary {
            files: results.len(),
            errors: results.iter().map(|result| result.errors.len()).sum(),
            warnings: results.iter().map(|result| result.warnings.len()).sum(),
        };
        Self {
            api_version: VALIDATE_API_VERSION.to_owned(),
            root: ".".to_owned(),
            results,
            summary,
        }
    }
}

/// Filesystem action completed by `norm init`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InitAction {
    /// A new convention file was created.
    Created,
    /// An existing convention file was replaced under explicit force.
    Overwritten,
}

/// Versioned machine response for a completed `init` operation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InitResponse {
    /// Version of the init response protocol.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Name of the embedded profile template that was written.
    pub profile: String,
    /// Output path relative to the current working directory when contained.
    pub path: String,
    /// Whether the output was created or explicitly overwritten.
    pub action: InitAction,
}

impl InitResponse {
    /// Construct a versioned init response.
    #[must_use]
    pub fn new(profile: impl Into<String>, path: impl Into<String>, action: InitAction) -> Self {
        Self {
            api_version: INIT_API_VERSION.to_owned(),
            profile: profile.into(),
            path: path.into(),
            action,
        }
    }
}

/// Versioned machine response for a handled command failure.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ErrorResponse {
    /// Version of the common error protocol.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Command that handled the failure.
    pub command: String,
    /// Stable error detail.
    pub error: ErrorDetail,
}

impl ErrorResponse {
    /// Construct a handled error response for a command.
    #[must_use]
    pub fn new(command: impl Into<String>, error: ErrorDetail) -> Self {
        Self {
            api_version: ERROR_API_VERSION.to_owned(),
            command: command.into(),
            error,
        }
    }
}

/// Stable machine-readable error detail.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ErrorDetail {
    /// Stable error code.
    pub code: String,
    /// Human-readable diagnostic; not intended for machine matching.
    pub message: String,
    /// Portable path associated with the error, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// CLI field associated with the error, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

impl ErrorDetail {
    /// Construct an error with no path or field context.
    #[must_use]
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            path: None,
            field: None,
        }
    }

    /// Attach a portable path to the error.
    #[must_use]
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Attach a CLI field name to the error.
    #[must_use]
    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde_json::json;

    use super::{
        CollectResponse, CollectedNorm, CompatibilityResponse, Diagnostic, ErrorDetail,
        ErrorResponse, InitAction, InitResponse, ParseResponse, ValidateResponse, ValidationResult,
        ValidationStatus,
    };
    use crate::ParsedNorm;

    fn json_value(value: impl Serialize) -> serde_json::Value {
        match serde_json::to_value(value) {
            Ok(value) => value,
            Err(error) => panic!("failed to serialize test response: {error}"),
        }
    }

    #[test]
    fn compatibility_response_exposes_exact_sorted_identifiers() {
        let response = CompatibilityResponse::current("sha256:example");
        assert_eq!(
            json_value(response),
            json!({
                "apiVersion": "norm-spec/compatibility/v1",
                "product": {
                    "name": "norm-spec",
                    "version": env!("CARGO_PKG_VERSION")
                },
                "formats": ["norm-spec/a1"],
                "rustApi": {
                    "id": "norm-spec/rust-api/v1",
                    "package": "norm-spec",
                    "version": env!("CARGO_PKG_VERSION")
                },
                "machineApis": [
                    "norm-spec/collect/v1",
                    "norm-spec/compatibility/v1",
                    "norm-spec/error/v1",
                    "norm-spec/init/v1",
                    "norm-spec/parse/v1",
                    "norm-spec/scan/v1",
                    "norm-spec/validate/v1"
                ],
                "conformance": {
                    "bundleApi": "norm-spec/contract-bundle/v1",
                    "reportApi": "norm-spec/conformance/v1",
                    "suite": "norm-spec/a1-cli/v1",
                    "caseCount": 82,
                    "contractDigest": "sha256:example"
                }
            })
        );
    }

    #[test]
    fn parse_response_uses_versioned_camel_case_envelope() {
        let response = ParseResponse::from(ParsedNorm {
            frontmatter: json!({"metadata": {}}),
            body: String::new(),
        });
        assert_eq!(
            json_value(response),
            json!({
                "apiVersion": "norm-spec/parse/v1",
                "frontmatter": {"metadata": {}},
                "body": ""
            })
        );
    }

    #[test]
    fn error_response_omits_absent_optional_context() {
        let response = ErrorResponse::new(
            "parse",
            ErrorDetail::new("norm/parse/empty", "empty").with_path(".norm"),
        );
        assert_eq!(
            json_value(response),
            json!({
                "apiVersion": "norm-spec/error/v1",
                "command": "parse",
                "error": {
                    "code": "norm/parse/empty",
                    "message": "empty",
                    "path": ".norm"
                }
            })
        );
    }

    #[test]
    fn collect_response_preserves_specificity_order() {
        let response = CollectResponse::new(
            "docs/module",
            vec![
                CollectedNorm::new(
                    "docs/module/.norm",
                    ParsedNorm {
                        frontmatter: json!({"metadata": {"layer": "module"}}),
                        body: "# Module".to_owned(),
                    },
                ),
                CollectedNorm::new(
                    ".norm",
                    ParsedNorm {
                        frontmatter: json!({"metadata": {"layer": "root"}}),
                        body: "# Root".to_owned(),
                    },
                ),
            ],
        );
        assert_eq!(
            json_value(response),
            json!({
                "apiVersion": "norm-spec/collect/v1",
                "root": ".",
                "target": "docs/module",
                "norms": [
                    {
                        "path": "docs/module/.norm",
                        "frontmatter": {"metadata": {"layer": "module"}},
                        "body": "# Module"
                    },
                    {
                        "path": ".norm",
                        "frontmatter": {"metadata": {"layer": "root"}},
                        "body": "# Root"
                    }
                ]
            })
        );
    }

    #[test]
    fn validate_response_orders_paths_diagnostics_and_counts_findings() {
        let response = ValidateResponse::new(vec![
            ValidationResult::new(
                "docs/.norm",
                Vec::new(),
                vec![
                    Diagnostic::new("norm/profile/z", "z").with_field("template"),
                    Diagnostic::new("norm/profile/a", "a").with_field("agent_rules"),
                ],
            ),
            ValidationResult::new(
                ".norm",
                vec![Diagnostic::new(
                    "norm/schema/version-format",
                    "invalid version",
                )],
                Vec::new(),
            ),
        ]);
        assert_eq!(response.results[0].path, ".norm");
        assert_eq!(response.results[0].status, ValidationStatus::Error);
        assert_eq!(response.results[1].status, ValidationStatus::Warning);
        assert_eq!(
            response.results[1].warnings[0].field.as_deref(),
            Some("agent_rules")
        );
        assert_eq!(response.summary.files, 2);
        assert_eq!(response.summary.errors, 1);
        assert_eq!(response.summary.warnings, 2);
        assert_eq!(json_value(response)["apiVersion"], "norm-spec/validate/v1");
    }

    #[test]
    fn init_response_uses_the_versioned_action_contract() {
        assert_eq!(
            json_value(InitResponse::new(
                "module",
                ".norm",
                InitAction::Overwritten,
            )),
            json!({
                "apiVersion": "norm-spec/init/v1",
                "profile": "module",
                "path": ".norm",
                "action": "overwritten"
            })
        );
    }
}
