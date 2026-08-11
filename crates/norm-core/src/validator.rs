//! Draft 7 schema and profile validation over parsed frontmatter.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::{Diagnostic, SchemaCompileError, SchemaViolation, validate_draft7};

const KNOWN_TOP_LEVEL: [&str; 6] = [
    "agent_rules",
    "cross_references",
    "metadata",
    "scope",
    "strong_references",
    "template",
];

/// Root and profile schemas supplied explicitly by a caller.
#[derive(Clone, Debug, PartialEq)]
pub struct SchemaBundle {
    root: Value,
    profiles: BTreeMap<String, Value>,
}

impl SchemaBundle {
    /// Construct a schema bundle keyed by profile name.
    #[must_use]
    pub fn new(root: Value, profiles: impl IntoIterator<Item = (String, Value)>) -> Self {
        Self {
            root,
            profiles: profiles.into_iter().collect(),
        }
    }

    /// Return whether the bundle contains a named profile.
    #[must_use]
    pub fn contains_profile(&self, name: &str) -> bool {
        self.profiles.contains_key(name)
    }
}

/// Options that alter validation severity or profile selection.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ValidationOptions<'a> {
    /// Downgrade unknown top-level fields from errors to warnings.
    pub compat_keys: bool,
    /// Override metadata-based profile selection.
    pub profile: Option<&'a str>,
}

/// Schema and profile findings for one parsed document.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValidationDiagnostics {
    /// Hard validation failures.
    pub errors: Vec<Diagnostic>,
    /// Non-fatal compatibility and recommendation findings.
    pub warnings: Vec<Diagnostic>,
}

/// Validate parsed frontmatter without reading the filesystem or environment.
///
/// # Errors
///
/// Returns [`SchemaCompileError`] when the selected root or profile schema is
/// invalid or cannot resolve its references from the supplied bundle.
pub fn validate_structure(
    frontmatter: &Value,
    schemas: &SchemaBundle,
    options: ValidationOptions<'_>,
) -> Result<ValidationDiagnostics, SchemaCompileError> {
    let mut diagnostics = ValidationDiagnostics::default();
    let selected_profile = select_profile(frontmatter, schemas, options.profile, &mut diagnostics);
    let schema = selected_profile
        .and_then(|name| schemas.profiles.get(name))
        .unwrap_or(&schemas.root);
    let resources = schema_identifier(&schemas.root)
        .map(|identifier| [(identifier, schemas.root.clone())])
        .unwrap_or_default();
    let violations = validate_draft7(schema, frontmatter, resources)?;
    map_schema_violations(
        &violations,
        options.compat_keys,
        &mut diagnostics.errors,
        &mut diagnostics.warnings,
    );
    if let Some(profile) = selected_profile.and_then(|name| schemas.profiles.get(name)) {
        validate_profile_rules(frontmatter, profile, &mut diagnostics);
    }
    Ok(diagnostics)
}

fn schema_identifier(schema: &Value) -> Option<String> {
    schema.get("$id")?.as_str().map(str::to_owned)
}

fn select_profile<'a>(
    frontmatter: &'a Value,
    schemas: &'a SchemaBundle,
    override_profile: Option<&'a str>,
    diagnostics: &mut ValidationDiagnostics,
) -> Option<&'a str> {
    if let Some(profile) = override_profile {
        return schemas.contains_profile(profile).then_some(profile);
    }
    let metadata = frontmatter.get("metadata")?.as_object()?;
    if let Some(explicit) = metadata.get("profile").and_then(Value::as_str) {
        if schemas.contains_profile(explicit) {
            return Some(explicit);
        }
        diagnostics.warnings.push(
            Diagnostic::new(
                "norm/profile/unknown-explicit",
                format!("The explicit profile is not available: {explicit}"),
            )
            .with_field("metadata.profile"),
        );
        return None;
    }
    metadata
        .get("layer")
        .and_then(Value::as_str)
        .filter(|layer| schemas.contains_profile(layer))
}

