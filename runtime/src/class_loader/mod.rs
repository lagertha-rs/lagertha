use crate::VmConfig;
use crate::class_loader::jmod::BootstrapJmodClassLoader;
use crate::class_loader::system::SystemClassLoader;
use crate::error::JvmError;
use std::path::PathBuf;
use thiserror::Error;
use tracing_log::log::debug;

mod jmod;
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
#[derive(Debug)]
pub struct ClassLoader {
    bootstrap: BootstrapJmodClassLoader,
    system: SystemClassLoader,
}

impl ClassLoader {
    pub fn new(vm_config: &VmConfig) -> Result<Self, ClassLoaderErr> {
        debug!("Creating BootstrapClassLoader...");
        let java_home = &vm_config.home;
        let jmods_dir = format!("{}/jmods", java_home);
        debug!(
            "Using JAVA_HOME at \"{}\" -> using jmods dir \"{}\"",
            java_home, jmods_dir
        );
        let jmod_loader = BootstrapJmodClassLoader::from_jmods_dir(jmods_dir)?;
        debug!("Creating SystemClassLoader...");
        let system_loader = SystemClassLoader::new(&vm_config.class_path)?;
        Ok(Self {
            bootstrap: jmod_loader,
            system: system_loader,
        })
    }

    pub fn load(&self, name: &str) -> Result<Vec<u8>, JvmError> {
        debug!(r#"Searching for bytecode of "{name}"..."#);

        match self.bootstrap.find_class(name) {
            Ok(bytes) => {
                debug!(
                    r#"Bytecode of "{name}" found in BootstrapClassLoader. Read {} bytes."#,
                    bytes.len()
                );
                return Ok(bytes);
            }
            Err(ClassLoaderErr::ClassNotFound(_)) => {
                debug!(
                    r#"Class "{name}" not found in BootstrapClassLoader, trying SystemClassLoader..."#
                );
            }
            Err(e) => return Err(e.into()),
        }

        let bytes = self.system.find_class(name)?;
        debug!(
            r#"Bytecode of "{name}" found in SystemClassLoader. Read {} bytes."#,
            bytes.len()
        );
        Ok(bytes)
    }
}
