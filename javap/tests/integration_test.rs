use assert_cmd::Command;
use rstest::*;
use std::io::BufRead;
use std::path::PathBuf;

#[rstest]
fn compare_with_javap(
    #[base_dir = "tests/testdata/compiled"]
    #[files("**/*.class")]
    path: PathBuf,
) {
    let mut cmd = Command::cargo_bin("javap").unwrap();
    cmd.arg(&path);

    let output = cmd.assert().success().get_output().clone();
    let my_output = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    let javap_output = std::process::Command::new("javap")
        .arg("-v")
        .arg("-p")
        .arg(&path)
        .output()
        .unwrap_or_else(|e| panic!("Can't run javap on file {:?}:{}\n", path, e))
        .stdout
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // When && Then
    for (i, (my, javap)) in my_output.iter().zip(javap_output.iter()).enumerate() {
        let my_line_normalized: String = my.chars().filter(|c| !c.is_whitespace()).collect();
        let javap_line_normalized: String = javap.chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!(
            my_line_normalized,
            javap_line_normalized,
            "Mismatch at line {} of file {:?}. My line: {:?}, javap line: {:?}",
            i + 1,
            path,
            my,
            javap
        );
    }
}
