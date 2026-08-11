//! Deterministic collection path rules independent of filesystem I/O.

use std::{
    error::Error,
    fmt,
    path::{Component, Path, PathBuf},
};

/// Failure to express a collection target relative to its project root.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollectPathError {
    /// The supplied path is absolute or contains a parent traversal.
    OutsideRoot,
}

impl fmt::Display for CollectPathError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("the collection target is not a contained relative path")
    }
}

impl Error for CollectPathError {}

/// Return candidate `.norm` paths from the most-specific directory to root.
///
/// A file target begins at its parent directory. A directory target begins at
/// the directory itself. The returned paths are relative to the project root.
///
/// # Errors
///
/// Returns [`CollectPathError::OutsideRoot`] when `target` is absolute or
/// contains a parent-directory component.
pub fn collect_candidate_paths(
    target: &Path,
    target_is_file: bool,
) -> Result<Vec<PathBuf>, CollectPathError> {
    let target = contained_relative_path(target)?;
    let mut directory = if target_is_file {
        target
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf()
    } else {
        target
    };
    let mut candidates = Vec::new();

    loop {
        candidates.push(if directory.as_os_str().is_empty() {
            PathBuf::from(".norm")
        } else {
            directory.join(".norm")
        });
        if !directory.pop() {
            break;
        }
    }

    Ok(candidates)
}

/// Render a root-relative project path with portable separators.
#[must_use]
pub fn project_path(path: &Path) -> String {
    let rendered = path.to_string_lossy().replace('\\', "/");
    if rendered.is_empty() {
        ".".to_owned()
    } else {
        rendered
    }
}

fn contained_relative_path(path: &Path) -> Result<PathBuf, CollectPathError> {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(component) => normalized.push(component),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(CollectPathError::OutsideRoot);
            }
        }
    }
    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::{CollectPathError, collect_candidate_paths, project_path};

    #[test]
    fn directory_candidates_are_most_specific_first() {
        assert_eq!(
            collect_candidate_paths(Path::new("docs/module"), false),
            Ok(vec![
                PathBuf::from("docs/module/.norm"),
                PathBuf::from("docs/.norm"),
                PathBuf::from(".norm"),
            ])
        );
    }

    #[test]
    fn file_candidates_begin_at_the_parent() {
        assert_eq!(
            collect_candidate_paths(Path::new("docs/module/file.txt"), true),
            Ok(vec![
                PathBuf::from("docs/module/.norm"),
                PathBuf::from("docs/.norm"),
                PathBuf::from(".norm"),
            ])
        );
    }

    #[test]
    fn root_target_has_one_candidate() {
        assert_eq!(
            collect_candidate_paths(Path::new(""), false),
            Ok(vec![PathBuf::from(".norm")])
        );
    }

    #[test]
    fn escaping_or_absolute_targets_are_rejected() {
        assert_eq!(
            collect_candidate_paths(Path::new("../outside"), false),
            Err(CollectPathError::OutsideRoot)
        );
        assert_eq!(
            collect_candidate_paths(Path::new("/outside"), false),
            Err(CollectPathError::OutsideRoot)
        );
    }

    #[test]
    fn project_paths_use_root_marker_and_portable_separators() {
        assert_eq!(project_path(Path::new("")), ".");
        assert_eq!(
            project_path(Path::new("docs/module/.norm")),
            "docs/module/.norm"
        );
    }
}
