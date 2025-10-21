use crate::VmConfig;
use crate::class_loader::system::SystemClassLoader;
use crate::error::JvmError;
use jimage::JImage;
use std::collections::HashSet;
use std::path::PathBuf;
use thiserror::Error;
use toml::Value;
use tracing_log::log::debug;

mod system;

// TODO: It is more like a stub for now, need to respect the doc

#[derive(Debug, Error)]
pub enum ClassLoaderErr {
    #[error("JavaHomeIsNotSet")]
    JavaHomeIsNotSet,
    #[error("CanNotAccessSource")]
    CanNotAccessSource,
    #[error("ClassNotFoundException: {0}")]
    ClassNotFound(String),
    #[error("ArchiveErr")]
    ArchiveErr,
}

#[derive(Debug, Clone)]
struct ClassSource {
    jmod_path: PathBuf,
    entry_name: String,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-5.html#jvms-5.3.1
pub struct ClassLoader {
    tested_parsing: HashSet<String>,
    jimage: JImage,
    system: SystemClassLoader,
}

impl ClassLoader {
    //TODO: remove afterwards, it forces whatever is loaded to be present in fixtures.toml (jclass tested)
    fn prepare_tested_classes() -> HashSet<String> {
        let fixtures_toml = include_str!("../../../jclass/fixtures.toml");
        let root_value: Value = fixtures_toml.parse().expect("Failed to parse TOML");

        let classes_set: HashSet<String> = root_value
            .get("modules")
            .and_then(|modules| modules.get("java.base"))
            .and_then(|java_base| java_base.get("classes"))
            .and_then(|classes_value| classes_value.as_array())
            .map(|classes_array| {
                classes_array
                    .iter()
                    .filter_map(|val| val.as_str())
                    .map(|v| v.replace('.', "/"))
                    .collect()
            })
            .unwrap();

        classes_set
    }

    pub fn new(vm_config: &VmConfig) -> Result<Self, ClassLoaderErr> {
        debug!("Creating BootstrapClassLoader...");
        let java_home = &vm_config.home;
        let jimage = JImage::new(java_home.join("lib").join("modules"));
        debug!("Creating SystemClassLoader...");
        let system_loader = SystemClassLoader::new(&vm_config.class_path)?;
        Ok(Self {
            jimage,
            system: system_loader,
            tested_parsing: Self::prepare_tested_classes(),
        })
    }

    // TODO: Can return &[u8] to avoid copying, but need to support proper SystemClassLoader first
    pub fn load(&self, name: &str) -> Result<Vec<u8>, JvmError> {
        debug!(r#"Searching for bytecode of "{name}"..."#);

        if let Some(bytes) = self.jimage.open_java_base_class(name) {
            debug!(
                r#"Bytecode of "{name}" found in JImage. Read {} bytes."#,
                bytes.len()
            );
            // TODO: remove afterwards
            assert!(
                self.tested_parsing.contains(name),
                "Class \"{}\" is not tested",
                name
            );
            Ok(bytes.to_vec())
        } else {
            let bytes = self.system.find_class(name)?;
            debug!(
                r#"Bytecode of "{name}" found in SystemClassLoader. Read {} bytes."#,
                bytes.len()
            );
            Ok(bytes)
        }
    }
}
