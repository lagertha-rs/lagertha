use assert_cmd::cargo::cargo_bin_cmd;
use insta::with_settings;
use rstest::rstest;
use std::path::{Path, PathBuf};
use std::process::Command;

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

fn java_bin() -> String {
    std::env::var("JAVA_HOME")
        .ok()
        .map(|j| {
            Path::new(&j)
                .join("bin/java")
                .to_string_lossy()
                .into_owned()
        })
        .filter(|p| Path::new(p).exists())
        .unwrap_or_else(|| "java".to_string())
}

fn run_real_jvm(class_path: &Path, main_class: &Path) -> (String, String, Option<i32>) {
    let bin = java_bin();
    let out = Command::new(&bin)
        .arg("-ea")
        .arg("-cp")
        .arg(class_path)
        .arg(main_class)
        .output();
    match out {
        Ok(o) => (
            String::from_utf8_lossy(&o.stdout).into_owned(),
            String::from_utf8_lossy(&o.stderr).into_owned(),
            o.status.code(),
        ),
        Err(e) => (String::new(), format!("failed to spawn {bin}: {e}"), None),
    }
}

fn render_combined(
    lagertha_stdout: &str,
    lagertha_stderr: &str,
    lagertha_status: Option<i32>,
    jvm_stdout: &str,
    jvm_stderr: &str,
    jvm_status: Option<i32>,
) -> String {
    format!(
        "===== Lagertha VM =====\n\
         ----- exit: {lagertha_status:?} -----\n\
         ----- STDOUT -----\n{lagertha_stdout}\n\
         ----- STDERR -----\n{lagertha_stderr}\n\n\
         ===== Real JVM =====\n\
         ----- exit: {jvm_status:?} -----\n\
         ----- STDOUT -----\n{jvm_stdout}\n\
         ----- STDERR -----\n{jvm_stderr}"
    )
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
    let mut cmd = cargo_bin_cmd!("vm");
    cmd.arg("-c").arg(&class_path).arg(&main_class_path);

    let output = cmd.assert().success().get_output().clone();
    let lvm_stdout = String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .to_string();
    let lvm_stderr = String::from_utf8_lossy(&output.stderr)
        .trim_end()
        .to_string();
    let lvm_status = output.status.code();

    let (jvm_stdout, jvm_stderr, jvm_status) = run_real_jvm(&class_path, &main_class_path);
    let jvm_stdout = jvm_stdout.trim_end().to_string();
    let jvm_stderr = jvm_stderr.trim_end().to_string();
    assert_eq!(
        jvm_status,
        Some(0),
        "real JVM should exit 0 for OkMain: {:?}",
        jvm_stderr
    );

    let combined = render_combined(
        &lvm_stdout,
        &lvm_stderr,
        lvm_status,
        &jvm_stdout,
        &jvm_stderr,
        jvm_status,
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
    let mut cmd = cargo_bin_cmd!("vm");
    cmd.arg("-c").arg(&class_path).arg(&main_class_path);

    // when
    let output = cmd.assert().failure().get_output().clone();
    let lvm_stdout = String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .to_string();
    let lvm_stderr = String::from_utf8_lossy(&output.stderr)
        .trim_end()
        .to_string();
    let lvm_status = output.status.code();

    let (jvm_stdout, jvm_stderr, jvm_status) = run_real_jvm(&class_path, &main_class_path);
    let jvm_stdout = jvm_stdout.trim_end().to_string();
    let jvm_stderr = jvm_stderr.trim_end().to_string();
    assert_ne!(
        jvm_status,
        Some(0),
        "real JVM should exit non-zero for ErrMain: {:?}",
        jvm_stderr
    );

    // then
    let combined = render_combined(
        &lvm_stdout,
        &lvm_stderr,
        lvm_status,
        &jvm_stdout,
        &jvm_stderr,
        jvm_status,
    );

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
