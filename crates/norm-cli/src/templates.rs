//! Build-time profile templates used by `norm init`.

const PROFILE_TEMPLATES: [(&str, &str); 7] = [
    (
        "architecture",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../templates/profiles/architecture.norm"
        )),
    ),
    (
        "convention",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../templates/profiles/convention.norm"
        )),
    ),
    (
        "epic",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../templates/profiles/epic.norm"
        )),
    ),
    (
        "module",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../templates/profiles/module.norm"
        )),
    ),
    (
        "root",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../templates/profiles/root.norm"
        )),
    ),
    (
        "task",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../templates/profiles/task.norm"
        )),
    ),
    (
        "test",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../templates/profiles/test.norm"
        )),
    ),
];

pub(crate) fn get(profile: &str) -> Option<&'static str> {
    PROFILE_TEMPLATES
        .iter()
        .find_map(|(name, contents)| (*name == profile).then_some(*contents))
}

#[cfg(test)]
mod tests {
    use super::PROFILE_TEMPLATES;
    use crate::schemas;
    use norm_spec_core::{
        ParseOptions, ReferenceResolver, ReferenceStatus, ValidationOptions, parse_norm,
        validate_frontmatter,
    };

    struct ExistingReferences;

    impl ReferenceResolver for ExistingReferences {
        fn resolve(&self, _target: &str) -> ReferenceStatus {
            ReferenceStatus::Exists
        }
    }

    #[test]
    fn every_embedded_profile_is_a_valid_a1_document() {
        assert_eq!(PROFILE_TEMPLATES.len(), 7);
        let schemas = schemas::load(None)
            .unwrap_or_else(|error| panic!("embedded schemas should load: {error}"));
        for (name, contents) in PROFILE_TEMPLATES {
            let parsed = parse_norm(contents, ParseOptions::default())
                .unwrap_or_else(|error| panic!("embedded {name} template should parse: {error}"));
            assert_eq!(parsed.frontmatter["metadata"]["layer"].as_str(), Some(name));
            let diagnostics = validate_frontmatter(
                &parsed.frontmatter,
                &schemas,
                ValidationOptions {
                    compat_keys: false,
                    profile: Some(name),
                },
                &ExistingReferences,
            )
            .unwrap_or_else(|error| {
                panic!("embedded {name} template schema should compile: {error}")
            });
            assert!(
                diagnostics.errors.is_empty(),
                "embedded {name} template has errors: {:?}",
                diagnostics.errors
            );
            assert!(
                diagnostics.warnings.is_empty(),
                "embedded {name} template has warnings: {:?}",
                diagnostics.warnings
            );
        }
    }
}
