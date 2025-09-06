use crate::JvmError;
use crate::class_loader::BootstrapClassLoader;
use crate::rt::class::class::Class;
use dashmap::DashMap;
use std::sync::Arc;
use tracing_log::log::debug;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.5.4
#[derive(Debug)]
pub struct MethodArea {
    bootstrap_class_loader: BootstrapClassLoader,
    classes: DashMap<String, Arc<Class>>,
    main: Arc<Class>,
}

impl MethodArea {
    pub fn try_with_main(main: Vec<u8>) -> Result<Self, JvmError> {
        let bootstrap_class_loader = BootstrapClassLoader::new()?;
        debug!("Loading main class from bytes...");
        let main_class = Arc::new(bootstrap_class_loader.load_with_bytes(main)?);

        let classes = DashMap::new();
        classes.insert(main_class.get_name()?.to_string(), main_class.clone());
        debug!(
            "MethodArea initialized with main class \"{}\"",
            main_class.get_name()?
        );
        Ok(Self {
            classes,
            main: main_class,
            bootstrap_class_loader,
        })
    }

    pub fn get_main(&self) -> Arc<Class> {
        self.main.clone()
    }

    pub fn get_class(&self, name: &str) -> Result<Arc<Class>, JvmError> {
        if let Some(class) = self.classes.get(name) {
            return Ok(class.clone());
        }
        let class = Arc::new(self.bootstrap_class_loader.load(name)?);

        self.classes.insert(name.to_string(), class.clone());

        Ok(class)
    }
}
