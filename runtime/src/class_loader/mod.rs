use crate::class_loader::jmod::BootstrapJmodLoader;
use crate::rt::class::LinkageError;
use crate::rt::class::class::Class;
use crate::{JvmError, VM_CONFIGURATION};
use class_file::ClassFile;
use std::env;
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

const BOOTSTRAP_CLASS_LOADER_NAME: &str = "BootstrapClassLoader";

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-5.html#jvms-5.3.1
#[derive(Debug)]
pub struct BootstrapClassLoader {
    jmod_loader: BootstrapJmodLoader,
    name: Arc<String>, // TODO: spec says I need it. Check it later
}

impl BootstrapClassLoader {
    pub fn new() -> Result<Self, ClassLoaderErr> {
        debug!("Creating BootstrapClassLoader...");
        let java_home = &VM_CONFIGURATION.get().unwrap().home;
        let jmods_dir = format!("{}/jmods", java_home);
        debug!(
            "Using JAVA_HOME at \"{}\" -> using jmods dir \"{}\"",
            java_home, jmods_dir
        );
        let jmod_loader = BootstrapJmodLoader::from_jmods_dir(jmods_dir)?;
        Ok(Self {
            jmod_loader,
            name: Arc::new(BOOTSTRAP_CLASS_LOADER_NAME.to_string()),
        })
    }

    pub fn load_with_bytes(&self, data: Vec<u8>) -> Result<Class, JvmError> {
        let cf = ClassFile::try_from(data).map_err(LinkageError::from)?;
        let class = Class::new(cf, self.name.clone())?;
        Ok(class)
    }

    pub fn load(&self, name: &str) -> Result<Class, JvmError> {
        debug!("BootstrapClassLoader: Loading class \"{}\"...", name);
        let data = self.jmod_loader.find_class(name)?;
        self.load_with_bytes(data)
    }
}
