use crate::JvmError;
use crate::method_area::MethodArea;
use crate::rt::class::field::{Field, StaticField};
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::constant_pool::reference::{ClassReference, NameAndTypeReference};
use crate::rt::method::java::Method;
use crate::rt::method::native::NativeMethod;
use crate::rt::method::{StaticMethodType, VirtualMethodType};
use class_file::ClassFile;
use class_file::attribute::class::ClassAttribute;
use class_file::flags::ClassFlags;
use common::descriptor;
use common::jtype::TypeValue;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

#[derive(Debug)]
pub struct Class {
    this: Arc<ClassReference>,
    access: ClassFlags,
    minor_version: u16,
    major_version: u16,
    super_class: Option<Arc<Class>>,
    fields: Vec<Field>,
    // TODO: TBD hashmap key type
    static_fields: HashMap<(Arc<String>, Arc<String>), StaticField>,
    // TODO: probably use hashmap for methods with method name+descriptor as key. TBD when execute instructions
    methods: Vec<VirtualMethodType>,
    static_methods: Vec<StaticMethodType>,
    constructors: Vec<Method>,
    initializer: Option<Method>,
    interfaces: Vec<String>,
    attributes: Vec<ClassAttribute>,
    cp: Arc<RuntimeConstantPool>,
    initialized: OnceLock<()>,
}

impl Class {
    pub fn new(cf: ClassFile, method_area: &MethodArea) -> Result<Self, JvmError> {
        let cp = Arc::new(RuntimeConstantPool::new(cf.cp.inner));
        let minor_version = cf.minor_version;
        let major_version = cf.major_version;
        let this = cp.get_class(&cf.this_class)?.clone();
        let access = cf.access_flags;
        let mut methods = vec![];
        let mut static_methods = vec![];
        let mut constructors = vec![];
        let mut initializer = None;
        for method in cf.methods {
            let flags = method.access_flags;
            let name = cp.get_utf8(&method.name_index)?.as_str();

            match (flags.is_native(), flags.is_static()) {
                (true, true) => {
                    static_methods.push(StaticMethodType::Native(NativeMethod::new(method, &cp)?))
                }
                (true, false) => {
                    methods.push(VirtualMethodType::Native(NativeMethod::new(method, &cp)?))
                }
                (false, true) => {
                    if name == "<clinit>" {
                        initializer = Some(Method::new(method, &cp)?);
                    } else {
                        static_methods.push(StaticMethodType::Java(Method::new(method, &cp)?));
                    }
                }
                (false, false) => {
                    if name == "<init>" {
                        constructors.push(Method::new(method, &cp)?);
                    } else {
                        methods.push(VirtualMethodType::Java(Method::new(method, &cp)?));
                    }
                }
            }
        }
        let mut static_fields = HashMap::new();
        let mut fields = vec![];

        for field in cf.fields {
            if field.access_flags.is_static() {
                let name = cp.get_utf8(&field.name_index)?.clone();
                let descriptor = cp.get_utf8(&field.descriptor_index)?.clone();
                static_fields.insert((name, descriptor), StaticField::new(field, &cp)?);
            } else {
                fields.push(Field::new(field, &cp)?)
            }
        }
        let super_class = if cf.super_class != 0 {
            Some(method_area.get_class(cp.get_class_name(&cf.super_class)?)?)
        } else {
            None
        };

        let initialized = if initializer.is_some() {
            OnceLock::new()
        } else {
            let lock = OnceLock::new();
            let _ = lock.set(());
            lock
        };

        Ok(Class {
            this,
            access,
            super_class,
            minor_version,
            major_version,
            fields,
            static_fields,
            static_methods,
            methods,
            constructors,
            initializer,
            interfaces: vec![],
            attributes: cf.attributes,
            cp,
            initialized,
        })
    }

    pub fn name(&self) -> Result<&Arc<String>, JvmError> {
        self.this.name().map_err(Into::into)
    }

    pub fn find_main_method(&self) -> Option<&Method> {
        self.static_methods
            .iter()
            .find(|m| m.is_main())
            .and_then(|m| match m {
                StaticMethodType::Java(m) => Some(m),
                StaticMethodType::Native(_) => None,
            })
    }

    pub fn cp(&self) -> &Arc<RuntimeConstantPool> {
        &self.cp
    }

    pub fn initialized(&self) -> bool {
        self.initialized.get().is_some()
    }

    pub fn set_initialized(&self) {
        let _ = self.initialized.set(());
    }

    pub fn super_class(&self) -> Option<&Arc<Class>> {
        self.super_class.as_ref()
    }

    pub fn initializer(&self) -> Option<&Method> {
        self.initializer.as_ref()
    }

    /// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-5.html#jvms-5.4.3.2
    pub fn set_static_field(
        &self,
        nat: &NameAndTypeReference,
        value: TypeValue,
    ) -> Result<(), JvmError> {
        let name = nat.name()?;
        let descriptor = nat.field_descriptor()?.raw();

        if let Some(field) = self.static_fields.get(&(name.clone(), descriptor.clone())) {
            field.set_value(value)?;
        } else {
            todo!()
        }

        /* TODO:
        NoSuchFieldError,
        IncompatibleClassChangeError,
        IllegalAccessError
        etc...
         */
        Ok(())
    }
}
