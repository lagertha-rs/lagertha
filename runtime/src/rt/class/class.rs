use crate::JvmError;
use crate::method_area::MethodArea;
use crate::rt::class::field::{Field, StaticField};
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::constant_pool::reference::{ClassReference, MethodReference, NameAndTypeReference};
use crate::rt::method::java::Method;
use crate::rt::method::native::NativeMethod;
use crate::rt::method::{StaticMethodType, VirtualMethodType};
use class_file::ClassFile;
use class_file::attribute::class::ClassAttribute;
use class_file::flags::ClassFlags;
use common::jtype::TypeValue;
use parking_lot::RwLock;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Eq, PartialEq)]
pub enum InitState {
    NotInitialized,
    Initializing,
    Initialized,
    // TODO: Failed(Throwable) ??
}

pub struct Class {
    this: Arc<ClassReference>,
    access: ClassFlags,
    minor_version: u16,
    major_version: u16,
    super_class: Option<Arc<Class>>,
    fields: Vec<Field>,
    // TODO: TBD hashmap check key type
    static_fields: HashMap<(Arc<String>, Arc<String>), StaticField>,
    // TODO: probably use hashmap for methods with method name+descriptor as key. TBD when execute instructions
    methods: Vec<VirtualMethodType>,
    // TODO: TBD hashmap check key type
    static_methods: HashMap<(Arc<String>, Arc<String>), StaticMethodType>,
    constructors: Vec<Method>,
    initializer: Option<Method>,
    interfaces: Vec<String>,
    attributes: Vec<ClassAttribute>,
    cp: Arc<RuntimeConstantPool>,
    state: RwLock<InitState>,
}

impl Class {
    pub fn new(cf: ClassFile, method_area: &MethodArea) -> Result<Self, JvmError> {
        let cp = Arc::new(RuntimeConstantPool::new(cf.cp.inner));
        let minor_version = cf.minor_version;
        let major_version = cf.major_version;
        let this = cp.get_class(&cf.this_class)?.clone();
        let access = cf.access_flags;
        let mut methods = vec![];
        let mut static_methods = HashMap::new();
        let mut constructors = vec![];
        let mut initializer = None;
        for method in cf.methods {
            let flags = method.access_flags;
            let name = cp.get_utf8(&method.name_index)?.as_str();

            match (flags.is_native(), flags.is_static()) {
                (true, true) => {
                    let method = NativeMethod::new(method, &cp)?;
                    let name = method.name().clone();
                    let descriptor = method.descriptor().raw().clone();
                    static_methods.insert((name, descriptor), StaticMethodType::Native(method));
                }
                (true, false) => {
                    methods.push(VirtualMethodType::Native(NativeMethod::new(method, &cp)?))
                }
                (false, true) => {
                    if name == "<clinit>" {
                        initializer = Some(Method::new(method, &cp)?);
                    } else {
                        let method = Method::new(method, &cp)?;
                        let name = method.name().clone();
                        let descriptor = method.descriptor().raw().clone();
                        static_methods.insert((name, descriptor), StaticMethodType::Java(method));
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
                let resolved_field = StaticField::new(field, &cp)?;
                let name = resolved_field.name.clone();
                let descriptor = resolved_field.descriptor.raw().clone();
                static_fields.insert((name, descriptor), resolved_field);
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
            RwLock::new(InitState::NotInitialized)
        } else {
            RwLock::new(InitState::Initialized)
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
            state: initialized,
        })
    }

    pub fn name(&self) -> Result<&Arc<String>, JvmError> {
        self.this.name().map_err(Into::into)
    }

    //TODO: check the logic
    pub fn find_main_method(&self) -> Option<&Method> {
        self.static_methods
            .iter()
            .find(|(_, m)| m.is_main())
            .and_then(|(_, m)| match m {
                StaticMethodType::Java(m) => Some(m),
                StaticMethodType::Native(_) => None,
            })
    }

    pub fn cp(&self) -> &Arc<RuntimeConstantPool> {
        &self.cp
    }

    //TODO: right now I don't use initializing state, but I will need it when implementing multithreading
    pub fn initialized(&self) -> bool {
        matches!(
            *self.state.read(),
            InitState::Initializing | InitState::Initialized
        )
    }

    pub fn set_state(&self, state: InitState) {
        *self.state.write() = state;
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

    pub fn get_static_field_value(
        &self,
        nat: &NameAndTypeReference,
    ) -> Result<TypeValue, JvmError> {
        let name = nat.name()?;
        let descriptor = nat.field_descriptor()?.raw();

        if let Some(field) = self.static_fields.get(&(name.clone(), descriptor.clone())) {
            Ok(field.value.borrow().clone())
        } else {
            todo!()
        }
    }

    pub fn get_static_method(
        &self,
        method_ref: &MethodReference,
    ) -> Result<&StaticMethodType, JvmError> {
        let nat = method_ref.name_and_type()?;
        let name = nat.name()?;
        let descriptor = nat.method_descriptor()?.raw();

        if let Some(method) = self.static_methods.get(&(name.clone(), descriptor.clone())) {
            Ok(method)
        } else {
            todo!()
        }
    }
}
