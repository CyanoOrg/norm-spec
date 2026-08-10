//! Black-box bootstrap tests for the `norm` command.

use std::process::{Command, Output};

fn run_norm(args: &[&str]) -> Output {
    match Command::new(env!("CARGO_BIN_EXE_norm")).args(args).output() {
        Ok(output) => output,
        Err(error) => panic!("failed to execute norm: {error}"),
    }
}

#[test]
fn version_is_available_during_bootstrap() {
    let output = run_norm(&["--version"]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "norm 0.4.0-alpha.1\n"
    );
}

#[test]
fn unimplemented_commands_fail_explicitly() {
    let output = run_norm(&["validate"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("not implemented yet"));
}
