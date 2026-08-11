//! Static integrity checks for the language-neutral Gate B contract manifest.

use std::{collections::BTreeSet, fs, path::Path};

use norm_spec_core::{
    COLLECT_API_VERSION, ERROR_API_VERSION, INIT_API_VERSION, PARSE_API_VERSION, SCAN_API_VERSION,
    VALIDATE_API_VERSION,
};

fn read_required(path: &Path) -> String {
    match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) => panic!(
            "required contract file {} is unavailable: {error}",
            path.display()
        ),
    }
}

#[test]
fn contract_manifest_covers_initial_protocols() {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let contract = repository.join("tests/contract");
    let manifest_path = contract.join("manifest.tsv");
    let manifest = read_required(&manifest_path);
    let mut case_ids = BTreeSet::new();
    let mut protocols = BTreeSet::new();
    let mut case_count = 0_u32;

    for (index, line) in manifest.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let columns: Vec<_> = line.split('\t').collect();
        assert_eq!(
            columns.len(),
            6,
            "manifest line {} must have six tab-separated columns",
            index + 1
        );

        let case_id = columns[0];
        let command = columns[1];
        let fixture = columns[2];
        let expected = columns[3];
        let protocol = columns[4];
        let exit_code = match columns[5].parse::<u8>() {
            Ok(value) => value,
            Err(error) => panic!("case {case_id} has an invalid exit code: {error}"),
        };
        let success_protocol = match command {
            "parse" => PARSE_API_VERSION,
            "collect" => COLLECT_API_VERSION,
            "validate" => VALIDATE_API_VERSION,
            "init" => INIT_API_VERSION,
            "scan" => SCAN_API_VERSION,
            _ => panic!("case {case_id} names an unsupported command: {command}"),
        };
        let required_protocol = if exit_code == 0 {
            success_protocol
        } else {
            ERROR_API_VERSION
        };

        assert!(
            case_ids.insert(case_id),
            "duplicate contract case ID: {case_id}"
        );
        assert!(
            exit_code <= 2,
            "case {case_id} has unsupported exit code {exit_code}"
        );
        assert_eq!(
            protocol, required_protocol,
            "case {case_id} uses a protocol inconsistent with its command and exit code"
        );

        if fixture != "-" {
            assert!(
                contract.join(fixture).exists(),
                "case {case_id} is missing fixture {fixture}"
            );
        }

        let expected_path = contract.join(expected);
        let expected_contents = read_required(&expected_path);
        assert!(
            expected_contents.trim_start().starts_with('{')
                && expected_contents.trim_end().ends_with('}'),
            "case {case_id} expected output is not a JSON object"
        );
        assert!(
            expected_contents.contains(&format!("\"apiVersion\": \"{protocol}\"")),
            "case {case_id} expected output does not declare {protocol}"
        );

        protocols.insert(protocol);
        case_count += 1;
    }

    assert!(
        case_count > 0,
        "contract manifest must contain at least one case"
    );
    for required in [
        PARSE_API_VERSION,
        COLLECT_API_VERSION,
        VALIDATE_API_VERSION,
        INIT_API_VERSION,
        SCAN_API_VERSION,
        ERROR_API_VERSION,
    ] {
        assert!(
            protocols.contains(required),
            "contract manifest is missing {required}"
        );
    }
}
