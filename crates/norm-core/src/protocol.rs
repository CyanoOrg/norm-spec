//! Versioned machine response models.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{COLLECT_API_VERSION, ERROR_API_VERSION, PARSE_API_VERSION, ParsedNorm};

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

    use super::{CollectResponse, CollectedNorm, ErrorDetail, ErrorResponse, ParseResponse};
    use crate::ParsedNorm;

    fn json_value(value: impl Serialize) -> serde_json::Value {
        match serde_json::to_value(value) {
            Ok(value) => value,
            Err(error) => panic!("failed to serialize test response: {error}"),
        }
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
}
