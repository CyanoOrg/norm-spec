//! Static integrity checks for the language-neutral Gate B contract inventory.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path},
};

use norm_spec_core::{
    COLLECT_API_VERSION, ERROR_API_VERSION, INIT_API_VERSION, PARSE_API_VERSION, SCAN_API_VERSION,
    VALIDATE_API_VERSION,
};

const COMMANDS: [&str; 5] = ["parse", "collect", "validate", "init", "scan"];
const MACHINE_PROTOCOLS: [&str; 6] = [
    PARSE_API_VERSION,
    COLLECT_API_VERSION,
    VALIDATE_API_VERSION,
    INIT_API_VERSION,
    SCAN_API_VERSION,
    ERROR_API_VERSION,
];
const MATCH_MODES: [&str; 6] = [
    "empty",
    "exact",
    "contains",
    "template",
    "json-exact",
    "json-subset",
];
const PLACEHOLDERS: [&str; 7] = [
    "fixture", "root", "outside", "schema", "output", "missing", "version",
];

fn read_required(path: &Path) -> String {
    match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) => panic!(
            "required contract file {} is unavailable: {error}",
            path.display()
        ),
    }
}

fn data_lines(contents: &str) -> impl Iterator<Item = (usize, &str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.is_empty() && !line.starts_with('#'))
        .map(|(index, line)| (index + 1, line))
}

fn split_columns(line: &str, line_number: usize, count: usize) -> Vec<&str> {
    let columns: Vec<_> = line.split('\t').collect();
    assert_eq!(
        columns.len(),
        count,
        "contract line {line_number} must have {count} tab-separated columns"
    );
    assert!(
        columns.iter().all(|column| !column.is_empty()),
        "contract line {line_number} contains an empty column; use '-' explicitly"
    );
    columns
}

fn assert_relative_contract_path(case_id: &str, value: &str) {
    let path = Path::new(value);
    assert!(
        !path.is_absolute()
            && path
                .components()
                .all(|component| !matches!(component, Component::ParentDir | Component::RootDir)),
        "case {case_id} has a non-portable contract path: {value}"
    );
}

fn assert_known_placeholders(context: &str, value: &str) {
    let mut remainder = value;
    while let Some(start) = remainder.find('{') {
        let after_open = &remainder[start + 1..];
        let Some(end) = after_open.find('}') else {
            break;
        };
        let placeholder = &after_open[..end];
        let is_placeholder = !placeholder.is_empty()
            && placeholder
                .chars()
                .all(|character| character.is_ascii_lowercase() || character == '-');
        if is_placeholder {
            assert!(
                PLACEHOLDERS.contains(&placeholder),
                "{context} uses unknown placeholder {{{placeholder}}}"
            );
            remainder = &after_open[end + 1..];
        } else {
            remainder = after_open;
        }
    }
}

fn is_json_string_array(value: &str) -> bool {
    let Some(inner) = value
        .strip_prefix('[')
        .and_then(|item| item.strip_suffix(']'))
    else {
        return false;
    };
    if inner.is_empty() {
        return true;
    }
    inner.split(',').all(|item| {
        item.len() >= 2
            && item.starts_with('"')
            && item.ends_with('"')
            && !item[1..item.len() - 1].contains('"')
    })
}

fn success_protocol(command: &str) -> Option<&'static str> {
    match command {
        "parse" => Some(PARSE_API_VERSION),
        "collect" => Some(COLLECT_API_VERSION),
        "validate" => Some(VALIDATE_API_VERSION),
        "init" => Some(INIT_API_VERSION),
        "scan" => Some(SCAN_API_VERSION),
        _ => None,
    }
}

