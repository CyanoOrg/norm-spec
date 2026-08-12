//! Candidate discovery, bundle verification, and suite orchestration.

mod bundle;
mod model;
mod suite;

use std::{collections::BTreeSet, fs, path::Path, process::Command};

use norm_spec_core::{
    A1_CLI_CASE_COUNT, A1_CLI_CONTRACT_DIGEST, A1_CLI_SUITE_ID, COLLECT_API_VERSION,
    COMPATIBILITY_API_VERSION, CONFORMANCE_API_VERSION, CONTRACT_BUNDLE_API_VERSION,
    CompatibilityResponse, ERROR_API_VERSION, FORMAT_ID, INIT_API_VERSION, PARSE_API_VERSION,
    RUST_API_VERSION, SCAN_API_VERSION, VALIDATE_API_VERSION,
};

pub(crate) use model::ConformanceReport;
use model::{CandidateIdentity, ConformanceIssue, RunOutcome};

struct Preflight {
    candidate: CandidateIdentity,
    issue: Option<ConformanceIssue>,
    executable: bool,
}

pub(crate) fn run(candidate: &Path, contract_dir: &Path) -> RunOutcome {
    let candidate = match fs::canonicalize(candidate) {
        Ok(candidate) if candidate.is_file() => candidate,
        _ => {
            return RunOutcome {
                report: ConformanceReport::incomplete(
                    CandidateIdentity::unavailable(),
                    vec![ConformanceIssue::new(
                        "norm/conformance/candidate-unavailable",
                        "The candidate is unavailable.",
                    )],
                ),
                exit_code: 2,
            };
        }
    };
    let preflight = discover(&candidate);
    if !preflight.executable {
        return RunOutcome {
            report: ConformanceReport::incomplete(
                preflight.candidate,
                preflight.issue.into_iter().collect(),
            ),
            exit_code: 2,
        };
    }
    let mut issues: Vec<_> = preflight.issue.into_iter().collect();

    let verified = match bundle::verify(contract_dir) {
        Ok(verified) => verified,
        Err(error) => {
            let code = match error.kind {
                bundle::BundleErrorKind::Unavailable => "norm/conformance/bundle-unavailable",
                bundle::BundleErrorKind::Mismatch => "norm/conformance/bundle-mismatch",
                bundle::BundleErrorKind::UnsafePath => "norm/conformance/bundle-unsafe-path",
            };
            return RunOutcome {
                report: ConformanceReport::incomplete(preflight.candidate, {
                    issues.push(ConformanceIssue::new(code, error.message));
                    issues
                }),
                exit_code: 2,
            };
        }
    };
    let version = preflight.candidate.version.as_deref();
    let suite = suite::execute(&candidate, &verified.root, version);
    let complete = suite.executed == A1_CLI_CASE_COUNT && suite.issue.is_none();
    if let Some(issue) = suite.issue {
        issues.push(issue);
    }
    let failed = suite.failures.len();
    let passed = suite.executed.saturating_sub(failed);
    let not_executed = A1_CLI_CASE_COUNT.saturating_sub(suite.executed);
    let compatible = preflight.candidate.compatibility == "compatible";
    let (status, exit_code) = if !complete {
        ("error", 2)
    } else if !compatible || failed > 0 {
        ("fail", 1)
    } else {
        ("pass", 0)
    };
    RunOutcome {
        report: ConformanceReport {
            api_version: CONFORMANCE_API_VERSION.to_owned(),
            suite: model::SuiteIdentity::current(),
            candidate: preflight.candidate,
            status: status.to_owned(),
            complete,
            summary: model::ReportSummary {
                declared: A1_CLI_CASE_COUNT,
                executed: suite.executed,
                passed,
                failed,
                not_executed,
            },
            issues,
            failures: suite.failures,
        },
        exit_code,
    }
}

fn discover(candidate: &Path) -> Preflight {
    let Ok(output) = Command::new(candidate).arg("compatibility").output() else {
        return Preflight {
            candidate: CandidateIdentity::unavailable(),
            issue: Some(ConformanceIssue::new(
                "norm/conformance/candidate-unavailable",
                "The candidate could not be executed.",
            )),
            executable: false,
        };
    };
    if output.status.code() != Some(0) || !output.stderr.is_empty() {
        return unavailable_discovery();
    }
    let response: CompatibilityResponse = match serde_json::from_slice(&output.stdout) {
        Ok(response) if structurally_valid(&response) => response,
        _ => return unavailable_discovery(),
    };
    let compatible = compatible(&response);
    Preflight {
        candidate: CandidateIdentity {
            name: Some(response.product.name),
            version: Some(response.product.version),
            compatibility: if compatible {
                "compatible".to_owned()
            } else {
                "incompatible".to_owned()
            },
        },
        issue: (!compatible).then(|| {
            ConformanceIssue::new(
                "norm/conformance/candidate-incompatible",
                "The candidate compatibility identity does not match the requested suite.",
            )
        }),
        executable: true,
    }
}

fn unavailable_discovery() -> Preflight {
    Preflight {
        candidate: CandidateIdentity::unavailable(),
        issue: Some(ConformanceIssue::new(
            "norm/conformance/compatibility-unavailable",
            "The candidate did not provide a valid compatibility response.",
        )),
        executable: true,
    }
}

fn structurally_valid(response: &CompatibilityResponse) -> bool {
    !response.product.name.trim().is_empty()
        && !response.product.version.trim().is_empty()
        && !response.rust_api.package.trim().is_empty()
        && !response.rust_api.version.trim().is_empty()
        && !response.conformance.contract_digest.trim().is_empty()
}

fn compatible(response: &CompatibilityResponse) -> bool {
    let machine_apis: BTreeSet<_> = response.machine_apis.iter().map(String::as_str).collect();
    let required: BTreeSet<_> = [
        COLLECT_API_VERSION,
        COMPATIBILITY_API_VERSION,
        ERROR_API_VERSION,
        INIT_API_VERSION,
        PARSE_API_VERSION,
        SCAN_API_VERSION,
        VALIDATE_API_VERSION,
    ]
    .into_iter()
    .collect();
    response.api_version == COMPATIBILITY_API_VERSION
        && response.product.name == "norm-spec"
        && response.formats.iter().any(|format| format == FORMAT_ID)
        && response.rust_api.id == RUST_API_VERSION
        && response.rust_api.package == "norm-spec"
        && required.is_subset(&machine_apis)
        && response.conformance.bundle_api == CONTRACT_BUNDLE_API_VERSION
        && response.conformance.report_api == CONFORMANCE_API_VERSION
        && response.conformance.suite == A1_CLI_SUITE_ID
        && response.conformance.case_count == A1_CLI_CASE_COUNT
        && response.conformance.contract_digest == A1_CLI_CONTRACT_DIGEST
}
