//! Executable Gate C contract coverage through `validate`.

use std::{
    fs, io,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;

const EXECUTABLE_CASES: usize = 63;
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
        let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_nanos(),
            Err(error) => error.duration().as_nanos(),
        };
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let prefix = format!(
            "norm-spec-contract-{}-{timestamp}-{sequence}-{case_id}",
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
        let _result = fs::remove_dir_all(&self.path);
        let _result = fs::remove_dir_all(&self.outside);
    }
}

fn contract_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/contract")
}

fn read_required(path: &Path) -> String {
    match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) => panic!(
            "required contract file {} is unavailable: {error}",
            path.display()
        ),
    }
}

fn executable_cases(contract: &Path) -> Vec<ContractCase> {
    let manifest = read_required(&contract.join("manifest.tsv"));
    manifest
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| {
            let columns: Vec<_> = line.split('\t').collect();
            assert_eq!(columns.len(), 11, "invalid contract manifest row: {line}");
            if !matches!(columns[1], "global" | "parse" | "collect" | "validate") {
                return None;
            }
            let exit_code = match columns[9].parse() {
                Ok(exit_code) => exit_code,
                Err(error) => panic!("case {} has an invalid exit code: {error}", columns[0]),
            };
            Some(ContractCase {
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
            })
        })
        .collect()
}

fn materialize_fixture(contract: &Path, case: &ContractCase, isolated: &IsolatedRoot) {
    if case.fixture == "-" {
        return;
    }
    if case.fixture.starts_with("layouts/") {
        materialize_layout(contract, &case.fixture, isolated);
        return;
    }

    let source = contract.join(&case.fixture);
    let destination = isolated.path.join(".norm");
    if let Err(error) = fs::copy(&source, &destination) {
        panic!(
            "case {} could not copy {} to {}: {error}",
            case.id,
            source.display(),
            destination.display()
        );
    }
}

fn materialize_layout(contract: &Path, layout: &str, isolated: &IsolatedRoot) {
    let recipe = read_required(&contract.join(layout));
    for line in recipe
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let columns: Vec<_> = line.split('\t').collect();
        assert_eq!(columns.len(), 3, "invalid layout row in {layout}: {line}");
        let destination = isolated.path.join(columns[1]);
        match columns[0] {
            "dir" => create_directory(layout, &destination),
            "file" => {
                create_parent(layout, &destination);
                if let Err(error) = fs::write(&destination, columns[2]) {
                    panic!(
                        "layout {layout} could not write {}: {error}",
                        destination.display()
                    );
                }
            }
            "copy" => {
                create_parent(layout, &destination);
                let source = contract.join(columns[2]);
                if let Err(error) = fs::copy(&source, &destination) {
                    panic!(
                        "layout {layout} could not copy {} to {}: {error}",
                        source.display(),
                        destination.display()
                    );
                }
            }
            "outside-dir" => assert_eq!(
                columns[2], "-",
                "layout {layout} outside directory must not have content"
            ),
            "symlink-file" => {
                create_parent(layout, &destination);
                create_file_symlink(Path::new(columns[2]), &destination, layout);
            }
            "symlink-dir" => {
                create_parent(layout, &destination);
                let target = columns[2].replace("{outside}", &isolated.outside.to_string_lossy());
                create_dir_symlink(Path::new(&target), &destination, layout);
            }
            instruction => panic!("layout {layout} requires unsupported instruction {instruction}"),
        }
    }
}

fn create_directory(layout: &str, path: &Path) {
    if let Err(error) = fs::create_dir_all(path) {
        panic!(
            "layout {layout} could not create {}: {error}",
            path.display()
        );
    }
}

fn create_parent(layout: &str, path: &Path) {
    if let Some(parent) = path.parent()
        && let Err(error) = fs::create_dir_all(parent)
    {
        panic!(
            "layout {layout} could not create {}: {error}",
            parent.display()
        );
    }
}

#[cfg(unix)]
fn create_file_symlink(target: &Path, link: &Path, layout: &str) {
    if let Err(error) = std::os::unix::fs::symlink(target, link) {
        panic!(
            "layout {layout} could not create file symlink {}: {error}",
            link.display()
        );
    }
}

#[cfg(windows)]
fn create_file_symlink(target: &Path, link: &Path, layout: &str) {
    if let Err(error) = std::os::windows::fs::symlink_file(target, link) {
        panic!(
            "layout {layout} could not create file symlink {}: {error}",
            link.display()
        );
    }
}

#[cfg(unix)]
fn create_dir_symlink(target: &Path, link: &Path, layout: &str) {
    if let Err(error) = std::os::unix::fs::symlink(target, link) {
        panic!(
            "layout {layout} could not create directory symlink {}: {error}",
            link.display()
        );
    }
}

#[cfg(windows)]
fn create_dir_symlink(target: &Path, link: &Path, layout: &str) {
    if let Err(error) = std::os::windows::fs::symlink_dir(target, link) {
        panic!(
            "layout {layout} could not create directory symlink {}: {error}",
            link.display()
        );
    }
}

fn replace_argument_placeholders(value: &str, isolated: &IsolatedRoot) -> String {
    let root = isolated.path.to_string_lossy();
    let outside = isolated.outside.to_string_lossy();
    let fixture = isolated.path.join(".norm").to_string_lossy().into_owned();
    let missing = isolated.path.join("missing").to_string_lossy().into_owned();
    let schema = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../schema")
        .to_string_lossy()
        .into_owned();
    value
        .replace("{fixture}", &fixture)
        .replace("{root}", &root)
        .replace("{outside}", &outside)
        .replace("{missing}", &missing)
        .replace("{schema}", &schema)
        .replace("{version}", env!("CARGO_PKG_VERSION"))
}

