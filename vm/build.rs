use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

const JAVA_FIXTURES_ROOT: &str = "tests/testdata/java";
const COMPILED_FIXTURES_ROOT: &str = "tests/testdata/compiled";

fn set_rebuild_when_changed() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", JAVA_FIXTURES_ROOT);
    println!("cargo:rerun-if-env-changed=JAVA_HOME")
}

fn main() {
    set_rebuild_when_changed();
    compile_test_fixtures();
}

fn remove_compiled_dir_if_exists() {
    let _ = fs::remove_dir_all(COMPILED_FIXTURES_ROOT);
}

fn compile_test_fixtures() {
    let java_files = collect_vm_java_fixtures();
    if java_files.is_empty() {
        panic!("No Java files found in fixtures.");
    }

    let javac = std::env::var("JAVA_HOME")
        .map(|j| Path::new(&j).join("bin/javac"))
        .ok()
        .filter(|p| p.exists())
        .map(|p| p.into_os_string())
        .unwrap_or_else(|| "javac".into());

    remove_compiled_dir_if_exists();

    let mut cmd = Command::new(javac);
    cmd.arg("-encoding")
        .arg("UTF-8")
        .arg("-g")
        .arg("-d")
        .arg(COMPILED_FIXTURES_ROOT);

    for file in &java_files {
        cmd.arg(file);
    }

    let output = cmd.output().expect("Failed to run javac");
    if !output.status.success() {
        panic!("javac failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn collect_vm_java_fixtures() -> Vec<PathBuf> {
    let mut java_files: Vec<PathBuf> = WalkDir::new(JAVA_FIXTURES_ROOT)
        .into_iter()
        .filter_map(Result::ok)
        .map(|e| e.into_path())
        .filter(|p| p.is_file() && p.extension().map(|e| e == "java").unwrap_or(false))
        .collect();
    java_files.sort();
    java_files
}
