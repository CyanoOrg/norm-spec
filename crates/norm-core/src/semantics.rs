//! Cross-field semantics and explicit reference-resolution boundaries.

use std::collections::BTreeSet;

use serde_json::Value;

use crate::{
    Diagnostic, SchemaBundle, SchemaCompileError, ValidationDiagnostics, ValidationOptions,
    validate_structure,
};

/// Filesystem observation for one local reference target.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceStatus {
    /// The target exists inside the project root.
    Exists,
    /// The target remains inside the root but does not exist.
    Missing,
    /// The target resolves outside the project root.
    OutsideRoot,
}

/// Explicit reference-resolution boundary implemented by filesystem adapters.
pub trait ReferenceResolver {
    /// Inspect one local target relative to the current `.norm` file.
    fn resolve(&self, target: &str) -> ReferenceStatus;
}

/// Validate schema, profile, lifecycle, SSOT, and reference semantics.
///
/// # Errors
///
/// Returns [`SchemaCompileError`] when the selected root or profile schema is
/// invalid or cannot resolve its references from the supplied bundle.
pub fn validate_frontmatter(
    frontmatter: &Value,
    schemas: &SchemaBundle,
    options: ValidationOptions<'_>,
    references: &dyn ReferenceResolver,
) -> Result<ValidationDiagnostics, SchemaCompileError> {
    let mut diagnostics = validate_structure(frontmatter, schemas, options)?;
    validate_lifecycle(frontmatter, &mut diagnostics);
    validate_single_sources(frontmatter, &mut diagnostics);
    validate_references(frontmatter, references, &mut diagnostics);
    Ok(diagnostics)
}

fn validate_lifecycle(frontmatter: &Value, diagnostics: &mut ValidationDiagnostics) {
    let Some(lifecycle) = frontmatter
        .get("agent_rules")
        .and_then(|rules| rules.get("document_lifecycle"))
    else {
        return;
    };
    if lifecycle.is_array() {
        diagnostics.warnings.push(
            Diagnostic::new(
                "norm/semantic/lifecycle-legacy",
                "The document lifecycle uses the legacy string-list form.",
            )
            .with_field("agent_rules.document_lifecycle"),
        );
        return;
    }
    let Some(lifecycle) = lifecycle.as_object() else {
        return;
    };
    let states = lifecycle
        .get("states")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|state| state.get("id").and_then(Value::as_str))
        .collect::<Vec<_>>();
    let state_set = states.iter().copied().collect::<BTreeSet<_>>();
    if states.len() != state_set.len() {
        push_lifecycle_error(diagnostics, "Lifecycle state identifiers must be unique.");
    }
    if let Some(initial) = lifecycle.get("initial").and_then(Value::as_str)
        && !state_set.contains(initial)
    {
        push_lifecycle_error(diagnostics, "The initial lifecycle state is not declared.");
    }
    let terminals = string_items(lifecycle.get("terminal")).collect::<BTreeSet<_>>();
    for terminal in &terminals {
        if !state_set.contains(terminal) {
            push_lifecycle_error(diagnostics, "A terminal lifecycle state is not declared.");
        }
    }
    let mut transitions = BTreeSet::new();
    for transition in lifecycle
        .get("transitions")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_object)
    {
        let source = transition.get("from").and_then(Value::as_str);
        let target = transition.get("to").and_then(Value::as_str);
        if source.is_some_and(|source| !state_set.contains(source)) {
            push_lifecycle_error(
                diagnostics,
                "A lifecycle transition source is not declared.",
            );
        }
        if target.is_some_and(|target| !state_set.contains(target)) {
            push_lifecycle_error(
                diagnostics,
                "A lifecycle transition target is not declared.",
            );
        }
        if let (Some(source), Some(target)) = (source, target) {
            if !transitions.insert((source, target)) {
                push_lifecycle_error(diagnostics, "Lifecycle transitions must be unique.");
            }
            if terminals.contains(source) {
                push_lifecycle_error(
                    diagnostics,
                    "A terminal lifecycle state cannot have an outgoing transition.",
                );
            }
        }
    }
}

fn push_lifecycle_error(diagnostics: &mut ValidationDiagnostics, message: &str) {
    diagnostics.errors.push(
        Diagnostic::new("norm/semantic/lifecycle", message)
            .with_field("agent_rules.document_lifecycle"),
    );
}

