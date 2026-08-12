//! Frozen manifest execution against an explicit candidate.

use std::{
    fs, io,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;

use super::{
    bundle::join_portable,
    model::{CaseFailure, ConformanceIssue},
};

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct ContractCase {
    id: String,
    command: String,
    args_json: String,
    fixture: String,
    stdout: String,
    stdout_match: String,
    stderr: String,
    stderr_match: String,
    api_version: String,
    exit_code: i32,
}

struct IsolatedRoot {
    path: PathBuf,
    outside: PathBuf,
}

impl IsolatedRoot {
    fn new(case_id: &str) -> io::Result<Self> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|error| error.duration())
            .as_nanos();
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let prefix = format!(
            "norm-spec-conformance-{}-{timestamp}-{sequence}-{case_id}",
            std::process::id()
        );
        let path = std::env::temp_dir().join(&prefix);
        let outside = std::env::temp_dir().join(format!("{prefix}-outside"));
        fs::create_dir(&path)?;
        fs::create_dir(&outside)?;
        Ok(Self {
            path: fs::canonicalize(path)?,
            outside: fs::canonicalize(outside)?,
        })
    }
}

impl Drop for IsolatedRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
        let _ = fs::remove_dir_all(&self.outside);
    }
}

pub(super) struct SuiteOutcome {
    pub(super) executed: usize,
    pub(super) failures: Vec<CaseFailure>,
    pub(super) issue: Option<ConformanceIssue>,
}

pub(super) fn execute(candidate: &Path, bundle: &Path, version: Option<&str>) -> SuiteOutcome {
    let cases = match read_cases(bundle) {
        Ok(cases) => cases,
        Err(message) => return setup_failure(0, message),
    };
    let mut failures = Vec::new();
    let mut executed = 0;

    for case in &cases {
        let Ok(isolated) = IsolatedRoot::new(&case.id) else {
            return setup_failure_with(executed, failures, &case.id);
        };
        let Ok(fixture) = materialize_fixture(bundle, case, &isolated) else {
            return setup_failure_with(executed, failures, &case.id);
        };
        let Ok(arguments) = arguments(case, &isolated, &fixture, bundle, version) else {
            return setup_failure_with(executed, failures, &case.id);
        };
        let Ok(expected_stdout) = expected_stream(bundle, case, true, &isolated, &fixture, version)
        else {
            return setup_failure_with(executed, failures, &case.id);
        };
        let Ok(expected_stderr) =
            expected_stream(bundle, case, false, &isolated, &fixture, version)
        else {
            return setup_failure_with(executed, failures, &case.id);
        };
        let Ok(output) = run_case(candidate, case, &isolated, &arguments) else {
            return SuiteOutcome {
                executed,
                failures,
                issue: Some(ConformanceIssue::new(
                    "norm/conformance/candidate-unavailable",
                    format!("The candidate became unavailable before case {}.", case.id),
                )),
            };
        };
        executed += 1;
        let checks = compare_case(
            case,
            &output,
            expected_stdout.as_deref(),
            expected_stderr.as_deref(),
        );
        if !checks.is_empty() {
            failures.push(CaseFailure {
                case_id: case.id.clone(),
                checks,
            });
        }
    }

    SuiteOutcome {
        executed,
        failures,
        issue: None,
    }
}

fn read_cases(bundle: &Path) -> Result<Vec<ContractCase>, String> {
    let manifest = fs::read_to_string(bundle.join("manifest.tsv"))
        .map_err(|_| "The verified contract manifest became unavailable.".to_owned())?;
    let mut cases = Vec::new();
    for line in manifest
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let columns: Vec<_> = line.split('\t').collect();
        if columns.len() != 11 {
            return Err("The verified contract manifest has an invalid row.".to_owned());
        }
        if !matches!(
            columns[1],
            "global" | "parse" | "collect" | "validate" | "init" | "scan"
        ) {
            continue;
        }
        let exit_code = columns[9]
            .parse()
            .map_err(|_| "The verified contract manifest has an invalid exit code.".to_owned())?;
        cases.push(ContractCase {
            id: columns[0].to_owned(),
            command: columns[1].to_owned(),
            args_json: columns[2].to_owned(),
            fixture: columns[3].to_owned(),
            stdout: columns[4].to_owned(),
            stdout_match: columns[5].to_owned(),
            stderr: columns[6].to_owned(),
            stderr_match: columns[7].to_owned(),
            api_version: columns[8].to_owned(),
            exit_code,
        });
    }
    if cases.len() != norm_spec_core::A1_CLI_CASE_COUNT {
        return Err("The verified contract manifest has the wrong case count.".to_owned());
    }
    Ok(cases)
}

fn materialize_fixture(
    bundle: &Path,
    case: &ContractCase,
    isolated: &IsolatedRoot,
) -> io::Result<PathBuf> {
    if case.fixture == "-" {
        return Ok(isolated.path.join(".norm"));
    }
    if case.fixture.starts_with("layouts/") {
        materialize_layout(bundle, &case.fixture, isolated)?;
        return Ok(isolated.path.join(".norm"));
    }
    let source = join_portable(bundle, &case.fixture);
    if source.is_dir() {
        copy_fixture_directory(&source, &isolated.path)?;
        return Ok(isolated.path.clone());
    }
    let destination = isolated.path.join(".norm");
    fs::copy(source, &destination)?;
    Ok(destination)
}

