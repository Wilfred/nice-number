use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_small_integer() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.write_stdin("42")
        .assert()
        .success()
        .stdout(predicate::str::contains("42"))
        .stdout(predicate::str::contains("(small)"));
}

#[test]
fn test_small_decimal() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.write_stdin("42.50")
        .assert()
        .success()
        .stdout(predicate::str::contains("42.50"))
        .stdout(predicate::str::contains("(small)"));
}

#[test]
fn test_decimal_with_rounding() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.write_stdin("42.123456")
        .assert()
        .success()
        .stdout(predicate::str::contains("42.12"))
        .stdout(predicate::str::contains("(rounded)"))
        .stdout(predicate::str::contains("(small)"));
}

#[test]
fn test_medium_number() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.write_stdin("5000")
        .assert()
        .success()
        .stdout(predicate::str::contains("5,000"))
        .stdout(predicate::str::contains("(medium)"));
}

#[test]
fn test_pretty_big_number() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.write_stdin("1234567")
        .assert()
        .success()
        .stdout(predicate::str::contains("1,234,567"))
        .stdout(predicate::str::contains("(pretty big)"));
}

#[test]
fn test_pretty_big_decimal() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.write_stdin("1234567.89")
        .assert()
        .success()
        .stdout(predicate::str::contains("1,234,567.89"))
        .stdout(predicate::str::contains("(pretty big)"));
}

#[test]
fn test_extremely_big_number() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.write_stdin("9876543210")
        .assert()
        .success()
        .stdout(predicate::str::contains("9,876,543,210"))
        .stdout(predicate::str::contains("(extremely big)"));
}

#[test]
fn test_negative_small_number() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.write_stdin("-42")
        .assert()
        .success()
        .stdout(predicate::str::contains("-42"))
        .stdout(predicate::str::contains("(small)"));
}

#[test]
fn test_negative_medium_number() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.write_stdin("-5000")
        .assert()
        .success()
        .stdout(predicate::str::contains("-5,000"))
        .stdout(predicate::str::contains("(medium)"));
}

#[test]
fn test_zero() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.write_stdin("0")
        .assert()
        .success()
        .stdout(predicate::str::contains("0"))
        .stdout(predicate::str::contains("(small)"));
}

#[test]
fn test_invalid_input() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.write_stdin("not a number")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Error: Please enter a valid number",
        ));
}

#[test]
fn test_empty_input() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.write_stdin("")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Error: Please enter a valid number",
        ));
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Format numbers with thousand separators",
        ))
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_help_short_flag() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Format numbers with thousand separators",
        ));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("nn"))
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_version_short_flag() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("nn"))
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_decimal_no_rounding_needed() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.write_stdin("123.45")
        .assert()
        .success()
        .stdout(predicate::str::contains("123.45"))
        .stdout(predicate::str::contains("(small)"))
        .stdout(predicate::str::contains("(rounded)").not());
}

#[test]
fn test_large_decimal_with_rounding() {
    let mut cmd = Command::cargo_bin("nn").unwrap();
    cmd.write_stdin("9876543210.999")
        .assert()
        .success()
        .stdout(predicate::str::contains("9,876,543,211"))
        .stdout(predicate::str::contains("(rounded)"))
        .stdout(predicate::str::contains("(extremely big)"));
}
