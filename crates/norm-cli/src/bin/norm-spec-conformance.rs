//! Independent arbitrary-candidate conformance runner for norm-spec.

mod conformance;

use std::{path::PathBuf, process::ExitCode};

use clap::{CommandFactory, Parser, error::ErrorKind};

use conformance::{ConformanceReport, run};

#[derive(Debug, Parser)]
#[command(
    name = "norm-spec-conformance",
    version,
    about = "Run the frozen norm-spec CLI contract against an arbitrary candidate"
)]
struct Cli {
    /// Exact candidate binary to execute.
    #[arg(long)]
    candidate: Option<PathBuf>,
    /// Exact exported contract bundle to verify and execute.
    #[arg(long)]
    contract_dir: Option<PathBuf>,
    /// Pretty-print the JSON report.
    #[arg(long)]
    pretty: bool,
}

fn main() -> ExitCode {
    let arguments = match Cli::try_parse() {
        Ok(arguments) => arguments,
        Err(error)
            if matches!(
                error.kind(),
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
            ) =>
        {
            let _ = error.print();
            return ExitCode::SUCCESS;
        }
        Err(error) => {
            return emit(&ConformanceReport::usage(error.to_string()), false, 2);
        }
    };

    let (Some(candidate), Some(contract_dir)) = (arguments.candidate, arguments.contract_dir)
    else {
        let message = Cli::command()
            .error(
                ErrorKind::MissingRequiredArgument,
                "--candidate and --contract-dir are required",
            )
            .to_string();
        return emit(&ConformanceReport::usage(message), arguments.pretty, 2);
    };

    let outcome = run(&candidate, &contract_dir);
    emit(&outcome.report, arguments.pretty, outcome.exit_code)
}

fn emit(report: &ConformanceReport, pretty: bool, exit_code: u8) -> ExitCode {
    let serialized = if pretty {
        serde_json::to_string_pretty(report)
    } else {
        serde_json::to_string(report)
    };
    match serialized {
        Ok(json) => {
            println!("{json}");
            ExitCode::from(exit_code)
        }
        Err(error) => {
            eprintln!("norm-spec-conformance serialization failure: {error}");
            ExitCode::from(2)
        }
    }
}
