mod common;

use insta::with_settings;
use rstest::rstest;
use std::path::{Path, PathBuf};

#[test]
fn kaka() {
    common::setup();
}
const DISPLAY_SNAPSHOT_PATH: &str = "../snapshots";

fn to_snapshot_name(path: &Path) -> String {
    let mut iter = path.iter().map(|s| s.to_string_lossy().to_string());
    for seg in iter.by_ref() {
        if seg == "test-classes" {
            break;
        }
    }
    let tail: Vec<String> = iter.collect();
    tail.join("-")
}

#[rstest]
#[trace]
fn non_error_cases(
    #[base_dir = "../target/test-classes/vm"]
    #[files("**/*OkMain.class")]
    path: PathBuf,
) {
    // given
    // requires cargo build
    let current_path = std::env::current_dir().unwrap();
    let mut cmd = assert_cmd::Command::cargo_bin("vm").unwrap();
    cmd.args([&path]);

    // when
    let output = cmd.assert().success().get_output().clone();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let combined = format!(
        "----- STDOUT -----\n{}\n----- STDERR -----\n{}",
        stdout.trim_end(),
        stderr.trim_end()
    );

    // then
    with_settings!(
        {
            snapshot_path => DISPLAY_SNAPSHOT_PATH,
            prepend_module_to_snapshot => false,
        },
        {
            insta::assert_snapshot!(to_snapshot_name(&path), combined);
        }
    );
}

#[rstest]
#[trace]
fn error_cases(
    #[base_dir = "../target/test-classes/vm"]
    #[files("**/*ErrMain.class")]
    path: PathBuf,
) {
    // given
    // requires cargo build
    let mut cmd = assert_cmd::Command::cargo_bin("vm").unwrap();
    cmd.args([&path]);

    // when
    let output = cmd.assert().failure().get_output().clone();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let combined = format!(
        "----- STDOUT -----\n{}\n----- STDERR -----\n{}",
        stdout.trim_end(),
        stderr.trim_end()
    );

    // then
    with_settings!(
        {
            snapshot_path => DISPLAY_SNAPSHOT_PATH,
            prepend_module_to_snapshot => false,
        },
        {
            insta::assert_snapshot!(to_snapshot_name(&path), &combined);
        }
    );
}
