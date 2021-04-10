use assert_cmd::prelude::*;
use predicates::prelude::*;
use rstest::*;
use std::error::Error;
use std::process::Command;

#[test]
fn showing_help() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("code-pattern")?;

    cmd.assert()
        .success()
        .code(0)
        .stdout(predicates::str::is_empty().not());

    Ok(())
}

#[rstest]
#[case("")]
#[case("-")]
#[case("- x-x-x")]
fn missing_cli_arguments(#[case] args: &str) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("code-pattern")?;

    cmd.args(args.split(" ").into_iter());
    cmd.assert()
        .failure()
        .code(1)
        .stdout(predicates::str::contains("One or more arguments missing!"));

    Ok(())
}

#[rstest]
#[case("- x-x-x 123", "1-2-3")]
#[case("- x-xx-xxx 123456", "1-23-456")]
#[case("; x;xx;xxx 123456", "1;23;456")]
#[case("; x;xx;xxx;xx;x abcdefghi", "a;bc;def;gh;i")]
fn applying_pattern_on_valid_code(
    #[case] args: &str,
    #[case] code: &str,
) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("code-pattern")?;

    cmd.args(args.split(" ").into_iter());
    cmd.assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains(code));

    Ok(())
}
