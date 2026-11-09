use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

fn create_test_log(contents: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

    let path = env::temp_dir().join(format!(
        "agent-log-grep-{}-{timestamp}-{id}.jsonl",
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

fn create_test_dir() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

    let root = env::temp_dir().join(format!(
        "agent-log-grep-dir-{}-{timestamp}-{id}",
        std::process::id()
    ));

    fs::create_dir(&root).unwrap();

    root
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
#[test]
fn search_finds_matches_in_nested_directories() {
    let root = create_test_dir();
    let nested = root.join("2026/11");

    fs::create_dir_all(&nested).unwrap();

    let first = root.join("0-first.log");
    let second = nested.join("second.jsonl");
    let ignored = root.join("ignored.txt");

    fs::write(&first, "no match\nerror in first\n").unwrap();
    fs::write(&second, "error in second\nno match\n").unwrap();
    fs::write(&ignored, "error in ignored file\n").unwrap();

    let output = run_cli(&["search", "error", root.to_str().unwrap()]);

    fs::remove_dir_all(&root).unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "{}:2: error in first\n{}:1: error in second\n",
        first.display(),
        second.display()
    );

    assert_eq!(stdout, expected);
}

#[test]
fn search_returns_empty_output_when_no_matches() {
    let path = create_test_log("first line\nsecond line\n");

    let output = run_cli(&["search", "not-found", path.to_str().unwrap()]);

    fs::remove_file(path).unwrap();

    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}