fn replace_expected_placeholders(value: &str, isolated: &IsolatedRoot) -> String {
    let outside = isolated.outside.to_string_lossy().replace('\\', "/");
    value
        .replace("{fixture}", ".norm")
        .replace("{root}", ".")
        .replace("{outside}", &outside)
        .replace("{missing}", "missing")
        .replace("{version}", env!("CARGO_PKG_VERSION"))
}

fn run_case(case: &ContractCase, isolated: &IsolatedRoot) -> Output {
    let arguments: Vec<String> = match serde_json::from_str(&case.args_json) {
        Ok(arguments) => arguments,
        Err(error) => panic!("case {} has invalid args JSON: {error}", case.id),
    };
    let arguments: Vec<_> = arguments
        .iter()
        .map(|argument| replace_argument_placeholders(argument, isolated))
        .collect();

    let mut command = Command::new(env!("CARGO_BIN_EXE_norm"));
    command.current_dir(&isolated.path);
    if case.command != "global" {
        command.arg(&case.command);
    }
    match command.args(arguments).output() {
        Ok(output) => output,
        Err(error) => panic!("case {} failed to execute norm: {error}", case.id),
    }
}

fn utf8_stream<'a>(case: &ContractCase, name: &str, bytes: &'a [u8]) -> &'a str {
    match std::str::from_utf8(bytes) {
        Ok(stream) => stream,
        Err(error) => panic!("case {} produced non-UTF-8 {name}: {error}", case.id),
    }
}

fn assert_stream(
    contract: &Path,
    case: &ContractCase,
    stream_name: &str,
    actual: &str,
    expected_path: &str,
    match_mode: &str,
    isolated: &IsolatedRoot,
) {
    if match_mode == "empty" {
        assert_eq!(
            expected_path, "-",
            "case {} has inconsistent empty fixture",
            case.id
        );
        assert!(
            actual.is_empty(),
            "case {} produced unexpected {stream_name}: {actual}",
            case.id
        );
        return;
    }

    let expected =
        replace_expected_placeholders(&read_required(&contract.join(expected_path)), isolated);
    match match_mode {
        "exact" | "template" => assert_eq!(
            actual.trim_end(),
            expected.trim_end(),
            "case {} {stream_name} mismatch",
            case.id
        ),
        "contains" => assert_contains_in_order(&case.id, stream_name, actual, &expected),
        "json-exact" => {
            let actual = parse_json(&case.id, stream_name, actual);
            let expected = parse_json(&case.id, "expected output", &expected);
            assert_eq!(
                actual, expected,
                "case {} {stream_name} JSON mismatch",
                case.id
            );
        }
        "json-subset" => {
            let actual = parse_json(&case.id, stream_name, actual);
            let expected = parse_json(&case.id, "expected output", &expected);
            assert!(
                is_json_subset(&expected, &actual),
                "case {} {stream_name} JSON did not contain required subset\nexpected: {expected}\nactual: {actual}",
                case.id
            );
        }
        mode => panic!(
            "case {} uses unsupported executable match mode {mode}",
            case.id
        ),
    }
}

fn assert_contains_in_order(case_id: &str, stream_name: &str, actual: &str, expected: &str) {
    let mut remaining = actual;
    for required in expected.lines().filter(|line| !line.is_empty()) {
        let Some(index) = remaining.find(required) else {
            panic!("case {case_id} {stream_name} omitted ordered cue: {required}");
        };
        remaining = &remaining[index + required.len()..];
    }
}

fn parse_json(case_id: &str, stream_name: &str, value: &str) -> Value {
    match serde_json::from_str(value) {
        Ok(value) => value,
        Err(error) => panic!("case {case_id} produced invalid JSON in {stream_name}: {error}"),
    }
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

fn assert_machine_protocol(case: &ContractCase, stdout: &str) {
    if case.api_version == "-" {
        return;
    }
    let value = parse_json(&case.id, "stdout", stdout);
    assert_eq!(
        value.get("apiVersion").and_then(Value::as_str),
        Some(case.api_version.as_str()),
        "case {} emitted the wrong protocol version",
        case.id
    );
    if case.api_version == norm_spec_core::ERROR_API_VERSION {
        let message = value
            .get("error")
            .and_then(|error| error.get("message"))
            .and_then(Value::as_str);
        assert!(
            message.is_some_and(|message| !message.trim().is_empty()),
            "case {} emitted an empty machine diagnostic",
            case.id
        );
    }
}

#[test]
fn implemented_contract_cases_execute_without_skips() {
    let contract = contract_root();
    let cases = executable_cases(&contract);
    assert_eq!(
        cases.len(),
        EXECUTABLE_CASES,
        "the executable slice inventory changed"
    );

    for case in &cases {
        let root = match IsolatedRoot::new(&case.id) {
            Ok(root) => root,
            Err(error) => panic!(
                "case {} could not create an isolated root: {error}",
                case.id
            ),
        };
        materialize_fixture(&contract, case, &root);
        let output = run_case(case, &root);
        assert_eq!(
            output.status.code(),
            Some(case.exit_code),
            "case {} returned the wrong exit code",
            case.id
        );

        let stdout = utf8_stream(case, "stdout", &output.stdout);
        let stderr = utf8_stream(case, "stderr", &output.stderr);
        assert_stream(
            &contract,
            case,
            "stdout",
            stdout,
            &case.stdout,
            &case.stdout_match,
            &root,
        );
        assert_stream(
            &contract,
            case,
            "stderr",
            stderr,
            &case.stderr,
            &case.stderr_match,
            &root,
        );
        assert_machine_protocol(case, stdout);
    }
}
