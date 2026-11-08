use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn create_test_log(contents: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let path = env::temp_dir().join(format!(
        "agent-log-grep-{}-{timestamp}.jsonl",
        std::process::id()
    ));

    fs::write(&path, contents).unwrap();

    path
}

fn run_cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_agent-log-grep"))
        .args(args)
        .env_remove("IGNORE_CASE")
        .output()
        .unwrap()
}

#[test]
fn search_finds_matching_records() {
    let path = create_test_log("ERROR upper\nerror lower\nother\n");

    let output = run_cli(&["search", "error", path.to_str().unwrap()]);

    fs::remove_file(path).unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout, "2: error lower\n");
}

#[test]
fn search_ignores_case_with_flag() {
    let path = create_test_log("ERROR upper\nerror lower\nother\n");

    let output = run_cli(&["search", "-i", "error", path.to_str().unwrap()]);

    fs::remove_file(path).unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout, "1: ERROR upper\n2: error lower\n");
}

#[test]
fn show_displays_all_records() {
    let path = create_test_log("first line\nsecond line\n");

    let output = run_cli(&["show", path.to_str().unwrap()]);

    fs::remove_file(path).unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout, "1: first line\n2: second line\n");
}

#[test]
fn rejects_unknown_command() {
    let output = run_cli(&["delete", "log.jsonl"]);

    assert!(!output.status.success());
}

#[test]
fn reports_missing_file() {
    let path = create_test_log("temporary");
    fs::remove_file(&path).unwrap();

    let output = run_cli(&["show", path.to_str().unwrap()]);

    assert!(!output.status.success());
}
