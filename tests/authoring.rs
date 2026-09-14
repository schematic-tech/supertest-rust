use schematic::{assume, supertest};
use std::process::Command;

#[supertest]
fn marked_function(value: i32) -> i32 {
    value + 1
}

#[test]
fn marker_preserves_the_function_item() {
    assert_eq!(marked_function(2), 3);
}

#[test]
fn true_assumption_evaluates_once_and_continues() {
    let mut calls = 0;
    assume({
        calls += 1;
        true
    });
    assert_eq!(calls, 1);
}

fn probe(mode: &str) -> std::process::Output {
    Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "runtime_probe", "--ignored", "--nocapture"])
        .env("SUPERTEST_PROBE_MODE", mode)
        .output()
        .unwrap()
}

#[test]
fn false_assumption_exits_successfully_without_unwinding() {
    let output = probe("false");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8(output.stderr).unwrap().trim(), "entered");
}

#[test]
fn assertion_failure_is_not_a_successful_exit() {
    let output = probe("assertion");
    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("ordinary assertion failure"));
}

// A child test process isolates process::exit from the parent test harness.
#[test]
#[ignore = "invoked in a subprocess by the runtime tests"]
fn runtime_probe() {
    let mode = std::env::var("SUPERTEST_PROBE_MODE").expect("run via the parent test");
    eprintln!("entered");
    let _ = std::panic::catch_unwind(|| {
        assume(mode != "false");
        panic!("ordinary assertion failure");
    });
    eprintln!("continued");
    std::process::exit(91);
}