fn copy_fixture_directory(source: &Path, destination: &Path) -> io::Result<()> {
    let mut entries = fs::read_dir(source)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            fs::create_dir(&destination_path)?;
            copy_fixture_directory(&source_path, &destination_path)?;
        } else if file_type.is_file() {
            fs::copy(source_path, destination_path)?;
        } else {
            return Err(io::Error::other("unsupported fixture entry"));
        }
    }
    Ok(())
}

fn materialize_layout(bundle: &Path, layout: &str, isolated: &IsolatedRoot) -> io::Result<()> {
    let recipe = fs::read_to_string(join_portable(bundle, layout))?;
    for line in recipe
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let columns: Vec<_> = line.split('\t').collect();
        if columns.len() != 3 {
            return Err(io::Error::other("invalid layout row"));
        }
        let destination = isolated.path.join(columns[1]);
        match columns[0] {
            "dir" => fs::create_dir_all(destination)?,
            "file" => {
                create_parent(&destination)?;
                fs::write(destination, columns[2])?;
            }
            "copy" => {
                create_parent(&destination)?;
                fs::copy(join_portable(bundle, columns[2]), destination)?;
            }
            "outside-dir" if columns[2] == "-" => {}
            "symlink-file" => {
                create_parent(&destination)?;
                create_file_symlink(Path::new(columns[2]), &destination)?;
            }
            "symlink-dir" => {
                create_parent(&destination)?;
                let target = columns[2].replace("{outside}", &isolated.outside.to_string_lossy());
                create_dir_symlink(Path::new(&target), &destination)?;
            }
            _ => return Err(io::Error::other("unsupported layout instruction")),
        }
    }
    Ok(())
}

fn create_parent(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

#[cfg(unix)]
fn create_file_symlink(target: &Path, link: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_file_symlink(target: &Path, link: &Path) -> io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
}

#[cfg(unix)]
fn create_dir_symlink(target: &Path, link: &Path) -> io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_dir_symlink(target: &Path, link: &Path) -> io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}

fn arguments(
    case: &ContractCase,
    isolated: &IsolatedRoot,
    fixture: &Path,
    bundle: &Path,
    version: Option<&str>,
) -> Result<Vec<String>, serde_json::Error> {
    let arguments: Vec<String> = serde_json::from_str(&case.args_json)?;
    Ok(arguments
        .iter()
        .map(|argument| replace_argument(argument, isolated, fixture, bundle, version))
        .collect())
}

fn replace_argument(
    value: &str,
    isolated: &IsolatedRoot,
    fixture: &Path,
    bundle: &Path,
    version: Option<&str>,
) -> String {
    let missing = isolated.path.join("missing");
    let output = isolated.path.join(".norm");
    for (placeholder, base) in [
        ("{fixture}", fixture),
        ("{root}", isolated.path.as_path()),
        ("{outside}", isolated.outside.as_path()),
        ("{missing}", missing.as_path()),
        ("{schema}", bundle.join("schema").as_path()),
        ("{output}", output.as_path()),
    ] {
        if let Some(path) = replace_path_placeholder(value, placeholder, base) {
            return path;
        }
    }
    value.replace(
        "{version}",
        version.unwrap_or("{candidate-version-unavailable}"),
    )
}

fn replace_path_placeholder(value: &str, placeholder: &str, base: &Path) -> Option<String> {
    if value == placeholder {
        return Some(base.to_string_lossy().into_owned());
    }
    let suffix = value.strip_prefix(placeholder)?.strip_prefix('/')?;
    let mut path = base.to_path_buf();
    for component in suffix.split('/').filter(|component| !component.is_empty()) {
        path.push(component);
    }
    Some(path.to_string_lossy().into_owned())
}

fn expected_stream(
    bundle: &Path,
    case: &ContractCase,
    stdout: bool,
    isolated: &IsolatedRoot,
    fixture: &Path,
    version: Option<&str>,
) -> io::Result<Option<String>> {
    let (path, mode) = if stdout {
        (&case.stdout, &case.stdout_match)
    } else {
        (&case.stderr, &case.stderr_match)
    };
    if mode == "empty" {
        return Ok(None);
    }
    let value = fs::read_to_string(join_portable(bundle, path))?;
    let outside = isolated.outside.to_string_lossy().replace('\\', "/");
    let fixture_display = display_relative_to_root(fixture, isolated);
    let output = expected_output_path(case, isolated, fixture, bundle, version);
    Ok(Some(
        value
            .replace("{fixture}", &fixture_display)
            .replace("{root}", ".")
            .replace("{outside}", &outside)
            .replace("{output}", &output)
            .replace("{missing}", "missing")
            .replace(
                "{version}",
                version.unwrap_or("{candidate-version-unavailable}"),
            ),
    ))
}

