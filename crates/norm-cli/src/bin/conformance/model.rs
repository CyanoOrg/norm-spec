//! Versioned conformance report model.

use norm_spec_core::{
    A1_CLI_CASE_COUNT, A1_CLI_CONTRACT_DIGEST, A1_CLI_SUITE_ID, CONFORMANCE_API_VERSION,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct SuiteIdentity {
    pub(super) id: String,
    pub(super) case_count: usize,
    pub(super) contract_digest: String,
}

impl SuiteIdentity {
    pub(super) fn current() -> Self {
        Self {
            id: A1_CLI_SUITE_ID.to_owned(),
            case_count: A1_CLI_CASE_COUNT,
            contract_digest: A1_CLI_CONTRACT_DIGEST.to_owned(),
        }
    }
}

#[derive(Debug, Serialize)]
pub(super) struct CandidateIdentity {
    pub(super) name: Option<String>,
    pub(super) version: Option<String>,
    pub(super) compatibility: String,
}

impl CandidateIdentity {
    pub(super) fn unavailable() -> Self {
        Self {
            name: None,
            version: None,
            compatibility: "unavailable".to_owned(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ReportSummary {
    pub(super) declared: usize,
    pub(super) executed: usize,
    pub(super) passed: usize,
    pub(super) failed: usize,
    pub(super) not_executed: usize,
}

#[derive(Debug, Serialize)]
pub(super) struct ConformanceIssue {
    pub(super) code: String,
    pub(super) message: String,
}

impl ConformanceIssue {
    pub(super) fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_owned(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CaseFailure {
    pub(super) case_id: String,
    pub(super) checks: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConformanceReport {
    #[serde(rename = "apiVersion")]
    pub(super) api_version: String,
    pub(super) suite: SuiteIdentity,
    pub(super) candidate: CandidateIdentity,
    pub(super) status: String,
    pub(super) complete: bool,
    pub(super) summary: ReportSummary,
    pub(super) issues: Vec<ConformanceIssue>,
    pub(super) failures: Vec<CaseFailure>,
}

impl ConformanceReport {
    pub(crate) fn usage(message: impl Into<String>) -> Self {
        Self::incomplete(
            CandidateIdentity::unavailable(),
            vec![ConformanceIssue::new("norm/conformance/usage", message)],
        )
    }

    pub(super) fn incomplete(candidate: CandidateIdentity, issues: Vec<ConformanceIssue>) -> Self {
        Self {
            api_version: CONFORMANCE_API_VERSION.to_owned(),
            suite: SuiteIdentity::current(),
            candidate,
            status: "error".to_owned(),
            complete: false,
            summary: ReportSummary {
                declared: A1_CLI_CASE_COUNT,
                executed: 0,
                passed: 0,
                failed: 0,
                not_executed: A1_CLI_CASE_COUNT,
            },
            issues,
            failures: Vec::new(),
        }
    }
}

pub(crate) struct RunOutcome {
    pub(crate) report: ConformanceReport,
    pub(crate) exit_code: u8,
}
