//! Bundle verification for the staged conformance runner.

mod bundle;
mod model;

use std::{fs, path::Path};

pub(crate) use model::ConformanceReport;
use model::{CandidateIdentity, ConformanceIssue, RunOutcome};

pub(crate) fn run(candidate: &Path, contract_dir: &Path) -> RunOutcome {
    match fs::canonicalize(candidate) {
        Ok(candidate) if candidate.is_file() => {}
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
    }

    let verified = match bundle::verify(contract_dir) {
        Ok(verified) => verified,
        Err(error) => {
            let code = match error.kind {
                bundle::BundleErrorKind::Unavailable => "norm/conformance/bundle-unavailable",
                bundle::BundleErrorKind::Mismatch => "norm/conformance/bundle-mismatch",
                bundle::BundleErrorKind::UnsafePath => "norm/conformance/bundle-unsafe-path",
            };
            return RunOutcome {
                report: ConformanceReport::incomplete(
                    CandidateIdentity::unavailable(),
                    vec![ConformanceIssue::new(code, error.message)],
                ),
                exit_code: 2,
            };
        }
    };
    if !verified.root.is_dir() {
        return RunOutcome {
            report: ConformanceReport::incomplete(
                CandidateIdentity::unavailable(),
                vec![ConformanceIssue::new(
                    "norm/conformance/bundle-unavailable",
                    "The verified contract bundle became unavailable.",
                )],
            ),
            exit_code: 2,
        };
    }

    RunOutcome {
        report: ConformanceReport::incomplete(
            CandidateIdentity::unavailable(),
            vec![ConformanceIssue::new(
                "norm/conformance/runner-unsupported",
                "Candidate execution is not available in this staged runner.",
            )],
        ),
        exit_code: 2,
    }
}
