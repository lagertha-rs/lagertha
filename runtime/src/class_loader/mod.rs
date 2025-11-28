use crate::class_loader::system::SystemClassLoader;
use crate::{VmConfig, debug_log};
use common::error::JvmError;
use jimage::JImage;
use std::collections::HashSet;
use std::path::PathBuf;
use toml::Value;

mod system;

// TODO: It is more like a stub for now, need to respect the doc

#[derive(Debug, Clone)]
struct ClassSource {
    jmod_path: PathBuf,
    entry_name: String,
}

/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.3.1
pub struct ClassLoader {
    tested_parsing: HashSet<String>,
    jimage: JImage,
    system: SystemClassLoader,
}

impl ClassLoader {
    //TODO: remove afterwards, it forces whatever is loaded to be present in fixtures.toml (jclass tested)
    fn prepare_tested_classes() -> HashSet<String> {
        let fixtures_toml = include_str!("../../../javap/tests/testdata/fixtures.toml");
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

    pub fn new(vm_config: &VmConfig) -> Result<Self, JvmError> {
        debug_log!("Creating ClassLoader...");
        let modules_path = &vm_config.home.join("lib").join("modules");
        debug_log!("Loading JImage from path: {:?}", modules_path);
        let jimage = JImage::new(modules_path);
        debug_log!(
            "Loading SystemClassLoader from classpath: {:?}",
            vm_config.class_path
        );
        let system_loader = SystemClassLoader::new(&vm_config.class_path)?;
        Ok(Self {
            jimage,
            system: system_loader,
            tested_parsing: Self::prepare_tested_classes(),
        })
    }

    // TODO: Can return &[u8] to avoid copying, but need to support proper SystemClassLoader first
    pub fn load(&self, name: &str) -> Result<Vec<u8>, JvmError> {
        if let Some(bytes) = self.jimage.open_java_base_class(name) {
            debug_log!("Bytecode of \"{name}\" found using JImage.");
            // TODO: remove afterwards
            assert!(
                self.tested_parsing.contains(name),
                "Class \"{}\" is not tested",
                name
            );
            Ok(bytes.to_vec())
        } else {
            let bytes = self.system.find_class(name)?;
            debug_log!("Bytecode of \"{name}\" found using SystemClassLoader.",);
            Ok(bytes)
        }
    }
}
