use constcat::concat;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

const JAVA_FIXTURES_ROOT: &str = "tests/testdata/java";
const COMPILED_FIXTURES_ROOT: &str = "../compiled";
const HASH_FILE_PATH: &str = concat!(COMPILED_FIXTURES_ROOT, "/.hash");

fn set_current_dir_to_java_fixtures() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let java_fixtures_path = current_dir.join(JAVA_FIXTURES_ROOT);
    std::env::set_current_dir(&java_fixtures_path)
        .expect("Failed to set current directory to Java fixtures");
}

fn collect_java_files() -> Vec<PathBuf> {
    let mut java_files: Vec<PathBuf> = WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .map(|e| e.into_path())
        .filter(|path| path.is_file() && path.extension().map(|ext| ext == "java").unwrap_or(false))
        .collect();

    java_files.sort_by_key(|e| e.to_owned());
    java_files
}

/// Compute SHA-256 over (relative path + file contents) for each file
fn compute_hash(files: &[PathBuf]) -> std::io::Result<[u8; 32]> {
    let mut hasher = Sha256::new();

    for path in files {
        let rel_str = path.to_string_lossy();
        hasher.update(rel_str.as_bytes());

        let mut file = fs::File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        hasher.update(&buf);
    }

    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest[..]);
    Ok(out)
}

fn read_saved_hash() -> Option<Vec<u8>> {
    fs::read(HASH_FILE_PATH).ok()
}

fn hashes_match(current: &[u8]) -> bool {
    if let Some(saved) = read_saved_hash() {
        saved == current
    } else {
        false
    }
}

fn remove_compiled_dir_if_exists() {
    let _ = fs::remove_dir(COMPILED_FIXTURES_ROOT);
}

fn write_hash(hash: &[u8]) -> std::io::Result<()> {
    let mut f = fs::File::create(HASH_FILE_PATH)?;
    f.write_all(hash)?;
    Ok(())
}

pub fn setup() {
    set_current_dir_to_java_fixtures();

    let java_files = collect_java_files();
    if java_files.is_empty() {
        panic!("No Java files found in fixtures.");
    }

    let hash = compute_hash(&java_files).expect("Failed to compute folder hash");
    if hashes_match(&hash) {
        //No changes detected in Java sources, skipping compilation
        return;
    }

    remove_compiled_dir_if_exists();

    let mut cmd = Command::new("javac");
    cmd.arg("-d").arg(COMPILED_FIXTURES_ROOT);
    for file in &java_files {
        cmd.arg(file);
    }

    // Run javac
    let output = cmd.output().expect("Failed to run javac");

    if !output.status.success() {
        panic!("javac failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    write_hash(&hash).expect("Failed to write hash file");
}
