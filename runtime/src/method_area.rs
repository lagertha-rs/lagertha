use crate::class_loader::BootstrapClassLoader;
use crate::rt::class::class::Class;
use crate::JvmError;
use dashmap::DashMap;
use std::rc::Rc;

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.5.4
pub struct MethodArea {
    bootstrap_class_loader: BootstrapClassLoader,
    classes: DashMap<String, Rc<Class>>,
}

impl MethodArea {
    pub fn with_main(main: Rc<Class>) -> Result<Self, JvmError> {
        let mut instance = Self::new()?;
        instance.classes.insert(main.get_name()?.to_string(), main);
        Ok(instance)
    }
    pub fn new() -> Result<Self, JvmError> {
        Ok(Self {
            bootstrap_class_loader: BootstrapClassLoader::new()?,
            classes: DashMap::new(),
        })
    }

    pub fn get_class(&self, name: &String) -> Result<Rc<Class>, JvmError> {
        let class = Rc::new(self.bootstrap_class_loader.try_load(name)?);

        self.classes.insert(name.clone(), class.clone());

        Ok(class)
    }
}
