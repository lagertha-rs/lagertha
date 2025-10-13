use clap::Parser;
use common::utils::telemetry::init_tracing;
use runtime::VmConfig;
use tracing_log::log::{debug, error};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(
        short = 'c',
        long = "classpath",
        visible_alias = "cp",
        visible_alias = "class-path",
        value_delimiter = ';',
        help = "Classpath entries (only dirs, no jars(todo)); use ';' as separator"
    )]
    pub class_path: Vec<String>,
    #[arg(help = "Main class to run")]
    pub main_class: String,
}

fn create_vm_configuration(args: Args) -> Result<VmConfig, String> {
    let java_home = std::env::var("JAVA_HOME").expect("JAVA_HOME not set");
    let release_file = format!("{}/release", java_home);

    let contents = std::fs::read_to_string(release_file).expect("cannot read release file");

    for line in contents.lines() {
        if let Some(value) = line.strip_prefix("JAVA_VERSION=") {
            return Ok(VmConfig {
                home: java_home,
                version: value.trim_matches('"').to_string(),
                class_path: args.class_path,
                initial_heap_size: 0,
                max_heap_size: 0,
                frame_stack_size: 256,
                operand_stack_size: 256,
            });
        }
    }
    Err("JAVA_VERSION not found in release file".to_string())
}

fn main() {
    init_tracing();
    let args = Args::parse();
    debug!("Provided command line arguments: {:?}", args);
    debug!("Trying to open class file: {}", args.main_class);
    let class_file_bytes = std::fs::read(&args.main_class);
    match class_file_bytes {
        Ok(bytes) => {
            debug!("Class file read successfully, size: {} bytes", bytes.len());
            let vm_config = match create_vm_configuration(args) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Error creating VM configuration: {}", e);
                    return;
                }
            };
            if let Err(err) = runtime::start(bytes, vm_config) {
                error!("VM execution failed: {err}");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to read class file {}: {}", args.main_class, e);
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::with_settings;
    use rstest::rstest;
    use std::path::{Path, PathBuf};

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
        let mut cmd = assert_cmd::Command::cargo_bin("vm").unwrap();
        cmd.args([&path]);

        // when
        let out = cmd.assert().success().get_output().stdout.clone();
        let out_str = String::from_utf8_lossy(&out);

        // then
        with_settings!(
            {
                snapshot_path => DISPLAY_SNAPSHOT_PATH,
                prepend_module_to_snapshot => false,
            },
            {
                insta::assert_snapshot!(to_snapshot_name(&path), &out_str);
            }
        );
    }

    #[rstest]
    #[trace]
    fn error_cases(
        #[base_dir = "../target/test-classes/vm"]
        #[files("**/*ExceptionMain.class")]
        path: PathBuf,
    ) {
        // given
        // requires cargo build
        let mut cmd = assert_cmd::Command::cargo_bin("vm").unwrap();
        cmd.args([&path]);

        // when
        let err = cmd.assert().success().get_output().stderr.clone();
        let err_str = String::from_utf8_lossy(&err);

        // then
        with_settings!(
            {
                snapshot_path => DISPLAY_SNAPSHOT_PATH,
                prepend_module_to_snapshot => false,
            },
            {
                insta::assert_snapshot!(to_snapshot_name(&path), &err_str);
            }
        );
    }
}
