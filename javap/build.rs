use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

const FIXTURES_TOML_PATH: &str = "tests/testdata/fixtures.toml";
const COMPILED_FIXTURES_PATH: &str = "tests/testdata/compiled";

#[derive(Debug, Deserialize)]
struct Fixtures {
    modules: HashMap<String, Module>,
}
#[derive(Debug, Deserialize)]
struct Module {
    classes: Vec<String>,
}

fn set_rebuild_when_changed() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={FIXTURES_TOML_PATH}");
    println!("cargo:rerun-if-env-changed=JAVA_HOME");
}

fn main() {
    set_rebuild_when_changed();

    let fixtures_toml = PathBuf::from(FIXTURES_TOML_PATH);
    if !fixtures_toml.exists() {
        panic!(
            "[fixtures] No fixtures.toml in {}; skipping extraction.",
            FIXTURES_TOML_PATH
        );
    }

    let text = fs::read_to_string(&fixtures_toml).expect("failed to read fixtures.toml");
    let fixtures: Fixtures = toml::from_str(&text).expect("failed to parse fixtures.toml");
    let out_root = PathBuf::from(COMPILED_FIXTURES_PATH);

    let java_home =
        PathBuf::from(env::var("JAVA_HOME").expect("JAVA_HOME must be set for fixture extraction"));
    let jmods_dir = java_home.join("jmods");
    if !jmods_dir.is_dir() {
        panic!(
            "{} not found — need a JDK 9+ layout with jmods/",
            jmods_dir.display()
        );
    }

    if Command::new("jmod").arg("--help").output().is_err() {
        panic!(
            "`jmod` tool not found in PATH (should be in {}/bin)",
            java_home.display()
        );
    }

    extract_from_jmods(&fixtures, &jmods_dir, &out_root);
}

fn extract_from_jmods(fixtures: &Fixtures, jmods_dir: &Path, out_root: &Path) {
    let tmpdir = tempfile::tempdir().expect("tempdir");
    let mut copied = 0usize;

    for (module, payload) in &fixtures.modules {
        if payload.classes.is_empty() {
            panic!("no classes listed for module {}", module);
        }
        let jmod_path = jmods_dir.join(format!("{}.jmod", module));
        if !jmod_path.is_file() {
            panic!("Error: missing required JDK module JMOD: {}", module);
        }

        let extract_dir = tmpdir.path().join(module);
        fs::create_dir_all(&extract_dir).unwrap();

        let status = Command::new("jmod")
            .args(["extract", "--dir"])
            .arg(&extract_dir)
            .arg(&jmod_path)
            .status()
            .expect("failed to run jmod");
        if !status.success() {
            panic!("jmod extract failed for {}", jmod_path.display());
        }

        let classes_root = extract_dir.join("classes");
        for fqn in &payload.classes {
            let rel = fqn_to_rel(fqn);
            let src = classes_root.join(&rel);
            let dst = out_root.join(&rel);
            if !src.is_file() {
                panic!("ERROR: {}:{} not found at {}", module, fqn, src.display());
            }
            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::copy(&src, &dst).unwrap();
            copied += 1;
        }
    }

    if copied == 0 {
        panic!("[fixtures] No class files were extracted from JMODs.");
    }
}

fn fqn_to_rel(class_fqn: &str) -> PathBuf {
    let mut p = PathBuf::new();
    for part in class_fqn.split('.') {
        p.push(part);
    }
    p.set_extension("class");
    p
}
