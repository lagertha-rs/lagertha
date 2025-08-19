use crate::class_file::ClassFile;
use crate::class_loader::jmod::BootstrapJmodLoader;
use crate::rt::class::class::Class;
use crate::JvmError;
use std::env;
use thiserror::Error;

mod jmod;

#[derive(Debug, Error)]
pub enum ClassLoaderErr {
    #[error("")]
    JavaHomeIsNotSet,
    #[error("")]
    CanNotAccessSource,
    #[error("")]
    ClassNotFound,
    #[error("")]
    ArchiveErr,
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-5.html#jvms-5.3.1
pub struct BootstrapClassLoader {
    jmod_loader: BootstrapJmodLoader,
}

impl BootstrapClassLoader {
    pub fn new() -> Result<Self, ClassLoaderErr> {
        let java_home = env::var("JAVA_HOME").map_err(|_| ClassLoaderErr::JavaHomeIsNotSet)?;
        let jmods_dir = format!("{}/jmods", java_home);
        let jmod_loader = BootstrapJmodLoader::from_jmods_dir(jmods_dir)?;
        Ok(Self { jmod_loader })
    }

    pub fn try_load(&self, name: &String) -> Result<Class, JvmError> {
        let raw = self.jmod_loader.find_class(name)?;
        let class_file = ClassFile::try_from(raw)?;
        Class::new(class_file)
    }
}