fn map_schema_violations(
    violations: &[SchemaViolation],
    compat_keys: bool,
    errors: &mut Vec<Diagnostic>,
    warnings: &mut Vec<Diagnostic>,
) {
    for violation in violations {
        if violation.keyword == "additionalProperties" && violation.instance_path.is_empty() {
            for field in &violation.unexpected_properties {
                let mut diagnostic = Diagnostic::new(
                    "norm/schema/unknown-key",
                    format!("Unknown top-level key: {field}"),
                )
                .with_field(field);
                if let Some(suggestion) = closest_key(field, &KNOWN_TOP_LEVEL, 3) {
                    diagnostic = diagnostic.with_suggestion(suggestion);
                }
                if compat_keys {
                    warnings.push(diagnostic);
                } else {
                    errors.push(diagnostic);
                }
            }
        } else if violation.keyword == "pattern" && violation.instance_path == "/metadata/version" {
            errors.push(
                Diagnostic::new(
                    "norm/schema/version-format",
                    "metadata.version must use MAJOR.MINOR format.",
                )
                .with_field("metadata.version")
                .with_suggestion("MAJOR.MINOR"),
            );
        } else {
            let field = pointer_to_field(&violation.instance_path);
            let mut diagnostic = Diagnostic::new(
                "norm/schema/invalid",
                format!(
                    "The value does not satisfy the Draft 7 '{}' keyword.",
                    violation.keyword
                ),
            );
            if !field.is_empty() {
                diagnostic = diagnostic.with_field(field);
            }
            errors.push(diagnostic);
        }
    }
}

fn validate_profile_rules(
    frontmatter: &Value,
    profile: &Value,
    diagnostics: &mut ValidationDiagnostics,
) {
    let Some(rules) = profile.get("profile_rules").and_then(Value::as_object) else {
        return;
    };
    for field in string_items(rules.get("required_fields")) {
        if frontmatter.get(field).is_none() {
            diagnostics.errors.push(
                Diagnostic::new(
                    "norm/schema/invalid",
                    format!("The selected profile requires '{field}'."),
                )
                .with_field(field),
            );
        }
    }
    for field in string_items(rules.get("recommended_fields")) {
        if frontmatter.get(field).is_none() {
            push_profile_warning(diagnostics, field);
        }
    }
    for (parent, rule) in [
        ("template", "recommended_template"),
        ("agent_rules", "recommended_agent_rules"),
        ("scope", "recommended_scope"),
    ] {
        let Some(parent_value) = frontmatter.get(parent).and_then(Value::as_object) else {
            continue;
        };
        for child in string_items(rules.get(rule)) {
            if !parent_value.contains_key(child) {
                push_profile_warning(diagnostics, &format!("{parent}.{child}"));
            }
        }
    }
}

fn push_profile_warning(diagnostics: &mut ValidationDiagnostics, field: &str) {
    diagnostics.warnings.push(
        Diagnostic::new(
            "norm/profile/recommended-field",
            format!("The selected profile recommends '{field}'."),
        )
        .with_field(field),
    );
}

fn string_items(value: Option<&Value>) -> impl Iterator<Item = &str> {
    value
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
}

fn closest_key<'a>(key: &str, candidates: &'a [&str], threshold: usize) -> Option<&'a str> {
    candidates
        .iter()
        .copied()
        .map(|candidate| (candidate, levenshtein(key, candidate)))
        .filter(|(_, distance)| *distance <= threshold)
        .min_by(|left, right| left.1.cmp(&right.1).then_with(|| left.0.cmp(right.0)))
        .map(|(candidate, _)| candidate)
}

fn levenshtein(left: &str, right: &str) -> usize {
    let right = right.chars().collect::<Vec<_>>();
    let mut previous = (0..=right.len()).collect::<Vec<_>>();
    for (left_index, left_character) in left.chars().enumerate() {
        let mut current = vec![left_index + 1];
        for (right_index, right_character) in right.iter().enumerate() {
            current.push(
                (previous[right_index + 1] + 1)
                    .min(current[right_index] + 1)
                    .min(previous[right_index] + usize::from(left_character != *right_character)),
            );
        }
        previous = current;
    }
    previous[right.len()]
}

