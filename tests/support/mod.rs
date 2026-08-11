use std::{
    fs, io,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

pub(crate) struct Fixture {
    pub(crate) path: PathBuf,
}

impl Fixture {
    pub(crate) fn new() -> io::Result<Self> {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(io::Error::other)?
            .as_nanos();
        let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "norm-spec-api-{}-{unique}-{sequence}",
            std::process::id()
        ));
        fs::create_dir_all(path.join("docs"))?;
        Ok(Self { path })
    }

    pub(crate) fn write(&self, relative: &str, contents: &str) -> io::Result<()> {
        let path = self.path.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, contents)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        match fs::remove_dir_all(&self.path) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => eprintln!("failed to remove API fixture: {error}"),
        }
    }
}

pub(crate) fn root_norm() -> &'static str {
    "---\nmetadata:\n  layer: root\n  scope: ./\n  version: \"1.0\"\n---\n\n# Root\n"
}

pub(crate) fn docs_norm() -> &'static str {
    "---\nmetadata:\n  layer: docs\n  scope: docs/\n  version: \"1.0\"\n---\n\n# Docs\n"
}

pub(crate) fn write_standard_fixture(fixture: &Fixture) {
    fixture
        .write(".norm", root_norm())
        .unwrap_or_else(|error| panic!("root convention should be written: {error}"));
    fixture
        .write("docs/.norm", docs_norm())
        .unwrap_or_else(|error| panic!("docs convention should be written: {error}"));
}
