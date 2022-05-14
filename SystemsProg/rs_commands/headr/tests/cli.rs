use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/bustle.txt";

fn get_randfilename() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn run_normal(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("USAGE"));
    }
    Ok(())
}

#[test]
fn skips_bad_file() -> TestResult {
    let badfilename = get_randfilename();
    let expected = format!("Failed to open {}: .* [(]os error 2[)]", badfilename);
    Command::cargo_bin(PRG)?
        .arg(&badfilename)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

#[test]
fn bustle_stdin_n() -> TestResult {
    run_stdin(BUSTLE, &["-n", "-"], "tests/expected/bustle.n.txt")
}

#[test]
fn empty() -> TestResult {
    run_normal(&[EMPTY], "tests/expected/empty.txt")
}

#[test]
fn fox() -> TestResult {
    run_normal(&[FOX], "tests/expected/fox.txt")
}

#[test]
fn spiders_b() -> TestResult {
    run_normal(
        &["--number-nonblank", SPIDERS],
        "tests/expected/spiders.b.txt",
    )
}
