//! Structural and executable contract for the canonical norm-spec Skill.

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};

use serde_json::Value;

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

const EXPECTED_FILES: [&str; 3] = [
    "SKILL.md",
    "references/authoring.md",
    "references/field-reference.md",
];

fn expected_files() -> Vec<String> {
    EXPECTED_FILES.map(str::to_owned).to_vec()
}

struct TemporaryRoot {
    path: PathBuf,
}

impl TemporaryRoot {
    fn new(label: &str) -> Self {
        for _ in 0..100 {
            let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "norm-spec-skill-{label}-{}-{sequence}",
                std::process::id()
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Self { path },
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => panic!("failed to create Skill temporary root: {error}"),
            }
        }
        panic!("failed to allocate a unique Skill temporary root")
    }
}

impl Drop for TemporaryRoot {
    fn drop(&mut self) {
        match fs::remove_dir_all(&self.path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => eprintln!("failed to remove Skill temporary root: {error}"),
        }
    }
}

fn skill_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills/norm-spec")
}

fn portable_relative(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or_else(|error| panic!("Skill file should stay under its root: {error}"))
        .to_string_lossy()
        .replace('\\', "/")
}

fn skill_files(root: &Path) -> Vec<String> {
    fn visit(root: &Path, directory: &Path, files: &mut Vec<String>) {
        let mut entries = fs::read_dir(directory)
            .unwrap_or_else(|error| panic!("Skill directory should be readable: {error}"))
            .map(|entry| {
                entry.unwrap_or_else(|error| panic!("Skill entry should be readable: {error}"))
            })
            .collect::<Vec<_>>();
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries {
            let file_type = entry
                .file_type()
                .unwrap_or_else(|error| panic!("Skill entry type should be readable: {error}"));
            assert!(!file_type.is_symlink(), "Skill must not contain symlinks");
            if file_type.is_dir() {
                visit(root, &entry.path(), files);
            } else if file_type.is_file() {
                files.push(portable_relative(&entry.path(), root));
            } else {
                panic!("Skill contains an unsupported filesystem entry");
            }
        }
    }

    let mut files = Vec::new();
    visit(root, root, &mut files);
    files.sort();
    files
}

fn markdown_links(contents: &str) -> Vec<&str> {
    let mut links = Vec::new();
    let mut remainder = contents;
    while let Some(start) = remainder.find("](") {
        let target_start = start + 2;
        let target_and_after = &remainder[target_start..];
        let Some(end) = target_and_after.find(')') else {
            panic!("Skill contains an unterminated Markdown link");
        };
        links.push(&target_and_after[..end]);
        remainder = &target_and_after[end + 1..];
    }
    links
}

fn assert_frontmatter(skill: &str) {
    let mut lines = skill.lines();
    assert_eq!(lines.next(), Some("---"));
    let frontmatter = lines
        .by_ref()
        .take_while(|line| *line != "---")
        .collect::<Vec<_>>();
    assert_eq!(
        frontmatter.len(),
        2,
        "Skill frontmatter must contain only name and description"
    );
    assert_eq!(frontmatter[0], "name: norm-spec");
    let description = frontmatter[1]
        .strip_prefix("description: ")
        .unwrap_or_else(|| panic!("Skill description should be one plain YAML line"));
    assert!(!description.is_empty());
    assert!(description.len() <= 1024);
    assert!(!description.contains(['<', '>']));
}

fn assert_content_is_product_neutral(root: &Path) {
    for relative in EXPECTED_FILES {
        let contents = fs::read_to_string(root.join(relative))
            .unwrap_or_else(|error| panic!("Skill file {relative} should be readable: {error}"));
        let lowercase = contents.to_ascii_lowercase();
        for forbidden in [
            "[todo",
            "todo:",
            "placeholder",
            "example.py",
            "/users/",
            "/home/",
            "c:\\users\\",
            "agents/openai.yaml",
        ] {
            assert!(
                !lowercase.contains(forbidden),
                "Skill file {relative} contains forbidden local or initializer text: {forbidden}"
            );
        }
        for permissive_fallback in [
            "if norm is unavailable, parse",
            "manually parse the yaml",
            "walk parent directories manually",
            "continue without validation",
            "return an empty ruleset",
            "you may skip a failed validation",
        ] {
            assert!(
                !lowercase.contains(permissive_fallback),
                "Skill permits a semantic fallback: {permissive_fallback}"
            );
        }
    }
}

fn copy_skill(source: &Path, destination: &Path) {
    for relative in EXPECTED_FILES {
        let target = destination.join(relative);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|error| {
                panic!("Skill target directory should be created: {error}")
            });
        }
        fs::copy(source.join(relative), &target)
            .unwrap_or_else(|error| panic!("Skill file should copy into isolation: {error}"));
    }
}

fn bash_examples(skill: &str) -> Vec<String> {
    let mut examples = Vec::new();
    let mut in_bash = false;
    for line in skill.lines() {
        if line == "```bash" {
            assert!(!in_bash, "nested bash fences are not supported");
            in_bash = true;
        } else if line == "```" && in_bash {
            in_bash = false;
        } else if in_bash && !line.trim().is_empty() {
            examples.push(line.to_owned());
        }
    }
    assert!(!in_bash, "Skill contains an unterminated bash fence");
    examples
}

