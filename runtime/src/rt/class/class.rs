use crate::ClassId;
use crate::error::JvmError;
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
use common::jtype::{HeapAddr, Value};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::ops::ControlFlow;
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
    id: OnceCell<ClassId>,
    name: Arc<str>,
    access: ClassFlags,
    minor_version: u16,
    major_version: u16,
    super_class: Option<Arc<Class>>,
    fields: Vec<Field>,
    field_idx: NatHashMap<usize>,
    static_fields: NatHashMap<StaticField>,
    methods: NatHashMap<VirtualMethodType>,
    static_methods: NatHashMap<StaticMethodType>,
    initializer: Option<StaticMethodType>,
    attributes: Vec<ClassAttribute>,
    cp: Arc<RuntimeConstantPool>,
    state: RwLock<InitState>,
    mirror: OnceCell<HeapAddr>,
}

impl Class {
    pub fn new(cf: ClassFile, method_area: &mut MethodArea) -> Result<Arc<Self>, JvmError> {
        let cp = Arc::new(RuntimeConstantPool::new(cf.cp.inner));
        let minor_version = cf.minor_version;
        let major_version = cf.major_version;
        let name = cp.get_class(&cf.this_class)?.name_arc()?;
        let access = cf.access_flags;
        let mut methods: NatHashMap<VirtualMethodType> = HashMap::new();
        let mut static_methods: NatHashMap<StaticMethodType> = HashMap::new();
        let mut initializer = None;
        for method in cf.methods {
            let flags = method.access_flags;
            let name = cp.get_utf8(&method.name_index)?;

            if flags.is_native() && flags.is_abstract() {
                unimplemented!()
            }

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
                    let method = NativeMethod::new(method, &cp)?;
                    let name = method.name_arc();
                    let descriptor = method.descriptor().raw_arc();
                    methods
                        .entry(name)
                        .or_default()
                        .insert(descriptor, VirtualMethodType::Native(method));
                }
                (false, true) => {
                    if name == "<clinit>" {
                        initializer =
                            Some(StaticMethodType::Java(Arc::new(Method::new(method, &cp)?)));
                    } else {
                        let method = Method::new(method, &cp)?;
                        let name = method.name_arc();
                        let descriptor = method.descriptor().raw_arc();
                        static_methods
                            .entry(name)
                            .or_default()
                            .insert(descriptor, StaticMethodType::Java(Arc::new(method)));
                    }
                }
                (false, false) => {
                    // TODO: probably need to put constructor methods in separate list
                    let method = Method::new(method, &cp)?;
                    let name = method.name_arc();
                    let descriptor = method.descriptor().raw_arc();
                    let method_type = if flags.is_abstract() {
                        VirtualMethodType::Abstract(Arc::new(method))
                    } else {
                        VirtualMethodType::Java(Arc::new(method))
                    };
                    methods
                        .entry(name)
                        .or_default()
                        .insert(descriptor, method_type);
                }
            }
        }
        let mut static_fields: NatHashMap<StaticField> = HashMap::new();
        let mut fields = vec![];
        let mut field_idx: NatHashMap<usize> = HashMap::new();

        for field in cf.fields.into_iter() {
            if field.access_flags.is_static() {
                let resolved_field = StaticField::new(field, &cp)?;
                let name = resolved_field.name_arc();
                let descriptor = resolved_field.descriptor().raw_arc();
                static_fields
                    .entry(name)
                    .or_default()
                    .insert(descriptor, resolved_field);
            } else {
                let field = Field::new(field, &cp)?;
                field_idx
                    .entry(field.name_arc())
                    .or_default()
                    .insert(field.descriptor().raw_arc(), fields.len());
                fields.push(field);
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

        let class = Arc::new(Class {
            name,
            access,
            super_class,
            minor_version,
            major_version,
            fields,
            field_idx,
            static_fields,
            static_methods,
            methods,
            initializer,
            attributes: cf.attributes,
            cp,
            state: initialized,
            mirror: OnceCell::new(),
            id: OnceCell::new(),
        });

        for (_, method) in class.methods.values().flatten() {
            method.set_class(class.clone())?;
        }
        for (_, method) in class.static_methods.values().flatten() {
            method.set_class(class.clone())?;
        }
        if let Some(init) = &class.initializer {
            init.set_class(class.clone())?;
        }

        Ok(class)
    }

    pub fn new_primitive(name: &str) -> Result<Arc<Self>, JvmError> {
        let cp = Arc::new(RuntimeConstantPool::new(vec![]));
        let name = Arc::from(name);
        let access = ClassFlags::new(0);
        let minor_version = 0;
        let major_version = 0;
        let fields = vec![];
        let field_idx = HashMap::new();
        let static_fields = HashMap::new();
        let static_methods = HashMap::new();
        let methods = HashMap::new();
        let initializer = None;
        let super_class = None;
        let initialized = RwLock::new(InitState::Initialized);

        let class = Arc::new(Class {
            name,
            access,
            super_class,
            minor_version,
            major_version,
            fields,
            field_idx,
            static_fields,
            static_methods,
            methods,
            initializer,
            attributes: vec![],
            cp,
            state: initialized,
            mirror: OnceCell::new(),
            id: OnceCell::new(),
        });

        Ok(class)
    }

    pub fn id(&self) -> Result<ClassId, JvmError> {
        self.id.get().copied().ok_or(JvmError::Uninitialized)
    }

    // TODO: proper error
    pub fn set_id(&self, id: ClassId) -> Result<(), JvmError> {
        self.id.set(id).map_err(|_| JvmError::Uninitialized)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn find_main_method(&self) -> Option<&Arc<Method>> {
        self.static_methods
            .get("main")
            .and_then(|m| m.get("([Ljava/lang/String;)V"))
            .and_then(|m| match m {
                StaticMethodType::Java(method) => Some(method),
                _ => None,
            })
    }

    pub fn mirror(&self) -> Option<HeapAddr> {
        self.mirror.get().copied()
    }

    pub fn set_mirror(&self, mirror: HeapAddr) -> Result<(), JvmError> {
        self.mirror
            .set(mirror)
            .map_err(|_| JvmError::ClassMirrorIsAlreadyCreated)
    }

    pub fn cp(&self) -> &Arc<RuntimeConstantPool> {
        &self.cp
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

    fn get_field_index_recursive(&self, name: &str, descriptor: &str) -> ControlFlow<usize, usize> {
        if let Some(idx) = self
            .field_idx
            .get(name)
            .and_then(|m| m.get(descriptor))
            .copied()
        {
            return ControlFlow::Break(idx);
        }

        let fields_count = self.fields.len();
        match &self.super_class {
            Some(super_class) => match super_class.get_field_index_recursive(name, descriptor) {
                ControlFlow::Break(idx) => ControlFlow::Break(idx + fields_count),
                ControlFlow::Continue(acc) => ControlFlow::Continue(acc + fields_count),
            },
            None => ControlFlow::Continue(fields_count),
        }
    }

    pub fn get_field_index(&self, name: &str, descriptor: &str) -> Result<usize, JvmError> {
        match self.get_field_index_recursive(name, descriptor) {
            ControlFlow::Break(idx) => Ok(idx),
            ControlFlow::Continue(_) => Err(JvmError::FieldNotFound(name.to_string())),
        }
    }

    pub fn get_field_index_by_nat(&self, nat: &NameAndTypeReference) -> Result<usize, JvmError> {
        let name = nat.name()?;
        let descriptor = nat.field_descriptor()?.raw();

        self.get_field_index(name, descriptor)
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

    pub fn get_virtual_method_by_nat(
        &self,
        method_ref: &MethodReference,
    ) -> Result<&VirtualMethodType, JvmError> {
        let nat = method_ref.name_and_type()?;
        let name = nat.name()?;
        let descriptor = nat.method_descriptor()?.raw();

        self.get_virtual_method(name, descriptor)
    }

    fn get_virtual_method_recursive(
        &self,
        name: &str,
        descriptor: &str,
    ) -> Option<&VirtualMethodType> {
        if let Some(m) = self.methods.get(name).and_then(|m| m.get(descriptor)) {
            return Some(m);
        }

        match &self.super_class {
            Some(super_class) => super_class.get_virtual_method_recursive(name, descriptor),
            None => None,
        }
    }

    pub fn get_virtual_method(
        &self,
        name: &str,
        descriptor: &str,
    ) -> Result<&VirtualMethodType, JvmError> {
        self.get_virtual_method_recursive(name, descriptor)
            .ok_or(JvmError::NoSuchMethod(format!("{}.{}", name, descriptor)))
    }
}
