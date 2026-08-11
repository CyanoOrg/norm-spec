//! Deterministic structural-scan classification and aggregation.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::SCAN_API_VERSION;

const RECURRING_FILENAME_LIMIT: usize = 20;

/// Filesystem-neutral observation of one traversed directory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectoryObservation {
    path: String,
    file_names: Vec<String>,
}

impl DirectoryObservation {
    /// Construct an observation from a portable relative path and regular files.
    #[must_use]
    pub fn new(
        path: impl Into<String>,
        file_names: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            path: path.into(),
            file_names: file_names.into_iter().map(Into::into).collect(),
        }
    }
}

/// Kind of symbolic link observed without traversal.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ScanSymlinkKind {
    /// Link target is a directory.
    Directory,
    /// Link target is a regular file.
    File,
    /// Link target is unavailable or another filesystem object.
    Other,
}

/// Action taken for an observed symbolic link.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScanSymlinkAction {
    /// The scanner reported the link without following it.
    NotFollowed,
}

/// Portable symbolic-link observation in a scan response.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScanSymlink {
    /// Path relative to the scan root.
    pub path: String,
    /// Observed target kind.
    pub kind: ScanSymlinkKind,
    /// Explicit no-follow action.
    pub action: ScanSymlinkAction,
}

impl ScanSymlink {
    /// Construct a no-follow symbolic-link observation.
    #[must_use]
    pub fn new(path: impl Into<String>, kind: ScanSymlinkKind) -> Self {
        Self {
            path: path.into(),
            kind,
            action: ScanSymlinkAction::NotFollowed,
        }
    }
}

/// One traversed directory summarized for machine output.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScannedDirectory {
    /// Path relative to the scan root.
    pub path: String,
    /// Number of path components below the scan root.
    pub depth: usize,
    /// Number of visible regular files, excluding `.norm` and other dotfiles.
    pub file_count: usize,
    /// Whether the directory contains a regular `.norm` file.
    pub has_norm: bool,
}

/// Naming-style counts collected from directories and visible files.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScanNaming {
    /// Counts for non-root directories, including untraversed directory links.
    pub directories: BTreeMap<String, usize>,
    /// Counts for visible regular filenames.
    pub files: BTreeMap<String, usize>,
}

/// Filename found in two or more traversed directories.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RecurringFilename {
    /// Exact filename including its extension.
    pub name: String,
    /// Number of distinct directories containing the filename.
    pub dir_count: usize,
}

/// Aggregate `.norm` coverage across traversed directories.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct NormCoverage {
    /// Total traversed directories, including the root.
    pub total_dirs: usize,
    /// Traversed directories containing a regular `.norm` file.
    pub dirs_with_norm: usize,
    /// Coverage ratio rounded to three decimal places.
    pub ratio: f64,
}

/// Versioned deterministic structural-scan response.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ScanResponse {
    /// Version of the scan response protocol.
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    /// Root marker for every portable path in the response.
    pub root: String,
    /// Number of traversed directories.
    pub directory_count: usize,
    /// Traversed directories ordered by portable path.
    pub directories: Vec<ScannedDirectory>,
    /// Untraversed symbolic links ordered by portable path.
    pub symlinks: Vec<ScanSymlink>,
    /// Deterministic naming-style counts.
    pub naming: ScanNaming,
    /// Recurring filenames ordered by count descending and name ascending.
    pub recurring_filenames: Vec<RecurringFilename>,
    /// Aggregate convention-file coverage.
    pub norm_coverage: NormCoverage,
}