fn write_project_fixture(root: &Path) {
    fs::create_dir_all(root.join("docs"))
        .unwrap_or_else(|error| panic!("Skill project docs should be created: {error}"));
    fs::write(
        root.join(".norm"),
        "---\nmetadata: {layer: project, scope: ./, version: \"1.0\"}\n---\n\n# Project\n",
    )
    .unwrap_or_else(|error| panic!("Skill project convention should be written: {error}"));
    fs::write(root.join("docs/conventions.md"), "# Conventions\n")
        .unwrap_or_else(|error| panic!("conventions fixture should be written: {error}"));
    fs::write(root.join("docs/status.md"), "# Status\n")
        .unwrap_or_else(|error| panic!("status fixture should be written: {error}"));
}

#[test]
fn canonical_skill_is_minimal_framework_neutral_and_self_contained() {
    let root = skill_root();
    assert!(root.is_dir(), "canonical Skill directory is missing");
    assert_eq!(skill_files(&root), expected_files());

    let skill = fs::read_to_string(root.join("SKILL.md"))
        .unwrap_or_else(|error| panic!("SKILL.md should be readable: {error}"));
    assert_frontmatter(&skill);
    assert!(skill.lines().count() < 500);
    for required in [
        "norm-spec/compatibility/v1",
        "stop the affected workflow",
        "never replace a",
        "failed command with manual YAML parsing",
        "does not promise automatic",
        "canonical Rust engine remains the only",
    ] {
        assert!(
            skill.contains(required),
            "Skill omitted boundary: {required}"
        );
    }

    let links = markdown_links(&skill);
    assert_eq!(
        links,
        ["references/authoring.md", "references/field-reference.md"]
    );
    for link in links {
        let path = Path::new(link);
        assert!(!path.is_absolute());
        assert!(
            !path
                .components()
                .any(|part| matches!(part, std::path::Component::ParentDir))
        );
        assert!(
            root.join(path).is_file(),
            "Skill reference is missing: {link}"
        );
    }
    assert_content_is_product_neutral(&root);

    let isolated = TemporaryRoot::new("isolated");
    let installed = isolated.path.join("norm-spec");
    copy_skill(&root, &installed);
    assert_eq!(skill_files(&installed), expected_files());
    let installed_skill = fs::read_to_string(installed.join("SKILL.md"))
        .unwrap_or_else(|error| panic!("isolated SKILL.md should be readable: {error}"));
    for link in markdown_links(&installed_skill) {
        assert!(installed.join(link).is_file());
    }
}

#[test]
fn every_documented_cli_example_executes_against_the_current_candidate() {
    let skill = fs::read_to_string(skill_root().join("SKILL.md"))
        .unwrap_or_else(|error| panic!("SKILL.md should be readable: {error}"));
    let examples = bash_examples(&skill);
    assert_eq!(
        examples,
        [
            "norm compatibility --pretty",
            "norm collect --root \"$NORM_ROOT\" --target \"$NORM_TARGET\" --pretty",
            "norm validate --all --root \"$NORM_ROOT\" --strict --json --pretty",
            "norm init --profile convention --output \"$NORM_ROOT/docs/.norm\" --json --pretty",
            "norm parse \"$NORM_FILE\" --pretty",
            "norm validate \"$NORM_FILE\" --root \"$NORM_ROOT\" --strict --json --pretty",
        ]
    );

    let fixture = TemporaryRoot::new("commands");
    let project = fixture.path.join("project");
    fs::create_dir(&project)
        .unwrap_or_else(|error| panic!("Skill project should be created: {error}"));
    write_project_fixture(&project);
    let norm_file = project.join("docs/.norm");
    let project_arg = project.to_string_lossy().into_owned();
    let target_arg = project.join("docs").to_string_lossy().into_owned();
    let norm_file_arg = norm_file.to_string_lossy().into_owned();

    let cases = [
        (
            vec!["compatibility", "--pretty"],
            "norm-spec/compatibility/v1",
        ),
        (
            vec![
                "collect",
                "--root",
                project_arg.as_str(),
                "--target",
                target_arg.as_str(),
                "--pretty",
            ],
            "norm-spec/collect/v1",
        ),
        (
            vec![
                "validate",
                "--all",
                "--root",
                project_arg.as_str(),
                "--strict",
                "--json",
                "--pretty",
            ],
            "norm-spec/validate/v1",
        ),
        (
            vec![
                "init",
                "--profile",
                "convention",
                "--output",
                norm_file_arg.as_str(),
                "--json",
                "--pretty",
            ],
            "norm-spec/init/v1",
        ),
        (
            vec!["parse", norm_file_arg.as_str(), "--pretty"],
            "norm-spec/parse/v1",
        ),
        (
            vec![
                "validate",
                norm_file_arg.as_str(),
                "--root",
                project_arg.as_str(),
                "--strict",
                "--json",
                "--pretty",
            ],
            "norm-spec/validate/v1",
        ),
    ];

    for (args, api_version) in cases {
        let output = Command::new(env!("CARGO_BIN_EXE_norm"))
            .args(args)
            .output()
            .unwrap_or_else(|error| panic!("documented command should execute: {error}"));
        assert_eq!(output.status.code(), Some(0), "documented command failed");
        assert!(
            output.stderr.is_empty(),
            "documented machine command wrote stderr"
        );
        let response: Value = serde_json::from_slice(&output.stdout)
            .unwrap_or_else(|error| panic!("documented command returned invalid JSON: {error}"));
        assert_eq!(
            response.get("apiVersion").and_then(Value::as_str),
            Some(api_version)
        );
    }
}