fn command_flags(command: &str) -> &'static [&'static str] {
    match command {
        "global" => &["--version", "-V", "--help", "-h"],
        "parse" => &["--legacy-format", "--pretty"],
        "collect" => &["--root", "--target", "--legacy-format", "--pretty"],
        "validate" => &[
            "--all",
            "--root",
            "--schema-dir",
            "--profile",
            "--strict",
            "--legacy-format",
            "--compat-keys",
            "--json",
            "--pretty",
        ],
        "init" => &["--profile", "--output", "--force", "--json", "--pretty"],
        "scan" => &["--root", "--pretty", "--text"],
        _ => &[],
    }
}

fn check_stream(contract: &Path, case_id: &str, expected: &str, match_mode: &str, machine: bool) {
    assert!(
        MATCH_MODES.contains(&match_mode),
        "case {case_id} has unknown match mode {match_mode}"
    );
    if expected == "-" {
        assert_eq!(
            match_mode, "empty",
            "case {case_id} must use empty matching for an absent stream fixture"
        );
        return;
    }

    assert_relative_contract_path(case_id, expected);
    assert!(
        expected.starts_with("expected/"),
        "case {case_id} stream expectation must live under expected/"
    );
    let contents = read_required(&contract.join(expected));
    assert!(
        !contents.trim().is_empty(),
        "case {case_id} has an empty stream expectation"
    );
    assert_known_placeholders(&format!("case {case_id} expected stream"), &contents);

    if machine {
        assert!(
            matches!(match_mode, "json-exact" | "json-subset"),
            "case {case_id} machine stdout must use JSON matching"
        );
        assert!(
            contents.trim_start().starts_with('{') && contents.trim_end().ends_with('}'),
            "case {case_id} machine stdout expectation is not a JSON object"
        );
    } else {
        assert!(
            !match_mode.starts_with("json-"),
            "case {case_id} human stream cannot use JSON matching"
        );
    }
}

fn check_layout(contract: &Path, case_id: &str, layout: &str) {
    let contents = read_required(&contract.join(layout));
    for (line_number, line) in data_lines(&contents) {
        let columns = split_columns(line, line_number, 3);
        let kind = columns[0];
        let path = columns[1];
        let value = columns[2];
        assert!(
            [
                "dir",
                "file",
                "copy",
                "outside-dir",
                "symlink-dir",
                "symlink-file"
            ]
            .contains(&kind),
            "layout {layout} line {line_number} has unknown instruction {kind}"
        );
        assert_relative_contract_path(case_id, path);
        assert_known_placeholders(&format!("layout {layout} line {line_number}"), value);
        match kind {
            "dir" | "outside-dir" => assert_eq!(
                value, "-",
                "layout {layout} line {line_number} directory value must be '-'"
            ),
            "copy" => {
                assert_relative_contract_path(case_id, value);
                assert!(
                    contract.join(value).is_file(),
                    "layout {layout} line {line_number} copy source is unavailable: {value}"
                );
            }
            "file" | "symlink-dir" | "symlink-file" => assert_ne!(
                value, "-",
                "layout {layout} line {line_number} requires a value"
            ),
            _ => unreachable!("layout instruction was checked above"),
        }
    }
}

struct ContractCase<'a> {
    id: &'a str,
    command: &'a str,
    arguments: &'a str,
    fixture: &'a str,
    stdout: &'a str,
    stdout_match: &'a str,
    stderr: &'a str,
    stderr_match: &'a str,
    protocol: &'a str,
    exit_code: u8,
    covers: &'a str,
}

impl<'a> ContractCase<'a> {
    fn from_columns(columns: &[&'a str]) -> Self {
        let id = columns[0];
        let exit_code = columns[9]
            .parse::<u8>()
            .unwrap_or_else(|error| panic!("case {id} has an invalid exit code: {error}"));
        Self {
            id,
            command: columns[1],
            arguments: columns[2],
            fixture: columns[3],
            stdout: columns[4],
            stdout_match: columns[5],
            stderr: columns[6],
            stderr_match: columns[7],
            protocol: columns[8],
            exit_code,
            covers: columns[10],
        }
    }
}

#[derive(Default)]
struct ContractEvidence<'a> {
    coverage: BTreeMap<&'a str, u32>,
    case_ids: BTreeSet<&'a str>,
    protocols: BTreeSet<&'a str>,
    machine_success: BTreeSet<&'a str>,
    machine_errors: BTreeSet<&'a str>,
    human_modes: BTreeSet<&'a str>,
    observed_flags: BTreeSet<(&'a str, &'static str)>,
    expectations: BTreeSet<&'a str>,
}

