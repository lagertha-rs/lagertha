use assert_cmd::cargo_bin_cmd;
use insta::with_settings;
use rstest::rstest;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

const SNAPSHOT_PATH: &str = "snapshots";

fn normalize_stderr_paths(stderr: &str, input_path: &Path) -> String {
    if let Ok(absolute) = input_path.canonicalize() {
        let abs_str = absolute.to_string_lossy();
        let mut normalized = stderr.to_string();

        let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let rel_path = if let Ok(rel) = absolute.strip_prefix(&cwd) {
            rel.to_string_lossy()
        } else {
            input_path.to_string_lossy()
        };

        normalized = normalized.replace(&*abs_str, &rel_path);

        let abs_with_colon = format!("{}:", abs_str);
        let rel_with_colon = format!("{}:", rel_path);
        normalized = normalized.replace(&abs_with_colon, &rel_with_colon);

        return normalized;
    }

    stderr.to_string()
}

fn get_file_contents(path: &Path) -> String {
    fs::read_to_string(path).expect("Unable to read file")
}

fn get_hash(path: &Path) -> String {
    let bytes = fs::read(path).expect("Unable to read class file");
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    format!("{:x}", hasher.finalize())
}

fn to_snapshot_name(path: &Path) -> String {
    let marker = Path::new("test_data/integration");
    let components = path.components().collect::<Vec<_>>();
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
    new_path.set_extension("");

    let stem = new_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let parent = new_path
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    let base_name = format!("{}--{}", parent, stem);
    base_name.replace("/", "-").replace("--", "-")
}

fn run_assemble_command(path: &Path, output_path: &Path) -> Output {
    let mut assemble_cmd = cargo_bin_cmd!("rnsc");
    assemble_cmd
        .arg("asm")
        .arg(path)
        .arg("--output")
        .arg(output_path);
    assemble_cmd.output().expect("Failed to assemble")
}

fn assert_stdout_empty(stdout: &str) {
    assert!(
        stdout.trim().is_empty(),
        "Expected stdout to be empty, but got:\n{}",
        stdout
    );
}

fn process_assembly_result(output: &Output, class_file: &TempClassFile) -> (String, String) {
    if output.status.success() {
        let hash = get_hash(class_file.path());

        let disassembled = {
            let mut cmd = cargo_bin_cmd!("rnsc");
            cmd.arg("dis").arg(class_file.path());
            let dis_output = cmd.output().expect("Failed to execute disassemble");
            String::from_utf8(dis_output.stdout).expect("Failed to read disassemble output")
        };
        (disassembled, hash)
    } else {
        ("not generated".to_string(), "not generated".to_string())
    }
}

fn verify_assembly_behavior(
    disassembled: &str,
    hash: &str,
    output: &Output,
    class_file: &TempClassFile,
) {
    if disassembled == "not generated" && hash == "not generated" {
        assert!(
            !output.status.success(),
            "Expected non-zero exit code when 'not generated' appears"
        );
        assert!(
            !class_file.path().exists(),
            "Class file should not exist when assembly fails"
        );
    } else {
        assert!(
            output.status.success(),
            "Expected zero exit code when class is generated"
        );
        let file_size = fs::metadata(class_file.path())
            .expect("Class file should exist on success")
            .len();
        assert!(file_size > 0, "Class file should have content on success");
    }
}

fn create_snapshot_content(
    disassembled: &str,
    input_path: &Path,
    stderr: &str,
    hash: &str,
) -> String {
    let original_contents = get_file_contents(input_path);
    format!(
        "----- DISASSEMBLED -----\n{}\n----- INPUT -----\n{}\n----- STDERR -----\n{}\n----- HASH -----\n{}",
        disassembled.trim_end(),
        original_contents.trim_end(),
        stderr.trim_end(),
        hash
    )
}

struct TempClassFile(PathBuf);

impl TempClassFile {
    fn new(source_path: &Path) -> Self {
        let temp_dir = std::env::temp_dir();
        let stem = source_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        let class_file = temp_dir.join(format!(
            "test_output_{}_{}.class",
            stem,
            rand::random::<u16>()
        ));
        fs::remove_file(&class_file).ok();
        Self(class_file)
    }

    fn path(&self) -> &PathBuf {
        &self.0
    }
}

impl Drop for TempClassFile {
    fn drop(&mut self) {
        fs::remove_file(&self.0).ok();
    }
}

#[rstest]
fn test_integration(
    #[base_dir = "test_data/integration/"]
    #[files("**/*.rns")]
    path: PathBuf,
) {
    let class_file = TempClassFile::new(&path);

    let assemble_output = run_assemble_command(&path, class_file.path());

    let stdout = String::from_utf8_lossy(&assemble_output.stdout);
    let stderr = String::from_utf8_lossy(&assemble_output.stderr);

    assert_stdout_empty(&stdout);

    let (disassembled, hash) = process_assembly_result(&assemble_output, &class_file);
    verify_assembly_behavior(&disassembled, &hash, &assemble_output, &class_file);

    let normalized_stderr = normalize_stderr_paths(&stderr, &path);
    let snapshot_content = create_snapshot_content(&disassembled, &path, &normalized_stderr, &hash);

    with_settings!(
        {
            snapshot_path => SNAPSHOT_PATH,
            prepend_module_to_snapshot => false,
        },
        {
            insta::assert_snapshot!(to_snapshot_name(&path), &snapshot_content);
        }
    );
}
