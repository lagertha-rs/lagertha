use clap::Parser;
use runtime::VmConfig;
use tracing_log::log::debug;

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

fn create_vm_configuration(mut args: Args, main_class: String) -> Result<VmConfig, String> {
    let java_home = std::env::var("JAVA_HOME").expect("JAVA_HOME not set");
    if args.class_path.is_empty() {
        let current_dir = std::env::current_dir()
            .map(|v| v.to_string_lossy().to_string())
            .expect("cannot get current dir");
        args.class_path.push(current_dir);
    }
    let home = std::path::PathBuf::from(&java_home);
    let release_file = format!("{}/release", java_home);

    let contents = std::fs::read_to_string(release_file).expect("cannot read release file");

    for line in contents.lines() {
        if let Some(value) = line.strip_prefix("JAVA_VERSION=") {
            return Ok(VmConfig {
                home,
                main_class,
                version: value.trim_matches('"').to_string(),
                class_path: args.class_path,
                initial_heap_size: 0,
                max_heap_size: 0,
                frame_stack_size: 256,
            });
        }
    }
    Err("JAVA_VERSION not found in release file".to_string())
}

fn main() {
    #[cfg(feature = "log-runtime-traces")]
    common::utils::telemetry::init_tracing();
    let args = Args::parse();
    debug!("Provided command line arguments: {:?}", args);

    let main_class = args.main_class_path.replace('.', "/");

    let vm_config = match create_vm_configuration(args, main_class) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error creating VM configuration: {}", e);
            return;
        }
    };
    if let Err(_) = runtime::start(vm_config) {
        std::process::exit(1);
    }
}
