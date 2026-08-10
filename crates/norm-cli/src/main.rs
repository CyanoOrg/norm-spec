//! Bootstrap entry point for the `norm` command.

use std::{env, process::ExitCode};

const BOOTSTRAP_MESSAGE: &str =
    "norm is in bootstrap; parse, collect, validate, init, and scan are not implemented yet";

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    if let Some("--version" | "-V") = args.next().as_deref() {
        println!("norm {}", norm_spec_core::crate_version());
        ExitCode::SUCCESS
    } else {
        eprintln!("{BOOTSTRAP_MESSAGE}");
        ExitCode::from(2)
    }
}
