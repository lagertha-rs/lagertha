use crate::class_loader::jmod::BootstrapJmodLoader;
use crate::{JvmError, VmConfig};
use std::sync::Arc;
use thiserror::Error;
use tracing_log::log::debug;

mod jmod;

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

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-5.html#jvms-5.3.1
#[derive(Debug)]
pub struct BootstrapClassLoader {
    vm_config: Arc<VmConfig>,
    jmod_loader: BootstrapJmodLoader,
}

impl BootstrapClassLoader {
    pub fn new(vm_config: Arc<VmConfig>) -> Result<Self, ClassLoaderErr> {
        debug!("Creating BootstrapClassLoader...");
        let java_home = &vm_config.home;
        let jmods_dir = format!("{}/jmods", java_home);
        debug!(
            "Using JAVA_HOME at \"{}\" -> using jmods dir \"{}\"",
            java_home, jmods_dir
        );
        let jmod_loader = BootstrapJmodLoader::from_jmods_dir(jmods_dir)?;
        Ok(Self {
            vm_config,
            jmod_loader,
        })
    }

    pub fn load(&self, name: &str) -> Result<Vec<u8>, JvmError> {
        debug!("Searching for bytecode of \"{}\"...", name);
        let data = self.jmod_loader.find_class(name)?;
        debug!("Bytecode of \"{}\" found. Read {} bytes.", name, data.len());
        Ok(data)
    }
}
