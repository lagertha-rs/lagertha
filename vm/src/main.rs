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
    #[arg(
        help = "Main class to run from path that matches the package structure \
        (e.g. com.example.Main or com/example/Main for com/example/Main.class)"
    )]
    pub main_class_path: String,
}

fn create_vm_configuration(args: Args) -> Result<VmConfig, String> {
    let java_home = std::env::var("JAVA_HOME").expect("JAVA_HOME not set");
    let home = std::path::PathBuf::from(&java_home);
    let release_file = format!("{}/release", java_home);

    let contents = std::fs::read_to_string(release_file).expect("cannot read release file");

    for line in contents.lines() {
        if let Some(value) = line.strip_prefix("JAVA_VERSION=") {
            return Ok(VmConfig {
                home,
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
    let current_path = std::env::current_dir().unwrap();
    eprintln!("{}", current_path.display());
    //init_tracing();
    let args = Args::parse();
    debug!("Provided command line arguments: {:?}", args);

    let package_like_main_path = args.main_class_path.replace('/', ".");
    let main_class = package_like_main_path.replace('.', "/") + ".class";

    debug!("Trying to open class file: {}", main_class);

    let jclass_bytes = std::fs::read(&main_class);
    match jclass_bytes {
        Ok(bytes) => {
            debug!("Class file read successfully, size: {} bytes", bytes.len());
            let vm_config = match create_vm_configuration(args) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Error creating VM configuration: {}", e);
                    return;
                }
            };
            if let Err(err) = runtime::start(&package_like_main_path, bytes, vm_config) {
                error!("VM execution failed: {err}");
                std::process::exit(1);
            }
        }
        Err(_) => {
            eprintln!(
                "Error: Could not find or load main class {}\n\
                 Caused by: java.lang.ClassNotFoundException: {}",
                package_like_main_path, package_like_main_path
            );
        }
    }
}
