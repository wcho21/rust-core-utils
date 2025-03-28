use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn hello_1_text_arg() -> TestResult {
    let arg = "hello  world";
    let expected_stdout = "hello  world\n";

    let mut cmd = Command::cargo_bin("echo_r")?;
    cmd.arg(arg).assert().success().stdout(expected_stdout);

    Ok(())
}

#[test]
fn hello_2_text_args() -> TestResult {
    let args = vec!["hello", "world"];
    let expected_stdout = "hello world\n";

    let mut cmd = Command::cargo_bin("echo_r")?;
    cmd.args(args).assert().success().stdout(expected_stdout);

    Ok(())
}

#[test]
fn hello_n_1_text_arg() -> TestResult {
    let args = vec!["-n", "hello  world"];
    let expected_stdout = "hello  world";

    let mut cmd = Command::cargo_bin("echo_r")?;
    cmd.args(args).assert().success().stdout(expected_stdout);

    Ok(())
}

#[test]
fn hello_n_2_text_args() -> TestResult {
    let args = vec!["-n", "hello", "world"];
    let expected_stdout = "hello world";

    let mut cmd = Command::cargo_bin("echo_r")?;
    cmd.args(args).assert().success().stdout(expected_stdout);

    Ok(())
}

#[test]
fn dies_if_no_args() -> TestResult {
    let expected_stderr = predicate::str::contains("Usage");

    let mut cmd = Command::cargo_bin("echo_r")?;
    cmd.assert().failure().stderr(expected_stderr);

    Ok(())
}
