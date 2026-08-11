use std::path::Path;

use norm_spec::{CollectRequest, ValidateRequest, collect, validate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("project");
    let collected = collect(CollectRequest::new(&root, Path::new("docs")))?;
    assert_eq!(collected.norms.len(), 2);
    assert_eq!(collected.norms[0].path, "docs/.norm");
    assert_eq!(collected.norms[1].path, ".norm");

    let validated = validate(ValidateRequest::all(&root))?;
    assert_eq!(validated.summary.files, 2);
    assert_eq!(validated.summary.errors, 0);
    assert_eq!(validated.summary.warnings, 0);
    println!("external collect+validate passed at an exact Git revision");
    Ok(())
}
