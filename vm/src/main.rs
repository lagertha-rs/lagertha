use clap::Parser;
use common::utils::telemetry::init_tracing;
use runtime::VmConfig;
use tracing_log::log::debug;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, help = "Path to the main class file to execute")]
    class_file: String,
}

fn create_vm_configuration() -> Result<VmConfig, String> {
    let java_home = std::env::var("JAVA_HOME").expect("JAVA_HOME not set");
    let release_file = format!("{}/release", java_home);

    let contents = std::fs::read_to_string(release_file).expect("cannot read release file");

    for line in contents.lines() {
        if let Some(value) = line.strip_prefix("JAVA_VERSION=") {
            return Ok(VmConfig {
                home: java_home,
                version: value.trim_matches('"').to_string(),
                initial_heap_size: 0,
                max_heap_size: 0,
                stack_size_per_thread: 0,
            });
        }
    }
    Err("JAVA_VERSION not found in release file".to_string())
}

fn main() {
    init_tracing();
    let args = Args::parse();
    debug!("Provided command line arguments: {:?}", args);
    debug!("Trying to open class file: {}", args.class_file);
    let class_file_bytes = std::fs::read(&args.class_file);
    match class_file_bytes {
        Ok(bytes) => {
            debug!("Class file read successfully, size: {} bytes", bytes.len());
            let vm_config = match create_vm_configuration() {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Error creating VM configuration: {}", e);
                    return;
                }
            };
            if let Err(e) = runtime::start(bytes, vm_config) {
                eprintln!("Error during JVM execution: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to read class file {}: {}", args.class_file, e);
        }
    }
}
