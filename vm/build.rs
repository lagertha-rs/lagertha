use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

const FIXTURES_ROOT: &str = "tests/testdata";
const COMPILED_FIXTURES_ROOT: &str = "tests/testdata/compiled";

const SUPPORTED_JAVAC: &str = "25.0.1";
const SUPPORTED_RNSC: &str = "0.2.1";

fn set_rebuild_when_changed() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", FIXTURES_ROOT);
    println!("cargo:rerun-if-env-changed=JAVA_HOME");
}

fn main() {
    set_rebuild_when_changed();
    check_tool_versions();
    compile_test_fixtures();
    compile_rns_fixtures();
}

fn check_tool_versions() {
    let javac = std::env::var("JAVA_HOME")
        .map(|j| Path::new(&j).join("bin/javac"))
        .ok()
        .filter(|p| p.exists())
        .map(|p| p.into_os_string())
        .unwrap_or_else(|| "javac".into());

    let javac_out = Command::new(&javac)
        .arg("--version")
        .output()
        .expect("Failed to run javac --version");
    let javac_ver = String::from_utf8_lossy(&javac_out.stdout)
        .trim()
        .split_whitespace()
        .last()
        .unwrap_or("")
        .to_string();
    if javac_ver != SUPPORTED_JAVAC {
        panic!(
            "javac version mismatch: expected {}, got {}",
            SUPPORTED_JAVAC, javac_ver
        );
    }

    let rnsc_out = Command::new("rnsc")
        .arg("--version")
        .output()
        .expect("Failed to run rnsc --version. Install with: cargo install rnsc");
    let rnsc_ver = String::from_utf8_lossy(&rnsc_out.stdout)
        .trim()
        .split_whitespace()
        .last()
        .unwrap_or("")
        .to_string();
    if rnsc_ver != SUPPORTED_RNSC {
        panic!(
            "rnsc version mismatch: expected {}, got {}",
            SUPPORTED_RNSC, rnsc_ver
        );
    }
}

fn remove_compiled_dir_if_exists() {
    let _ = fs::remove_dir_all(COMPILED_FIXTURES_ROOT);
}

fn compile_test_fixtures() {
    let java_files = collect_fixtures("java");
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

fn compile_rns_fixtures() {
    let rns_files = collect_fixtures("rns");
    if rns_files.is_empty() {
        return;
    }

    for file in &rns_files {
        let rel = fixture_output_path(file);
        let mut out = PathBuf::from(COMPILED_FIXTURES_ROOT);
        out.push(&rel);
        out.set_extension("class");

        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent).expect("Failed to create output dir");
        }

        let output = Command::new("rnsc")
            .arg("asm")
            .arg(file)
            .arg("-o")
            .arg(&out)
            .output()
            .expect("Failed to run rnsc");

        if !output.status.success() {
            panic!(
                "rnsc failed on {}: {}",
                file.display(),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}

fn collect_fixtures(extension: &str) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = WalkDir::new(FIXTURES_ROOT)
        .into_iter()
        .filter_map(Result::ok)
        .map(|e| e.into_path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .is_some_and(|candidate| candidate == extension)
        })
        .collect();
    files.sort();
    files
}

fn fixture_output_path(path: &Path) -> PathBuf {
    path.strip_prefix(FIXTURES_ROOT)
        .expect("fixture not under fixtures root")
        .to_path_buf()
}
