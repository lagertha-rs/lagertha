use assert_cmd::cargo::cargo_bin_cmd;
use feature_tracking::fixture_identity;
use insta::with_settings;
use lvm_common::test_metadata::{TestCategory, parse_test_metadata};
use rstest::rstest;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DISPLAY_SNAPSHOT_PATH: &str = "../snapshots";

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

fn run_metadata_case(source_path: &Path) {
    let source = fs::read_to_string(source_path).expect("Cannot read test source");
    let metadata = parse_test_metadata(source_path, &source).expect("Invalid test metadata");
    let current_dir = std::env::current_dir().expect("Cannot get current dir");
    let fixtures_root = current_dir.join("tests/testdata");
    let class_path = fixtures_root.join("compiled");
    let main_class = PathBuf::from(
        fixture_identity(&fixtures_root, source_path, &source)
            .expect("Cannot derive compiled fixture identity"),
    );
    let mut cmd = cargo_bin_cmd!("vm");
    cmd.arg("-c").arg(&class_path).arg(&main_class);

    let output = cmd.output().expect("Cannot run Lagertha VM");
    let lvm_stdout = String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .to_string();
    let lvm_stderr = String::from_utf8_lossy(&output.stderr)
        .trim_end()
        .to_string();
    let lvm_status = output.status.code();

    match metadata.category {
        TestCategory::Success => {
            if !output.status.success() {
                panic!(
                    "\n\nLagertha FAILED (expected success) for {}\n\
                     Exit code: {:?}\n\
                     --- STDOUT ---\n{}\n\
                     --- STDERR ---\n{}\n",
                    source_path.display(),
                    lvm_status,
                    lvm_stdout,
                    lvm_stderr
                );
            }
        }
        TestCategory::Error => {
            if output.status.success() {
                panic!(
                    "\n\nLagertha SUCCEEDED (expected failure) for {}\n\
                     --- STDOUT ---\n{}\n\
                     --- STDERR ---\n{}\n",
                    source_path.display(),
                    lvm_stdout,
                    lvm_stderr
                );
            }
        }
    }

    let (jvm_stdout, jvm_stderr, jvm_status) = run_real_jvm(&class_path, &main_class);
    let jvm_stdout = jvm_stdout.trim_end().to_string();
    let jvm_stderr = jvm_stderr.trim_end().to_string();
    match metadata.category {
        TestCategory::Success => {
            if jvm_status != Some(0) {
                panic!(
                    "\n\nReference JVM FAILED (expected success) for {}\n\
                     Exit code: {:?}\n\
                     --- STDOUT ---\n{}\n\
                     --- STDERR ---\n{}\n",
                    source_path.display(),
                    jvm_status,
                    jvm_stdout,
                    jvm_stderr
                );
            }
        }
        TestCategory::Error => {
            if jvm_status == Some(0) {
                panic!(
                    "\n\nReference JVM SUCCEEDED (expected failure) for {}\n\
                     --- STDOUT ---\n{}\n\
                     --- STDERR ---\n{}\n",
                    source_path.display(),
                    jvm_stdout,
                    jvm_stderr
                );
            }
        }
    }

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
            insta::assert_snapshot!(to_snapshot_name(&main_class), combined);
        }
    );
}

#[rstest]
#[trace]
fn metadata_java_cases(
    #[base_dir = "tests/testdata"]
    #[files("**/*Test.java")]
    path: PathBuf,
) {
    run_metadata_case(&path);
}

#[rstest]
#[trace]
fn metadata_rns_cases(
    #[base_dir = "tests/testdata"]
    #[files("**/*Test.rns")]
    path: PathBuf,
) {
    run_metadata_case(&path);
}