fn load_requirements(contents: &str) -> BTreeSet<&str> {
    let mut requirements = BTreeSet::new();
    for (line_number, line) in data_lines(contents) {
        let columns = split_columns(line, line_number, 2);
        let requirement_id = columns[0];
        assert!(
            requirements.insert(requirement_id),
            "duplicate Gate B requirement ID: {requirement_id}"
        );
        assert_ne!(
            columns[1], "-",
            "requirement {requirement_id} needs a description"
        );
    }
    requirements
}

fn check_case_fixture(contract: &Path, case: &ContractCase<'_>) {
    if case.fixture == "-" {
        return;
    }
    assert_relative_contract_path(case.id, case.fixture);
    let fixture_path = contract.join(case.fixture);
    assert!(
        fixture_path.exists(),
        "case {} is missing fixture {}",
        case.id,
        case.fixture
    );
    if case.fixture.starts_with("layouts/") {
        check_layout(contract, case.id, case.fixture);
        if case.arguments.contains("{outside}") {
            let layout = read_required(&fixture_path);
            assert!(
                data_lines(&layout).any(|(_, line)| line.starts_with("outside-dir\t")),
                "case {} uses {{outside}} without an outside-dir layout",
                case.id
            );
        }
    }
}

fn check_case_protocol<'a>(
    contract: &Path,
    case: &ContractCase<'a>,
    evidence: &mut ContractEvidence<'a>,
) {
    if case.protocol == "-" {
        assert!(
            case.command == "global" || ["validate", "init", "scan"].contains(&case.command),
            "case {} declares unsupported human mode for {}",
            case.id,
            case.command
        );
        if case.command != "global" && case.exit_code == 0 {
            evidence.human_modes.insert(case.command);
        }
        return;
    }

    assert!(
        MACHINE_PROTOCOLS.contains(&case.protocol),
        "case {} names unknown protocol {}",
        case.id,
        case.protocol
    );
    assert_eq!(
        case.stderr, "-",
        "case {} machine mode must leave stderr empty",
        case.id
    );
    let expected = read_required(&contract.join(case.stdout));
    assert!(
        expected.contains(&format!("\"apiVersion\": \"{}\"", case.protocol)),
        "case {} expected stdout does not declare {}",
        case.id,
        case.protocol
    );
    evidence.protocols.insert(case.protocol);
    if case.protocol == ERROR_API_VERSION {
        assert_ne!(
            case.exit_code, 0,
            "case {} error envelope cannot succeed",
            case.id
        );
        evidence.machine_errors.insert(case.command);
    } else {
        assert_eq!(
            Some(case.protocol),
            success_protocol(case.command),
            "case {} protocol does not match command",
            case.id
        );
        assert!(
            case.exit_code == 0 || (case.command == "validate" && case.exit_code == 1),
            "case {} success protocol has invalid exit classification",
            case.id
        );
        evidence.machine_success.insert(case.command);
    }
}