fn expected_output_path(
    case: &ContractCase,
    isolated: &IsolatedRoot,
    fixture: &Path,
    bundle: &Path,
    version: Option<&str>,
) -> String {
    let arguments: Vec<String> = serde_json::from_str(&case.args_json).unwrap_or_default();
    let raw = arguments
        .windows(2)
        .find_map(|pair| (pair[0] == "--output").then_some(pair[1].as_str()))
        .unwrap_or(".norm");
    let expanded = replace_argument(raw, isolated, fixture, bundle, version);
    display_relative_to_root(Path::new(&expanded), isolated)
}

fn display_relative_to_root(path: &Path, isolated: &IsolatedRoot) -> String {
    path.strip_prefix(&isolated.path).map_or_else(
        |_| path.to_string_lossy().replace('\\', "/"),
        norm_spec_core::project_path,
    )
}

fn run_case(
    candidate: &Path,
    case: &ContractCase,
    isolated: &IsolatedRoot,
    arguments: &[String],
) -> io::Result<Output> {
    let mut command = Command::new(candidate);
    command.current_dir(&isolated.path);
    if case.command != "global" {
        command.arg(&case.command);
    }
    command.args(arguments).output()
}

fn compare_case(
    case: &ContractCase,
    output: &Output,
    expected_stdout: Option<&str>,
    expected_stderr: Option<&str>,
) -> Vec<String> {
    let mut checks = Vec::new();
    if output.status.code().is_none() {
        checks.push("execution".to_owned());
    } else if output.status.code() != Some(case.exit_code) {
        checks.push("exit".to_owned());
    }
    let stdout = std::str::from_utf8(&output.stdout).ok();
    let stderr = std::str::from_utf8(&output.stderr).ok();
    if stdout.is_none_or(|actual| !stream_matches(actual, expected_stdout, &case.stdout_match)) {
        checks.push("stdout".to_owned());
    }
    if stderr.is_none_or(|actual| !stream_matches(actual, expected_stderr, &case.stderr_match)) {
        checks.push("stderr".to_owned());
    }
    if case.api_version != "-"
        && stdout.is_none_or(|actual| !machine_protocol_matches(actual, &case.api_version))
    {
        checks.push("apiVersion".to_owned());
    }
    checks
}

fn stream_matches(actual: &str, expected: Option<&str>, mode: &str) -> bool {
    match mode {
        "empty" => actual.is_empty(),
        "exact" | "template" => expected.is_some_and(|value| actual.trim_end() == value.trim_end()),
        "contains" => expected.is_some_and(|value| contains_in_order(actual, value)),
        "json-exact" => expected.is_some_and(|value| {
            parse_json(actual)
                .zip(parse_json(value))
                .is_some_and(|(actual, expected)| actual == expected)
        }),
        "json-subset" => expected.is_some_and(|value| {
            parse_json(actual)
                .zip(parse_json(value))
                .is_some_and(|(actual, expected)| is_json_subset(&expected, &actual))
        }),
        _ => false,
    }
}

fn contains_in_order(actual: &str, expected: &str) -> bool {
    let mut remaining = actual;
    for required in expected.lines().filter(|line| !line.is_empty()) {
        let Some(index) = remaining.find(required) else {
            return false;
        };
        remaining = &remaining[index + required.len()..];
    }
    true
}

fn parse_json(value: &str) -> Option<Value> {
    serde_json::from_str(value).ok()
}

fn is_json_subset(expected: &Value, actual: &Value) -> bool {
    match (expected, actual) {
        (Value::Object(expected), Value::Object(actual)) => expected.iter().all(|(key, value)| {
            actual
                .get(key)
                .is_some_and(|actual| is_json_subset(value, actual))
        }),
        (Value::Array(expected), Value::Array(actual)) => {
            let mut actual = actual.iter();
            expected.iter().all(|expected| {
                actual
                    .by_ref()
                    .any(|actual| is_json_subset(expected, actual))
            })
        }
        _ => expected == actual,
    }
}

fn machine_protocol_matches(stdout: &str, expected: &str) -> bool {
    let Some(value) = parse_json(stdout) else {
        return false;
    };
    if value.get("apiVersion").and_then(Value::as_str) != Some(expected) {
        return false;
    }
    if expected == norm_spec_core::ERROR_API_VERSION {
        return value
            .pointer("/error/message")
            .and_then(Value::as_str)
            .is_some_and(|message| !message.trim().is_empty());
    }
    true
}

fn setup_failure(executed: usize, message: String) -> SuiteOutcome {
    SuiteOutcome {
        executed,
        failures: Vec::new(),
        issue: Some(ConformanceIssue::new(
            "norm/conformance/case-setup",
            message,
        )),
    }
}

fn setup_failure_with(executed: usize, failures: Vec<CaseFailure>, case_id: &str) -> SuiteOutcome {
    SuiteOutcome {
        executed,
        failures,
        issue: Some(ConformanceIssue::new(
            "norm/conformance/case-setup",
            format!("Case {case_id} could not be prepared."),
        )),
    }
}
