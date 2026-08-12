//! Exact contract-bundle verification.

use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
};

use norm_spec_core::{
    A1_CLI_CASE_COUNT, A1_CLI_CONTRACT_DIGEST, A1_CLI_SUITE_ID, CONTRACT_BUNDLE_API_VERSION,
};
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BundleLock {
    #[serde(rename = "apiVersion")]
    api_version: String,
    suite: String,
    case_count: usize,
    contract_digest: String,
    files: Vec<LockedFile>,
}

#[derive(Debug, Deserialize)]
struct LockedFile {
    path: String,
    sha256: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum BundleErrorKind {
    Unavailable,
    Mismatch,
    UnsafePath,
}

pub(super) struct BundleError {
    pub(super) kind: BundleErrorKind,
    pub(super) message: String,
}

pub(super) struct VerifiedBundle {
    pub(super) root: PathBuf,
}

pub(super) fn verify(path: &Path) -> Result<VerifiedBundle, BundleError> {
    let root =
        fs::canonicalize(path).map_err(|_| unavailable("The contract bundle is unavailable."))?;
    if !root.is_dir() {
        return Err(unavailable("The contract bundle is not a directory."));
    }
    let lock_bytes = fs::read(root.join("bundle.lock.json"))
        .map_err(|_| unavailable("The contract bundle lock is unavailable."))?;
    let lock: BundleLock = serde_json::from_slice(&lock_bytes)
        .map_err(|_| mismatch("The contract bundle lock is invalid."))?;

    if lock.api_version != CONTRACT_BUNDLE_API_VERSION
        || lock.suite != A1_CLI_SUITE_ID
        || lock.case_count != A1_CLI_CASE_COUNT
        || lock.contract_digest != A1_CLI_CONTRACT_DIGEST
    {
        return Err(mismatch(
            "The contract bundle identity does not match the compiled suite.",
        ));
    }

    let mut previous: Option<&str> = None;
    let mut expected = BTreeSet::new();
    let mut identity = Sha256::new();
    identity.update(b"norm-spec/contract-digest/v1\n");
    identity.update(format!("suite={}\n", lock.suite));
    identity.update(format!("cases={}\n", lock.case_count));

    for file in &lock.files {
        if !is_safe_relative_path(&file.path) {
            return Err(unsafe_path(
                "The contract bundle lock contains an unsafe path.",
            ));
        }
        if previous.is_some_and(|value| value >= file.path.as_str()) {
            return Err(mismatch(
                "The contract bundle inventory is not uniquely sorted.",
            ));
        }
        previous = Some(&file.path);
        expected.insert(file.path.clone());

        let bytes = fs::read(join_portable(&root, &file.path))
            .map_err(|_| mismatch("A locked contract bundle file is unavailable."))?;
        let actual = hex_digest(&bytes);
        if actual != file.sha256 {
            return Err(mismatch("A locked contract bundle file has changed."));
        }
        identity.update(file.path.as_bytes());
        identity.update([0]);
        identity.update(file.sha256.as_bytes());
        identity.update(b"\n");
    }

    let computed = format!("sha256:{}", hex_bytes(&identity.finalize()));
    if computed != lock.contract_digest {
        return Err(mismatch(
            "The contract bundle digest does not match its inventory.",
        ));
    }

    expected.insert("bundle.lock.json".to_owned());
    let actual = enumerate_files(&root)?;
    if actual != expected {
        return Err(mismatch(
            "The contract bundle contains missing or unlisted files.",
        ));
    }
    Ok(VerifiedBundle { root })
}

fn enumerate_files(root: &Path) -> Result<BTreeSet<String>, BundleError> {
    let mut files = BTreeSet::new();
    enumerate_directory(root, root, &mut files)?;
    Ok(files)
}

fn enumerate_directory(
    root: &Path,
    directory: &Path,
    files: &mut BTreeSet<String>,
) -> Result<(), BundleError> {
    let entries = fs::read_dir(directory)
        .map_err(|_| unavailable("The contract bundle could not be enumerated."))?;
    for entry in entries {
        let entry = entry.map_err(|_| unavailable("A contract bundle entry is unavailable."))?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|_| unavailable("A contract bundle entry is unavailable."))?;
        if metadata.file_type().is_symlink() {
            return Err(unsafe_path("The contract bundle contains a symbolic link."));
        }
        if metadata.is_dir() {
            enumerate_directory(root, &path, files)?;
        } else if metadata.is_file() {
            let relative = path
                .strip_prefix(root)
                .map_err(|_| unsafe_path("A contract bundle entry escaped its root."))?;
            let portable = relative
                .components()
                .map(|component| match component {
                    Component::Normal(value) => Ok(value.to_string_lossy().into_owned()),
                    _ => Err(unsafe_path("A contract bundle entry has an unsafe path.")),
                })
                .collect::<Result<Vec<_>, _>>()?
                .join("/");
            files.insert(portable);
        } else {
            return Err(unsafe_path(
                "The contract bundle contains an unsupported entry.",
            ));
        }
    }
    Ok(())
}

fn is_safe_relative_path(value: &str) -> bool {
    if value.is_empty() || value.contains('\\') || value.contains("//") {
        return false;
    }
    let path = Path::new(value);
    !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._-/".contains(&byte))
}

pub(super) fn join_portable(root: &Path, value: &str) -> PathBuf {
    value
        .split('/')
        .fold(root.to_path_buf(), |path, component| path.join(component))
}

fn hex_digest(bytes: &[u8]) -> String {
    hex_bytes(&Sha256::digest(bytes))
}

fn hex_bytes(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut value = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(value, "{byte:02x}");
    }
    value
}

fn unavailable(message: &str) -> BundleError {
    BundleError {
        kind: BundleErrorKind::Unavailable,
        message: message.to_owned(),
    }
}

fn mismatch(message: &str) -> BundleError {
    BundleError {
        kind: BundleErrorKind::Mismatch,
        message: message.to_owned(),
    }
}

fn unsafe_path(message: &str) -> BundleError {
    BundleError {
        kind: BundleErrorKind::UnsafePath,
        message: message.to_owned(),
    }
}
