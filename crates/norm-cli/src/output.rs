//! Shared CLI machine-output handling.

use std::process::ExitCode;

use norm_spec_core::{ErrorDetail, ErrorResponse};
use serde::Serialize;

pub(crate) const EXIT_OPERATION: u8 = 1;
pub(crate) const EXIT_USAGE: u8 = 2;

pub(crate) fn emit_error(
    command: &str,
    error: ErrorDetail,
    pretty: bool,
    exit_code: u8,
) -> ExitCode {
    emit_json(&ErrorResponse::new(command, error), pretty, exit_code)
}

pub(crate) fn emit_json(value: &impl Serialize, pretty: bool, exit_code: u8) -> ExitCode {
    let serialized = if pretty {
        serde_json::to_string_pretty(value)
    } else {
        serde_json::to_string(value)
    };

    match serialized {
        Ok(json) => {
            println!("{json}");
            ExitCode::from(exit_code)
        }
        Err(error) => {
            eprintln!("norm internal serialization failure: {error}");
            ExitCode::from(EXIT_OPERATION)
        }
    }
}
