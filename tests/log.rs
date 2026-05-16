use std::{path::Path, process::Command};

fn build<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let output = Command::new("cargo")
        .args(["build"])
        .current_dir(path)
        .output()
        .expect("failed to build test crate");

    assert!(output.status.success(), "build failed");

    String::from_utf8(output.stderr)
        .expect("stderr did not contain valid UTF-8")
        .lines()
        .filter_map(|s| s.strip_prefix("warning: ").map(ToOwned::to_owned))
        .collect()
}

#[test]
fn trace() {
    let output = build("tests/trace");

    assert_eq!(
        output,
        vec!["trace@0.0.0: TRACE:build_script_build -- Hello, world!"]
    );
}

#[test]
fn debug() {
    let output = build("tests/debug");

    assert_eq!(
        output,
        vec!["debug@0.0.0: DEBUG:build_script_build -- Hello, world!"]
    );
}

#[test]
fn info() {
    let output = build("tests/info");

    assert_eq!(
        output,
        vec!["info@0.0.0: INFO:build_script_build -- Hello, world!"]
    );
}

#[test]
fn warn() {
    let output = build("tests/warn");

    assert_eq!(
        output,
        vec!["warn@0.0.0: WARN:build_script_build -- Hello, world!"]
    );
}

#[test]
fn error() {
    let output = build("tests/error");

    assert_eq!(
        output,
        vec!["error@0.0.0: ERROR:build_script_build -- Hello, world!"]
    );
}
