use assert_cmd::Command;
use insta::with_settings;
use rstest::rstest;
use std::path::{Path, PathBuf};

const DISPLAY_SNAPSHOT_PATH: &str = "../snapshots";

fn transform_absolute_path_to_package(path: &Path) -> PathBuf {
    let marker = Path::new("tests/testdata/compiled");
    let components = path.components().collect::<Vec<_>>();

    // Find index of "tests/testdata/compiled"
    let marker_parts = marker.components().collect::<Vec<_>>();
    let idx = components
        .windows(marker_parts.len())
        .position(|window| window == marker_parts)
        .expect("Marker path not found in the given path");

    let after = &components[idx + marker_parts.len()..];
    let mut new_path = PathBuf::new();
    for c in after {
        new_path.push(c);
    }

    // Remove ".class" extension if present
    new_path.set_extension("");

    new_path
}

fn to_snapshot_name(path: &Path) -> String {
    path.iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join("-")
}

#[rstest]
#[trace]
fn non_error_cases(
    #[base_dir = "tests/testdata/compiled"]
    #[files("**/*OkMain.class")]
    path: PathBuf,
) {
    // requires cargo build
    let current_dir = std::env::current_dir().expect("Cannot get current dir");
    let class_path = current_dir.join("tests/testdata/compiled");
    let main_class_path = transform_absolute_path_to_package(&path);
    let mut cmd = Command::cargo_bin("vm").unwrap();
    cmd.arg("-c").arg(class_path).arg(&main_class_path);

    let output = cmd.assert().success().get_output().clone();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let combined = format!(
        "----- STDOUT -----\n{}\n----- STDERR -----\n{}",
        stdout.trim_end(),
        stderr.trim_end()
    );

    with_settings!(
        {
            snapshot_path => DISPLAY_SNAPSHOT_PATH,
            prepend_module_to_snapshot => false,
        },
        {
            insta::assert_snapshot!(to_snapshot_name(&main_class_path), combined);
        }
    );
}

#[rstest]
#[trace]
fn error_cases(
    #[base_dir = "tests/testdata/compiled"]
    #[files("**/*ErrMain.class")]
    path: PathBuf,
) {
    // given
    // requires cargo build
    let current_dir = std::env::current_dir().expect("Cannot get current dir");
    let class_path = current_dir.join("tests/testdata/compiled");
    let main_class_path = transform_absolute_path_to_package(&path);
    let mut cmd = Command::cargo_bin("vm").unwrap();
    cmd.arg("-c").arg(class_path).arg(&main_class_path);

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
            insta::assert_snapshot!(to_snapshot_name(&main_class_path), &combined);
        }
    );
}
