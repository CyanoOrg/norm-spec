//! Offline JSON Schema Draft 7 evaluation primitives.

use std::{collections::BTreeMap, error::Error, fmt};

use jsonschema::{Retrieve, Uri, error::ValidationErrorKind};
use serde_json::Value;

/// One structured Draft 7 schema violation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaViolation {
    /// JSON Schema keyword that rejected the instance.
    pub keyword: String,
    /// JSON Pointer into the validated instance.
    pub instance_path: String,
    /// JSON Pointer to the evaluating schema keyword.
    pub schema_path: String,
    /// Unexpected property names for `additionalProperties` violations.
    pub unexpected_properties: Vec<String>,
}

/// Failure to compile a Draft 7 schema from the supplied offline resources.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaCompileError {
    detail: String,
}

impl SchemaCompileError {
    fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }
}

impl fmt::Display for SchemaCompileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl Error for SchemaCompileError {}

#[derive(Clone, Debug, Default)]
struct OfflineRetriever {
    resources: BTreeMap<String, Value>,
}

impl Retrieve for OfflineRetriever {
    fn retrieve(&self, uri: &Uri<String>) -> Result<Value, Box<dyn Error + Send + Sync + 'static>> {
        self.resources
            .get(uri.as_str())
            .cloned()
            .ok_or_else(|| format!("schema resource is not registered: {uri}").into())
    }
}

/// Validate an instance with an explicit Draft 7 schema and offline resources.
///
/// Resource keys are absolute schema identifiers. Relative `$ref` values are
/// resolved against the schema's `$id` and may retrieve only from this supplied
/// map. The function never reads files, environment variables, or the network.
///
/// # Errors
///
/// Returns [`SchemaCompileError`] when the schema is invalid or a referenced
/// resource is not present in the supplied map.
pub fn validate_draft7(
    schema: &Value,
    instance: &Value,
    resources: impl IntoIterator<Item = (String, Value)>,
) -> Result<Vec<SchemaViolation>, SchemaCompileError> {
    let retriever = OfflineRetriever {
        resources: resources.into_iter().collect(),
    };
    let validator = jsonschema::draft7::options()
        .with_retriever(retriever)
        .should_validate_formats(true)
        .build(schema)
        .map_err(|error| SchemaCompileError::new(error.to_string()))?;

    let mut violations = validator
        .iter_errors(instance)
        .map(|error| {
            let mut unexpected_properties = match error.kind() {
                ValidationErrorKind::AdditionalProperties { unexpected } => unexpected.clone(),
                _ => Vec::new(),
            };
            unexpected_properties.sort();
            SchemaViolation {
                keyword: error.kind().keyword().to_owned(),
                instance_path: error.instance_path().to_string(),
                schema_path: error.schema_path().to_string(),
                unexpected_properties,
            }
        })
        .collect::<Vec<_>>();
    violations.sort_by(|left, right| {
        left.instance_path
            .cmp(&right.instance_path)
            .then_with(|| left.keyword.cmp(&right.keyword))
            .then_with(|| left.schema_path.cmp(&right.schema_path))
            .then_with(|| left.unexpected_properties.cmp(&right.unexpected_properties))
    });
    Ok(violations)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::validate_draft7;

    const ROOT_ID: &str = "https://norm-spec.dev/schema/norm-schema.json";

    fn root_schema() -> serde_json::Value {
        json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "$id": ROOT_ID,
            "type": "object",
            "required": ["metadata"]
        })
    }

    #[test]
    fn draft7_validation_returns_structured_paths_and_keywords() {
        let violations = validate_draft7(&root_schema(), &json!({}), [])
            .unwrap_or_else(|error| panic!("test schema should compile: {error}"));
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].keyword, "required");
        assert_eq!(violations[0].instance_path, "");
        assert_eq!(violations[0].schema_path, "/required");
    }

    #[test]
    fn relative_references_resolve_only_from_supplied_resources() {
        let profile = json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "$id": "https://norm-spec.dev/schema/profiles/test.json",
            "allOf": [{"$ref": "../norm-schema.json"}]
        });
        let instance = json!({"metadata": {}});
        let violations =
            validate_draft7(&profile, &instance, [(ROOT_ID.to_owned(), root_schema())])
                .unwrap_or_else(|error| {
                    panic!("registered relative reference should compile: {error}")
                });
        assert!(violations.is_empty());
    }

    #[test]
    fn missing_external_resources_fail_instead_of_falling_back() {
        let profile = json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "$id": "https://norm-spec.dev/schema/profiles/test.json",
            "allOf": [{"$ref": "../norm-schema.json"}]
        });
        let error = match validate_draft7(&profile, &json!({}), []) {
            Ok(violations) => {
                panic!("unregistered reference unexpectedly compiled: {violations:?}")
            }
            Err(error) => error,
        };
        assert!(
            error
                .to_string()
                .contains("schema resource is not registered")
        );
        assert!(error.to_string().contains(ROOT_ID));
    }
}