fn inspect_manifest<'a>(
    contract: &Path,
    contents: &'a str,
    requirements: &BTreeSet<&str>,
) -> ContractEvidence<'a> {
    let mut evidence = ContractEvidence::default();
    for (line_number, line) in data_lines(contents) {
        let columns = split_columns(line, line_number, 11);
        let case = ContractCase::from_columns(&columns);
        assert!(
            evidence.case_ids.insert(case.id),
            "duplicate contract case ID: {}",
            case.id
        );
        assert!(
            case.command == "global" || COMMANDS.contains(&case.command),
            "case {} names unsupported command {}",
            case.id,
            case.command
        );
        assert!(
            is_json_string_array(case.arguments),
            "case {} arguments are not a JSON string array",
            case.id
        );
        assert_known_placeholders(&format!("case {} arguments", case.id), case.arguments);
        assert!(
            case.exit_code <= 2,
            "case {} has unsupported exit code {}",
            case.id,
            case.exit_code
        );
        for flag in command_flags(case.command) {
            if case.arguments.contains(&format!("\"{flag}\"")) {
                evidence.observed_flags.insert((case.command, *flag));
            }
        }
        check_case_fixture(contract, &case);
        let machine = case.protocol != "-";
        check_stream(contract, case.id, case.stdout, case.stdout_match, machine);
        check_stream(contract, case.id, case.stderr, case.stderr_match, false);
        evidence.expectations.extend(
            [case.stdout, case.stderr]
                .into_iter()
                .filter(|item| *item != "-"),
        );
        check_case_protocol(contract, &case, &mut evidence);
        for requirement_id in case.covers.split(',') {
            assert!(
                requirements.contains(requirement_id),
                "case {} covers unknown requirement {requirement_id}",
                case.id
            );
            *evidence.coverage.entry(requirement_id).or_default() += 1;
        }
    }
    evidence
}

fn assert_complete(requirements: &BTreeSet<&str>, evidence: &ContractEvidence<'_>) {
    assert!(
        !evidence.case_ids.is_empty(),
        "contract manifest must contain cases"
    );
    for requirement_id in requirements {
        assert!(
            evidence.coverage.contains_key(requirement_id),
            "Gate B requirement {requirement_id} has no manifest case"
        );
    }
    for protocol in MACHINE_PROTOCOLS {
        assert!(
            evidence.protocols.contains(protocol),
            "manifest is missing protocol {protocol}"
        );
    }
    for command in COMMANDS {
        assert!(
            evidence.machine_success.contains(command),
            "manifest is missing machine success coverage for {command}"
        );
        assert!(
            evidence.machine_errors.contains(command),
            "manifest is missing machine error coverage for {command}"
        );
    }
    for command in ["validate", "init", "scan"] {
        assert!(
            evidence.human_modes.contains(command),
            "manifest is missing human success coverage for {command}"
        );
    }
    for command in ["global", "parse", "collect", "validate", "init", "scan"] {
        for flag in command_flags(command) {
            assert!(
                evidence.observed_flags.contains(&(command, *flag)),
                "manifest has no case exercising {command} flag {flag}"
            );
        }
    }
}

fn assert_no_orphan_expectations(contract: &Path, referenced: &BTreeSet<&str>) {
    let expected_directory = contract.join("expected");
    let entries = fs::read_dir(&expected_directory).unwrap_or_else(|error| {
        panic!(
            "required contract directory {} is unavailable: {error}",
            expected_directory.display()
        )
    });
    for entry in entries {
        let entry = entry.unwrap_or_else(|error| panic!("cannot inspect expected asset: {error}"));
        assert!(
            entry.path().is_file(),
            "expected/ may contain only stream fixture files: {}",
            entry.path().display()
        );
        let asset = format!("expected/{}", entry.file_name().to_string_lossy());
        assert!(
            referenced.contains(asset.as_str()),
            "orphan contract expectation is not referenced: {asset}"
        );
    }
}

#[test]
fn contract_manifest_is_complete_and_self_contained() {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let contract = repository.join("tests/contract");
    let requirement_text = read_required(&contract.join("requirements.tsv"));
    let manifest_text = read_required(&contract.join("manifest.tsv"));
    let requirements = load_requirements(&requirement_text);
    let evidence = inspect_manifest(&contract, &manifest_text, &requirements);
    assert_complete(&requirements, &evidence);
    assert_no_orphan_expectations(&contract, &evidence.expectations);
}