impl ScanResponse {
    /// Aggregate explicit traversal observations into the versioned response.
    #[must_use]
    pub fn new(
        mut observations: Vec<DirectoryObservation>,
        mut symlinks: Vec<ScanSymlink>,
    ) -> Self {
        observations.sort_by(|left, right| left.path.cmp(&right.path));
        symlinks.sort_by(|left, right| left.path.cmp(&right.path));

        let mut directory_naming = BTreeMap::new();
        let mut file_naming = BTreeMap::new();
        let mut filename_counts = BTreeMap::new();
        let mut directories = Vec::with_capacity(observations.len());
        let mut dirs_with_norm: usize = 0;

        for observation in observations {
            if observation.path != "."
                && let Some(name) = portable_file_name(&observation.path)
            {
                increment(&mut directory_naming, classify_name(name));
            }
            let has_norm = observation.file_names.iter().any(|name| name == ".norm");
            if has_norm {
                dirs_with_norm += 1;
            }
            let visible_files = observation
                .file_names
                .into_iter()
                .filter(|name| !name.starts_with('.'))
                .collect::<BTreeSet<_>>();
            for name in &visible_files {
                increment(&mut file_naming, classify_name(name));
                increment(&mut filename_counts, name.clone());
            }
            directories.push(ScannedDirectory {
                depth: portable_depth(&observation.path),
                path: observation.path,
                file_count: visible_files.len(),
                has_norm,
            });
        }

        for symlink in &symlinks {
            if symlink.kind == ScanSymlinkKind::Directory
                && let Some(name) = portable_file_name(&symlink.path)
            {
                increment(&mut directory_naming, classify_name(name));
            }
        }

        let mut recurring_filenames = filename_counts
            .into_iter()
            .filter_map(|(name, dir_count)| {
                (dir_count >= 2).then_some(RecurringFilename { name, dir_count })
            })
            .collect::<Vec<_>>();
        recurring_filenames.sort_by(|left, right| {
            right
                .dir_count
                .cmp(&left.dir_count)
                .then_with(|| left.name.cmp(&right.name))
        });
        recurring_filenames.truncate(RECURRING_FILENAME_LIMIT);

        let total_dirs = directories.len();
        let thousandths = dirs_with_norm
            .saturating_mul(1000)
            .saturating_add(total_dirs / 2)
            .checked_div(total_dirs)
            .unwrap_or(0);
        let ratio = f64::from(u32::try_from(thousandths).unwrap_or(1000)) / 1000.0;
        Self {
            api_version: SCAN_API_VERSION.to_owned(),
            root: ".".to_owned(),
            directory_count: total_dirs,
            directories,
            symlinks,
            naming: ScanNaming {
                directories: directory_naming,
                files: file_naming,
            },
            recurring_filenames,
            norm_coverage: NormCoverage {
                total_dirs,
                dirs_with_norm,
                ratio,
            },
        }
    }
}

/// Return whether a directory is infrastructure excluded from structural scan.
#[must_use]
pub fn is_ignored_directory(name: &str) -> bool {
    matches!(
        name,
        ".git"
            | "__pycache__"
            | ".venv"
            | "venv"
            | "node_modules"
            | ".mypy_cache"
            | ".pytest_cache"
            | ".idea"
            | ".vscode"
            | "dist"
            | "build"
            | "target"
    )
}

/// Classify one directory name or filename into a deterministic style bucket.
#[must_use]
pub fn classify_name(name: &str) -> String {
    let stem = if name.starts_with('.') {
        name
    } else {
        name.rsplit_once('.').map_or(name, |(stem, _)| stem)
    };
    if has_date_prefix(name) {
        "date_prefix"
    } else if is_delimited_ascii(stem, '-') {
        "kebab-case"
    } else if is_delimited_ascii(stem, '_') {
        "snake_case"
    } else if stem.is_empty() {
        "mixed"
    } else {
        let has_upper = stem.chars().any(char::is_uppercase);
        let has_lower = stem.chars().any(char::is_lowercase);
        let alphanumeric = stem.chars().all(char::is_alphanumeric);
        let upper_with_underscores = stem
            .chars()
            .filter(|character| *character != '_')
            .all(char::is_alphanumeric);
        let first = stem.chars().next();
        if has_upper && !has_lower && upper_with_underscores {
            "UPPER"
        } else if first.is_some_and(char::is_uppercase) && has_lower && alphanumeric {
            "PascalCase"
        } else if first.is_some_and(char::is_lowercase) && has_upper && alphanumeric {
            "camelCase"
        } else if has_lower && !has_upper && alphanumeric {
            "lowercase"
        } else {
            "mixed"
        }
    }
    .to_owned()
}