fn validate_single_sources(frontmatter: &Value, diagnostics: &mut ValidationDiagnostics) {
    let domains = frontmatter
        .get("template")
        .and_then(|template| template.get("single_source_of_truth"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|item| item.get("for").and_then(Value::as_str));
    let mut seen = BTreeSet::new();
    let mut duplicates = BTreeSet::new();
    for domain in domains {
        if !seen.insert(domain) {
            duplicates.insert(domain);
        }
    }
    for domain in duplicates {
        diagnostics.errors.push(
            Diagnostic::new(
                "norm/semantic/ssot-duplicate",
                format!("The single-source domain is declared more than once: {domain}"),
            )
            .with_field("template.single_source_of_truth"),
        );
    }
}

fn validate_references(
    frontmatter: &Value,
    resolver: &dyn ReferenceResolver,
    diagnostics: &mut ValidationDiagnostics,
) {
    let policy = frontmatter
        .get("agent_rules")
        .and_then(|rules| rules.get("reference_policy"))
        .and_then(Value::as_object);
    let must_exist = policy
        .and_then(|policy| policy.get("target_must_exist"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let allow_external = policy
        .and_then(|policy| policy.get("allow_external"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let require_description = policy
        .and_then(|policy| policy.get("require_description"))
        .and_then(Value::as_bool)
        .unwrap_or(false);

    for collection in ["cross_references", "strong_references"] {
        let mut seen = BTreeSet::new();
        for (index, reference) in frontmatter
            .get(collection)
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .enumerate()
        {
            let Some(reference) = reference.as_object() else {
                continue;
            };
            let Some(target) = reference.get("target").and_then(Value::as_str) else {
                continue;
            };
            let target_field = format!("{collection}[{index}].target");
            let identity = (
                reference.get("type").and_then(Value::as_str).unwrap_or(""),
                target,
            );
            if !seen.insert(identity) {
                diagnostics.errors.push(
                    Diagnostic::new(
                        "norm/reference/duplicate",
                        format!("The typed reference target is duplicated: {target}"),
                    )
                    .with_field(&target_field),
                );
            }
            if require_description
                && reference
                    .get("description")
                    .and_then(Value::as_str)
                    .is_none_or(str::is_empty)
            {
                diagnostics.errors.push(
                    Diagnostic::new(
                        "norm/reference/description-required",
                        format!("The reference requires a description: {target}"),
                    )
                    .with_field(format!("{collection}[{index}].description")),
                );
            }
            let external = target.contains("://");
            if external && !allow_external {
                diagnostics.errors.push(
                    Diagnostic::new(
                        "norm/reference/external-not-allowed",
                        format!("External references are not allowed: {target}"),
                    )
                    .with_field(&target_field),
                );
                continue;
            }
            if must_exist && !external {
                match resolver.resolve(target) {
                    ReferenceStatus::Exists => {}
                    ReferenceStatus::Missing => diagnostics.errors.push(
                        Diagnostic::new(
                            "norm/reference/not-found",
                            format!("The reference target does not exist: {target}"),
                        )
                        .with_field(&target_field),
                    ),
                    ReferenceStatus::OutsideRoot => diagnostics.errors.push(
                        Diagnostic::new(
                            "norm/reference/outside-root",
                            format!("The reference target resolves outside the root: {target}"),
                        )
                        .with_field(&target_field),
                    ),
                }
            }
        }
    }
}

fn string_items(value: Option<&Value>) -> impl Iterator<Item = &str> {
    value
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{ReferenceResolver, ReferenceStatus, validate_frontmatter};
    use crate::{SchemaBundle, ValidationOptions};

    struct Resolver {
        status: ReferenceStatus,
    }

    impl ReferenceResolver for Resolver {
        fn resolve(&self, _target: &str) -> ReferenceStatus {
            self.status
        }
    }

    fn schemas() -> SchemaBundle {
        let root: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../schema/norm-schema.json"
        )))
        .unwrap_or_else(|error| panic!("root schema fixture should parse: {error}"));
        SchemaBundle::new(root, [])
    }

    fn valid() -> serde_json::Value {
        json!({"metadata": {"layer": "example", "scope": "./", "version": "1.0"}})
    }

    #[test]
    fn lifecycle_and_single_source_invariants_are_checked() {
        let mut frontmatter = valid();
        frontmatter["agent_rules"] = json!({
            "document_lifecycle": {
                "state_field": "state",
                "states": [{"id": "draft"}, {"id": "draft"}],
                "initial": "missing",
                "terminal": ["missing"],
                "transitions": [
                    {"from": "draft", "to": "missing"},
                    {"from": "draft", "to": "missing"}
                ]
            }
        });
        frontmatter["template"] = json!({
            "single_source_of_truth": [
                {"source": "a", "for": "architecture"},
                {"source": "b", "for": "architecture"}
            ]
        });
        let diagnostics = validate_frontmatter(
            &frontmatter,
            &schemas(),
            ValidationOptions::default(),
            &Resolver {
                status: ReferenceStatus::Exists,
            },
        )
        .unwrap_or_else(|error| panic!("schemas should compile: {error}"));
        assert!(
            diagnostics
                .errors
                .iter()
                .any(|diagnostic| diagnostic.code == "norm/semantic/lifecycle")
        );
        assert!(
            diagnostics
                .errors
                .iter()
                .any(|diagnostic| diagnostic.code == "norm/semantic/ssot-duplicate")
        );
    }

    #[test]
    fn reference_policy_uses_explicit_resolution_observations() {
        let mut frontmatter = valid();
        frontmatter["agent_rules"] = json!({
            "reference_policy": {
                "target_must_exist": true,
                "allow_external": false,
                "require_description": true
            }
        });
        frontmatter["strong_references"] = json!([
            {"type": "source", "target": "missing.md", "sync": "manual", "validation": "strict"},
            {"type": "source", "target": "missing.md", "sync": "manual", "validation": "strict"},
            {"type": "source", "target": "https://example.invalid", "sync": "manual", "validation": "strict", "description": "external"}
        ]);
        let diagnostics = validate_frontmatter(
            &frontmatter,
            &schemas(),
            ValidationOptions::default(),
            &Resolver {
                status: ReferenceStatus::Missing,
            },
        )
        .unwrap_or_else(|error| panic!("schemas should compile: {error}"));
        for code in [
            "norm/reference/not-found",
            "norm/reference/duplicate",
            "norm/reference/description-required",
            "norm/reference/external-not-allowed",
        ] {
            assert!(
                diagnostics
                    .errors
                    .iter()
                    .any(|diagnostic| diagnostic.code == code),
                "missing reference diagnostic {code}"
            );
        }
    }
}
