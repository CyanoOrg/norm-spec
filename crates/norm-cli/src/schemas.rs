//! Explicit loading of embedded or caller-selected schema bundles.

use std::{
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

use norm_spec_core::SchemaBundle;
use serde_json::Value;

const EMBEDDED_ROOT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schema/norm-schema.json"
));
const EMBEDDED_PROFILES: [(&str, &str); 7] = [
    (
        "architecture",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../schema/profiles/architecture.json"
        )),
    ),
    (
        "convention",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../schema/profiles/convention.json"
        )),
    ),
    (
        "epic",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../schema/profiles/epic.json"
        )),
    ),
    (
        "module",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../schema/profiles/module.json"
        )),
    ),
    (
        "root",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../schema/profiles/root.json"
        )),
    ),
    (
        "task",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../schema/profiles/task.json"
        )),
    ),
    (
        "test",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../schema/profiles/test.json"
        )),
    ),
];

#[derive(Debug)]
pub(crate) struct SchemaLoadError {
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

    pub(crate) fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
}

impl fmt::Display for SchemaLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl Error for SchemaLoadError {}

pub(crate) fn load(explicit: Option<&Path>) -> Result<SchemaBundle, SchemaLoadError> {
    explicit.map_or_else(load_embedded, load_directory)
}

fn load_embedded() -> Result<SchemaBundle, SchemaLoadError> {
    let root = parse_schema(None, EMBEDDED_ROOT)?;
    let profiles = EMBEDDED_PROFILES
        .into_iter()
        .map(|(name, contents)| {
            parse_schema(None, contents).map(|schema| (name.to_owned(), schema))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(SchemaBundle::new(root, profiles))
}

fn load_directory(directory: &Path) -> Result<SchemaBundle, SchemaLoadError> {
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
    use std::path::PathBuf;

    use super::load;

    #[test]
    fn embedded_and_explicit_bundles_include_known_profiles() {
        let embedded =
            load(None).unwrap_or_else(|error| panic!("embedded schemas should load: {error}"));
        assert!(embedded.contains_profile("module"));

        let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../schema");
        let explicit = load(Some(&directory))
            .unwrap_or_else(|error| panic!("repository schemas should load: {error}"));
        assert!(explicit.contains_profile("convention"));
    }
}