fn pointer_to_field(pointer: &str) -> String {
    let mut field = String::new();
    for token in pointer
        .strip_prefix('/')
        .unwrap_or(pointer)
        .split('/')
        .filter(|token| !token.is_empty())
    {
        let token = token.replace("~1", "/").replace("~0", "~");
        if token.chars().all(|character| character.is_ascii_digit()) {
            field.push('[');
            field.push_str(&token);
            field.push(']');
        } else {
            if !field.is_empty() {
                field.push('.');
            }
            field.push_str(&token);
        }
    }
    field
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{ValidationOptions, validate_structure};
    use crate::test_support::schemas;

    fn valid() -> serde_json::Value {
        json!({"metadata": {"layer": "example", "scope": "./", "version": "1.0"}})
    }

    #[test]
    fn valid_free_form_layer_has_no_findings() {
        let diagnostics = validate_structure(&valid(), &schemas(), ValidationOptions::default())
            .unwrap_or_else(|error| panic!("schemas should compile: {error}"));
        assert!(diagnostics.errors.is_empty());
        assert!(diagnostics.warnings.is_empty());
    }

    #[test]
    fn unknown_keys_and_versions_have_specialized_diagnostics() {
        let mut frontmatter = valid();
        frontmatter["metadata"]["version"] = json!("1.0.0");
        frontmatter["agent_rulez"] = json!({});
        let diagnostics =
            validate_structure(&frontmatter, &schemas(), ValidationOptions::default())
                .unwrap_or_else(|error| panic!("schemas should compile: {error}"));
        assert!(diagnostics.errors.iter().any(|diagnostic| {
            diagnostic.code == "norm/schema/unknown-key"
                && diagnostic.field.as_deref() == Some("agent_rulez")
                && diagnostic.suggestion.as_deref() == Some("agent_rules")
        }));
        assert!(diagnostics.errors.iter().any(|diagnostic| {
            diagnostic.code == "norm/schema/version-format"
                && diagnostic.field.as_deref() == Some("metadata.version")
        }));
    }

    #[test]
    fn compat_keys_downgrades_only_unknown_top_level_fields() {
        let mut frontmatter = valid();
        frontmatter["agent_rulez"] = json!({});
        let diagnostics = validate_structure(
            &frontmatter,
            &schemas(),
            ValidationOptions {
                compat_keys: true,
                profile: None,
            },
        )
        .unwrap_or_else(|error| panic!("schemas should compile: {error}"));
        assert!(diagnostics.errors.is_empty());
        assert_eq!(diagnostics.warnings[0].code, "norm/schema/unknown-key");
    }

    #[test]
    fn profiles_warn_for_recommendations_and_unknown_explicit_names() {
        let profile_diagnostics = validate_structure(
            &valid(),
            &schemas(),
            ValidationOptions {
                compat_keys: false,
                profile: Some("convention"),
            },
        )
        .unwrap_or_else(|error| panic!("profile schema should compile: {error}"));
        assert!(profile_diagnostics.warnings.iter().any(|diagnostic| {
            diagnostic.code == "norm/profile/recommended-field"
                && diagnostic.field.as_deref() == Some("template")
        }));

        let mut frontmatter = valid();
        frontmatter["metadata"]["profile"] = json!("missing");
        let unknown_diagnostics =
            validate_structure(&frontmatter, &schemas(), ValidationOptions::default())
                .unwrap_or_else(|error| panic!("root schema should compile: {error}"));
        assert!(unknown_diagnostics.warnings.iter().any(|diagnostic| {
            diagnostic.code == "norm/profile/unknown-explicit"
                && diagnostic.field.as_deref() == Some("metadata.profile")
        }));
    }
}
