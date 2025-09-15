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
use common::jtype::Value;
use parking_lot::RwLock;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::sync::Arc;

type NatHashMap<T> = HashMap<Arc<str>, HashMap<Arc<str>, T>>;

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
    static_fields: NatHashMap<StaticField>,
    // TODO: probably use hashmap for methods with method name+descriptor as key. TBD when execute instructions
    methods: Vec<VirtualMethodType>,
    static_methods: NatHashMap<StaticMethodType>,
    constructors: Vec<Method>,
    // TODO: can't be native, but easier to handle it this way for now
    initializer: Option<StaticMethodType>,
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
        let mut static_methods: NatHashMap<StaticMethodType> = HashMap::new();
        let mut constructors = vec![];
        let mut initializer = None;
        for method in cf.methods {
            let flags = method.access_flags;
            let name = cp.get_utf8(&method.name_index)?;

            match (flags.is_native(), flags.is_static()) {
                (true, true) => {
                    let method = NativeMethod::new(method, &cp)?;
                    let name = method.name_arc();
                    let descriptor = method.descriptor().raw_arc();
                    static_methods
                        .entry(name)
                        .or_default()
                        .insert(descriptor, StaticMethodType::Native(method));
                }
                (true, false) => {
                    methods.push(VirtualMethodType::Native(NativeMethod::new(method, &cp)?))
                }
                (false, true) => {
                    if name == "<clinit>" {
                        initializer = Some(StaticMethodType::Java(Method::new(method, &cp)?));
                    } else {
                        let method = Method::new(method, &cp)?;
                        let name = method.name_arc();
                        let descriptor = method.descriptor().raw_arc();
                        static_methods
                            .entry(name)
                            .or_default()
                            .insert(descriptor, StaticMethodType::Java(method));
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
        let mut static_fields: NatHashMap<StaticField> = HashMap::new();
        let mut fields = vec![];

        for field in cf.fields {
            if field.access_flags.is_static() {
                let resolved_field = StaticField::new(field, &cp)?;
                let name = resolved_field.name_arc();
                let descriptor = resolved_field.descriptor().raw_arc();
                static_fields
                    .entry(name)
                    .or_default()
                    .insert(descriptor, resolved_field);
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

    pub fn name(&self) -> Result<&str, JvmError> {
        self.this.name().map_err(Into::into)
    }

    pub fn find_main_method(&self) -> Option<&Method> {
        self.static_methods
            .get("main")
            .and_then(|m| m.get("([Ljava/lang/String;)V"))
            .and_then(|m| match m {
                StaticMethodType::Java(method) => Some(method),
                _ => None,
            })
    }

    pub fn cp(&self) -> &Arc<RuntimeConstantPool> {
        &self.cp
    }

    pub fn idx(&self) -> u16 {
        *self.this.cp_index()
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
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

    pub fn initializer(&self) -> Option<&StaticMethodType> {
        self.initializer.as_ref()
    }

    /// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-5.html#jvms-5.4.3.2
    pub fn set_static_field(
        &self,
        nat: &NameAndTypeReference,
        value: Value,
    ) -> Result<(), JvmError> {
        let name = nat.name()?;
        let descriptor = nat.field_descriptor()?.raw();

        self.static_fields
            .get(name)
            .and_then(|m| m.get(descriptor))
            .map(|f| f.set_value(value))
            .ok_or(JvmError::FieldNotFound(name.to_string()))??;

        /* TODO:
        NoSuchFieldError,
        IncompatibleClassChangeError,
        IllegalAccessError
        etc...
         */
        Ok(())
    }

    pub fn get_static_field_value(&self, nat: &NameAndTypeReference) -> Result<Value, JvmError> {
        let name = nat.name()?;
        let descriptor = nat.field_descriptor()?.raw();

        self.static_fields
            .get(name)
            .and_then(|m| m.get(descriptor))
            .map(|f| f.value())
            .ok_or(JvmError::FieldNotFound(name.to_string()))
    }

    pub fn get_static_method_by_nat(
        &self,
        method_ref: &MethodReference,
    ) -> Result<&StaticMethodType, JvmError> {
        let nat = method_ref.name_and_type()?;
        let name = nat.name()?;
        let descriptor = nat.method_descriptor()?.raw();

        self.get_static_method(name, descriptor)
    }

    pub fn get_static_method(
        &self,
        name: &str,
        descriptor: &str,
    ) -> Result<&StaticMethodType, JvmError> {
        self.static_methods
            .get(name)
            .and_then(|m| m.get(descriptor))
            .ok_or(JvmError::NoSuchMethod(format!("{}.{}", name, descriptor)))
    }
}
