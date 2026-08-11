use std::{
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

use norm_spec_core::SchemaBundle;
use serde_json::Value;

const EMBEDDED_ROOT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/schema/norm-schema.json"
));
const EMBEDDED_PROFILES: [(&str, &str); 7] = [
    (
        "architecture",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/schema/profiles/architecture.json"
        )),
    ),
    (
        "convention",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/schema/profiles/convention.json"
        )),
    ),
    (
        "epic",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/schema/profiles/epic.json"
        )),
    ),
    (
        "module",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/schema/profiles/module.json"
        )),
    ),
    (
        "root",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/schema/profiles/root.json"
        )),
    ),
    (
        "task",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/schema/profiles/task.json"
        )),
    ),
    (
        "test",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/schema/profiles/test.json"
        )),
    ),
];
const EMBEDDED_TEMPLATES: [(&str, &str); 7] = [
    (
        "architecture",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/profiles/architecture.norm"
        )),
    ),
    (
        "convention",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/profiles/convention.norm"
        )),
    ),
    (
        "epic",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/profiles/epic.norm"
        )),
    ),
    (
        "module",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/profiles/module.norm"
        )),
    ),
    (
        "root",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/profiles/root.norm"
        )),
    ),
    (
        "task",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/profiles/task.norm"
        )),
    ),
    (
        "test",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/profiles/test.norm"
        )),
    ),
];

/// Names of the release-owned Schema profiles and authoring templates.
pub const PROFILE_NAMES: [&str; 7] = [
    "architecture",
    "convention",
    "epic",
    "module",
    "root",
    "task",
    "test",
];

/// Return one packaged authoring template by profile name.
#[must_use]
pub fn profile_template(profile: &str) -> Option<&'static str> {
    EMBEDDED_TEMPLATES
        .iter()
        .find_map(|(name, contents)| (*name == profile).then_some(*contents))
}

/// Failure to load or parse a packaged or explicit Schema bundle.
#[derive(Debug)]
pub struct SchemaLoadError {
    path: Option<PathBuf>,
    detail: String,
}

impl SchemaLoadError {
    fn new(path: Option<PathBuf>, detail: impl Into<String>) -> Self {
        Self {
            path,
            detail: detail.into(),
        }
    }

    /// Return the explicit Schema path associated with the failure, if any.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}

impl fmt::Display for SchemaLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl Error for SchemaLoadError {}

/// Load the release-owned Schema bundle embedded in this package.
///
/// # Errors
///
/// Returns [`SchemaLoadError`] if a packaged Schema is not valid JSON.
pub fn embedded_schema_bundle() -> Result<SchemaBundle, SchemaLoadError> {
    let root = parse_schema(None, EMBEDDED_ROOT)?;
    let profiles = EMBEDDED_PROFILES
        .into_iter()
        .map(|(name, contents)| {
            parse_schema(None, contents).map(|schema| (name.to_owned(), schema))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(SchemaBundle::new(root, profiles))
}

/// Load a root Schema and profile Schemas from an explicit directory.
///
/// # Errors
///
/// Returns [`SchemaLoadError`] when the directory or a required Schema cannot
/// be read or parsed.
pub fn schema_bundle_from_dir(directory: &Path) -> Result<SchemaBundle, SchemaLoadError> {
    if !directory.is_dir() {
        return Err(SchemaLoadError::new(
            Some(directory.to_path_buf()),
            "The explicit schema path is not an available directory.",
        ));
    }
    let root_path = directory.join("norm-schema.json");
    let root = read_schema(&root_path)?;
    let profiles_path = directory.join("profiles");
    let mut profile_paths = fs::read_dir(&profiles_path)
        .map_err(|error| {
            SchemaLoadError::new(
                Some(profiles_path.clone()),
                format!("The profile schema directory is unavailable: {error}"),
            )
        })?
        .map(|entry| {
            entry.map(|entry| entry.path()).map_err(|error| {
                SchemaLoadError::new(
                    Some(profiles_path.clone()),
                    format!("A profile schema entry is unavailable: {error}"),
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    profile_paths.retain(|path| {
        path.extension()
            .is_some_and(|extension| extension == "json")
    });
    profile_paths.sort();
    let profiles = profile_paths
        .into_iter()
        .map(|path| {
            let name = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .ok_or_else(|| {
                    SchemaLoadError::new(
                        Some(path.clone()),
                        "A profile schema filename is not valid UTF-8.",
                    )
                })?
                .to_owned();
            read_schema(&path).map(|schema| (name, schema))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(SchemaBundle::new(root, profiles))
}

fn read_schema(path: &Path) -> Result<Value, SchemaLoadError> {
    let contents = fs::read_to_string(path).map_err(|error| {
        SchemaLoadError::new(
            Some(path.to_path_buf()),
            format!("The schema file is unavailable: {error}"),
        )
    })?;
    parse_schema(Some(path), &contents)
}

fn parse_schema(path: Option<&Path>, contents: &str) -> Result<Value, SchemaLoadError> {
    serde_json::from_str(contents).map_err(|error| {
        SchemaLoadError::new(
            path.map(Path::to_path_buf),
            format!("The schema file is not valid JSON: {error}"),
        )
    })
}

#[cfg(test)]
mod tests {
    use norm_spec_core::{
        ParseOptions, ReferenceResolver, ReferenceStatus, ValidationOptions, parse_norm,
        validate_frontmatter,
    };

    use super::{EMBEDDED_TEMPLATES, embedded_schema_bundle};

    struct ExistingReferences;

    impl ReferenceResolver for ExistingReferences {
        fn resolve(&self, _target: &str) -> ReferenceStatus {
            ReferenceStatus::Exists
        }
    }

    #[test]
    fn every_packaged_profile_template_is_valid() {
        let schemas = embedded_schema_bundle()
            .unwrap_or_else(|error| panic!("packaged schemas should load: {error}"));
        for (name, contents) in EMBEDDED_TEMPLATES {
            let parsed = parse_norm(contents, ParseOptions::default())
                .unwrap_or_else(|error| panic!("packaged {name} template should parse: {error}"));
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
                panic!("packaged {name} template schema should compile: {error}")
            });
            assert!(
                diagnostics.errors.is_empty(),
                "packaged {name} template has errors: {:?}",
                diagnostics.errors
            );
            assert!(
                diagnostics.warnings.is_empty(),
                "packaged {name} template has warnings: {:?}",
                diagnostics.warnings
            );
        }
    }
}
