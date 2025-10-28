use crate::class_loader::ClassLoader;
use crate::rt::class_deprecated::ClassDeprecated;
use crate::rt::method_deprecated::MethodDeprecated;
use crate::{ClassIdDeprecated, MethodIdDeprecated, VmConfig};
use common::error::{JvmError, LinkageError};
use common::instruction::ArrayType;
use jclass::ClassFile;
use lasso::ThreadedRodeo;
use std::collections::HashMap;
use std::sync::Arc;
use tracing_log::log::debug;

//TODO: finally need to decide to return Arc<Class> or &Arc<Class>
//TODO: the class loading process is working stub, need to be improved and respect the spec
/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.5.4
pub struct MethodArea {
    bootstrap_class_loader: ClassLoader,
    classes: HashMap<ClassIdDeprecated, Arc<ClassDeprecated>>,
    methods: Vec<MethodDeprecated>,
    string_interner: Arc<ThreadedRodeo>,
}

impl MethodArea {
    pub fn new(
        vm_config: &VmConfig,
        string_interner: Arc<ThreadedRodeo>,
    ) -> Result<Self, JvmError> {
        debug!("Initializing MethodArea...");
        let bootstrap_class_loader = ClassLoader::new(vm_config)?;
        let method_area = Self {
            methods: Vec::new(),
            classes: HashMap::new(),
            bootstrap_class_loader,
            string_interner,
        };

        debug!("MethodArea initialized");
        Ok(method_area)
    }

    pub fn get_class_or_load_by_name(
        &mut self,
        name: &str,
    ) -> Result<&Arc<ClassDeprecated>, JvmError> {
        let id = self.string_interner.get_or_intern(name);
        if self.classes.contains_key(&id) {
            return Ok(self.classes.get(&id).unwrap());
        }
        if name.starts_with("[") {
            self.create_array_class(id, name)
        } else {
            let class_data = self.bootstrap_class_loader.load(name)?;
            self.add_raw_bytecode(id, class_data)
        }
    }

    // assumes if there is a class_id, the class must be loaded
    pub fn get_class_by_id(
        &self,
        class_id: &ClassIdDeprecated,
    ) -> Result<&Arc<ClassDeprecated>, JvmError> {
        self.classes.get(class_id).ok_or(JvmError::ClassNotFound(
            self.string_interner.resolve(class_id).to_string(),
        ))
    }

    pub fn add_raw_bytecode(
        &mut self,
        id: ClassIdDeprecated,
        data: Vec<u8>,
    ) -> Result<&Arc<ClassDeprecated>, JvmError> {
        let cf = ClassFile::try_from(data).map_err(LinkageError::from)?;
        let class = ClassDeprecated::new(id, cf, self)?;
        self.add_class(class)
    }

    fn add_class(
        &mut self,
        class: Arc<ClassDeprecated>,
    ) -> Result<&Arc<ClassDeprecated>, JvmError> {
        let class_id = *class.id();
        self.classes.insert(class_id, class);
        self.get_class_by_id(&class_id)
    }

    fn create_array_class(
        &mut self,
        id: ClassIdDeprecated,
        name: &str,
    ) -> Result<&Arc<ClassDeprecated>, JvmError> {
        let class = if let Ok(primitive) = ArrayType::try_from(name) {
            ClassDeprecated::new_primitive_array(id, primitive)?
        } else {
            ClassDeprecated::new_array(id, name)?
        };
        self.add_class(class)
    }

    pub fn get_class_id(&mut self, name: &str) -> Result<ClassIdDeprecated, JvmError> {
        let class = self.get_class_or_load_by_name(name)?;
        Ok(*class.id())
    }
}
