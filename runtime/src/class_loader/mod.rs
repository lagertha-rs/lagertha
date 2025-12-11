use crate::class_loader::system::SystemClassLoader;
use crate::error::JvmError;
use crate::{VmConfig, debug_log};
use jimage::JImage;
use std::path::PathBuf;
//use toml::Value;
//use toml_edit::Document;

mod system;

// TODO: It is more like a stub for now, need to respect the doc

#[derive(Debug, Clone)]
struct ClassSource {
    jmod_path: PathBuf,
    entry_name: String,
}

/// https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.3.1

pub struct ClassLoader {
    jimage: JImage,
    system: SystemClassLoader,
    //fixtures_path: PathBuf,
}

impl ClassLoader {
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

        //let fixtures_path = PathBuf::from("javap/tests/testdata/fixtures.toml");

        Ok(Self {
            jimage,
            system: system_loader,
            //fixtures_path,
        })
    }

    #[hotpath::measure]
    pub fn load(&self, name: &str) -> Result<Vec<u8>, JvmError> {
        if let Some(bytes) = self.jimage.open_java_base_class(name) {
            debug_log!("Bytecode of \"{name}\" found using JImage.");
            //self.add_tested_class(name)?;
            Ok(bytes.to_vec())
        } else {
            let bytes = self.system.find_class(name)?;
            debug_log!("Bytecode of \"{name}\" found using SystemClassLoader.");
            Ok(bytes)
        }
    }

    /*
    fn add_tested_class(&self, name: &str) -> Result<(), JvmError> {
        let content = std::fs::read_to_string(&self.fixtures_path).unwrap();

        let mut doc: Document = content.parse().unwrap();

        let classes = doc
            .entry("modules")
            .or_insert_with(toml_edit::table)
            .as_table_mut()
            .unwrap()
            .entry("java.base")
            .or_insert_with(toml_edit::table)
            .as_table_mut()
            .unwrap()
            .entry("classes")
            .or_insert_with(toml_edit::array)
            .as_array_mut()
            .unwrap();

        if !classes.iter().any(|item| item.as_str() == Some(name)) {
            classes.push(name);
            debug_log!("Added class \"{}\" to fixtures.toml", name);
        }

        std::fs::write(&self.fixtures_path, doc.to_string()).unwrap();

        Ok(())
    }
     */
}