fn increment(counts: &mut BTreeMap<String, usize>, key: String) {
    *counts.entry(key).or_default() += 1;
}

fn portable_file_name(path: &str) -> Option<&str> {
    path.rsplit('/').find(|component| !component.is_empty())
}

fn portable_depth(path: &str) -> usize {
    if path == "." {
        0
    } else {
        path.split('/')
            .filter(|component| !component.is_empty())
            .count()
    }
}

fn is_delimited_ascii(value: &str, delimiter: char) -> bool {
    value.contains(delimiter)
        && value.split(delimiter).all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
        })
}

fn has_date_prefix(name: &str) -> bool {
    let bytes = name.as_bytes();
    let long = bytes.len() >= 11
        && bytes[..4].iter().all(u8::is_ascii_digit)
        && bytes.get(4) == Some(&b'-')
        && bytes
            .get(5..7)
            .is_some_and(|part| part.iter().all(u8::is_ascii_digit))
        && bytes.get(7) == Some(&b'-')
        && bytes
            .get(8..10)
            .is_some_and(|part| part.iter().all(u8::is_ascii_digit))
        && matches!(bytes.get(10), Some(b'-' | b'_'));
    let compact = bytes.len() >= 9
        && bytes[..8].iter().all(u8::is_ascii_digit)
        && matches!(bytes.get(8), Some(b'-' | b'_'));
    long || compact
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{
        DirectoryObservation, ScanResponse, ScanSymlink, ScanSymlinkKind, classify_name,
        is_ignored_directory,
    };

    #[test]
    fn naming_classification_covers_every_declared_bucket() {
        for (name, expected) in [
            ("2026-08-11-release.md", "date_prefix"),
            ("my-module", "kebab-case"),
            ("my_module.py", "snake_case"),
            ("README.md", "UPPER"),
            ("ModuleName", "PascalCase"),
            ("moduleName", "camelCase"),
            ("module.rs", "lowercase"),
            ("module.name.rs", "mixed"),
        ] {
            assert_eq!(classify_name(name), expected);
        }
    }

    #[test]
    fn infrastructure_policy_is_explicit_and_bounded() {
        for ignored in [".git", "node_modules", "__pycache__", "build", "target"] {
            assert!(is_ignored_directory(ignored));
        }
        assert!(!is_ignored_directory("docs"));
    }

    #[test]
    fn response_aggregates_order_naming_recurrence_and_coverage() {
        let response = ScanResponse::new(
            vec![
                DirectoryObservation::new("src", ["README.md", "lib.rs"]),
                DirectoryObservation::new(".", ["README.md"]),
                DirectoryObservation::new("docs", ["guide.md", ".norm"]),
            ],
            vec![ScanSymlink::new("linked-docs", ScanSymlinkKind::Directory)],
        );
        assert_eq!(response.directory_count, 3);
        assert_eq!(response.directories[0].path, ".");
        assert_eq!(response.directories[1].path, "docs");
        assert_eq!(response.directories[1].file_count, 1);
        assert!(response.directories[1].has_norm);
        assert!((response.norm_coverage.ratio - 0.333).abs() < f64::EPSILON);
        assert_eq!(response.recurring_filenames[0].name, "README.md");
        assert_eq!(response.recurring_filenames[0].dir_count, 2);
        assert_eq!(
            response.naming.directories,
            BTreeMap::from([("kebab-case".to_owned(), 1), ("lowercase".to_owned(), 2)])
        );
        assert_eq!(response.naming.files.get("UPPER"), Some(&2));
        assert_eq!(response.symlinks[0].path, "linked-docs");
    }
}
