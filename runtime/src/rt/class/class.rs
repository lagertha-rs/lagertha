use crate::ClassId;
use crate::error::JvmError;
use crate::method_area::MethodArea;
use crate::rt::class::field::{Field, StaticField};
use crate::rt::constant_pool::RuntimeConstantPool;
use crate::rt::constant_pool::reference::{
    MethodDescriptorReference, MethodReference, NameAndTypeReference,
};
use crate::rt::method::{Method, MethodType};
use class_file::ClassFile;
use class_file::attribute::class::ClassAttribute;
use class_file::flags::{ClassFlags, MethodFlags};
use common::descriptor::MethodDescriptor;
use common::instruction::ArrayType;
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
    primitive: Option<ArrayType>,
    name: Arc<str>,
    access: ClassFlags,
    minor_version: u16,
    major_version: u16,
    super_class: Option<Arc<Class>>,
    fields: Vec<Field>,
    field_idx: NatHashMap<usize>,
    static_fields: NatHashMap<StaticField>,
    method_idx: NatHashMap<usize>,
    methods: Vec<Arc<Method>>,
    static_method_idx: NatHashMap<usize>,
    static_methods: Vec<Arc<Method>>,
    initializer: Option<Arc<Method>>,
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
        let mut method_idx: NatHashMap<usize> = HashMap::new();
        let mut methods = vec![];
        let mut static_method_idx: NatHashMap<usize> = HashMap::new();
        let mut static_methods = vec![];
        let mut initializer = None;
        for method in cf.methods {
            let flags = method.access_flags;
            let name = cp.get_utf8(&method.name_index)?;

            if flags.is_native() && flags.is_abstract() {
                unimplemented!()
            }

            match (flags.is_native(), flags.is_static()) {
                (true, true) => {
                    let method = Method::new(method, MethodType::Native, &cp)?;
                    let name = method.name_arc();
                    let descriptor = method.descriptor().raw_arc();
                    static_method_idx
                        .entry(name)
                        .or_default()
                        .insert(descriptor, static_methods.len());
                    static_methods.push(Arc::new(method));
                }
                (true, false) => {
                    let method = Method::new(method, MethodType::Native, &cp)?;
                    let name = method.name_arc();
                    let descriptor = method.descriptor().raw_arc();
                    method_idx
                        .entry(name)
                        .or_default()
                        .insert(descriptor, methods.len());
                    methods.push(Arc::new(method));
                }
                (false, true) => {
                    if name == "<clinit>" {
                        initializer = Some(Arc::new(Method::new(method, MethodType::Java, &cp)?));
                    } else {
                        let method = Method::new(method, MethodType::Java, &cp)?;
                        let name = method.name_arc();
                        let descriptor = method.descriptor().raw_arc();
                        static_method_idx
                            .entry(name)
                            .or_default()
                            .insert(descriptor, static_methods.len());
                        static_methods.push(Arc::new(method))
                    }
                }
                (false, false) => {
                    // TODO: probably need to put constructor methods in separate list
                    let method = if flags.is_abstract() {
                        Method::new(method, MethodType::Abstract, &cp)?
                    } else {
                        Method::new(method, MethodType::Java, &cp)?
                    };
                    let name = method.name_arc();
                    let descriptor = method.descriptor().raw_arc();
                    method_idx
                        .entry(name)
                        .or_default()
                        .insert(descriptor, methods.len());
                    methods.push(Arc::new(method))
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
            static_method_idx,
            method_idx,
            initializer,
            attributes: cf.attributes,
            primitive: None,
            cp,
            state: initialized,
            mirror: OnceCell::new(),
            id: OnceCell::new(),
            methods,
            static_methods,
        });

        for (i, method) in class.methods.iter().enumerate() {
            method.set_class(class.clone())?;
            method.set_id(i)?;
        }
        for (i, method) in class.static_methods.iter().enumerate() {
            method.set_class(class.clone())?;
            method.set_id(i)?;
        }
        if let Some(init) = &class.initializer {
            init.set_class(class.clone())?;
            init.set_id(0)?;
        }

        Ok(class)
    }

    pub fn new_array(class_name: &str) -> Result<Arc<Self>, JvmError> {
        Ok(Self::default(Arc::from(class_name), None))
    }

    pub fn new_primitive_array(primitive: ArrayType) -> Result<Arc<Self>, JvmError> {
        Ok(Self::default(
            Arc::from(primitive.descriptor()),
            Some(primitive),
        ))
    }

    pub fn id(&self) -> Result<ClassId, JvmError> {
        self.id.get().copied().ok_or(JvmError::Uninitialized)
    }

    pub fn primitive(&self) -> Option<ArrayType> {
        self.primitive
    }

    pub fn instance_of(&self, class: &Arc<Class>) -> bool {
        if self.name == class.name {
            return true;
        }
        match &self.super_class {
            Some(super_class) => super_class.instance_of(class),
            None => false,
        }
    }

    // TODO: proper error
    pub fn set_id(&self, id: ClassId) -> Result<(), JvmError> {
        self.id.set(id).map_err(|_| JvmError::Uninitialized)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn find_main_method(&self) -> Option<&Arc<Method>> {
        self.static_method_idx
            .get("main")
            .and_then(|m| m.get("([Ljava/lang/String;)V"))
            .and_then(|i| self.static_methods.get(*i))
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

    pub fn initializer(&self) -> Option<&Arc<Method>> {
        self.initializer.as_ref()
    }

    pub fn set_static_field(
        &self,
        name: &str,
        descriptor: &str,
        value: Value,
    ) -> Result<(), JvmError> {
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

    /// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-5.html#jvms-5.4.3.2
    pub fn set_static_field_by_nat(
        &self,
        nat: &NameAndTypeReference,
        value: Value,
    ) -> Result<(), JvmError> {
        let name = nat.name()?;
        let descriptor = nat.field_descriptor()?.raw();

        self.set_static_field(name, descriptor, value)?;

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

    //TODO: probably I don't need to index fields by descriptor, because field overloading is not allowed in Java
    pub fn get_field_index_by_name(&self, name: &str) -> Result<usize, JvmError> {
        let descriptors = self.field_idx.get(name);
        if let Some(descriptors) = descriptors {
            if descriptors.len() == 1 {
                if let Some(idx) = descriptors.values().next() {
                    return Ok(*idx);
                }
            } else {
                panic!("More than one field with name {}", name);
            }
        }
        Err(JvmError::FieldNotFound(name.to_string()))
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

    pub fn get_static_field_value_by_nat(
        &self,
        nat: &NameAndTypeReference,
    ) -> Result<Value, JvmError> {
        let name = nat.name()?;
        let descriptor = nat.field_descriptor()?.raw();

        self.get_static_field_value(name, descriptor)
    }

    fn get_static_field_value_recursive(
        &self,
        name: &str,
        descriptor: &str,
    ) -> Option<&StaticField> {
        if let Some(f) = self.static_fields.get(name).and_then(|m| m.get(descriptor)) {
            return Some(f);
        }

        match &self.super_class {
            Some(super_class) => super_class.get_static_field_value_recursive(name, descriptor),
            None => None,
        }
    }

    pub fn get_static_field_value(&self, name: &str, descriptor: &str) -> Result<Value, JvmError> {
        self.get_static_field_value_recursive(name, descriptor)
            .map(|f| f.value())
            .ok_or(JvmError::FieldNotFound(name.to_string()))
    }

    pub fn get_static_method_by_nat(
        &self,
        method_ref: &MethodReference,
    ) -> Result<&Arc<Method>, JvmError> {
        let nat = method_ref.name_and_type()?;
        let name = nat.name()?;
        let descriptor = nat.method_descriptor()?.raw();

        self.get_static_method(name, descriptor)
    }

    pub fn get_static_method(
        &self,
        name: &str,
        descriptor: &str,
    ) -> Result<&Arc<Method>, JvmError> {
        self.static_method_idx
            .get(name)
            .and_then(|m| m.get(descriptor))
            .and_then(|i| self.static_methods.get(*i))
            .ok_or(JvmError::NoSuchMethod(format!("{}.{}", name, descriptor)))
    }

    pub fn get_virtual_method_by_nat(
        &self,
        method_ref: &MethodReference,
    ) -> Result<&Arc<Method>, JvmError> {
        let nat = method_ref.name_and_type()?;
        let name = nat.name()?;
        let descriptor = nat.method_descriptor()?.raw();

        self.get_virtual_method(name, descriptor)
    }

    // Returns method and the constant pool of the class where the method was found
    pub fn get_virtual_method_and_cp_by_nat(
        &self,
        method_ref: &MethodReference,
    ) -> Result<(&Arc<Method>, &Arc<RuntimeConstantPool>), JvmError> {
        let nat = method_ref.name_and_type()?;
        let name = nat.name()?;
        let descriptor = nat.method_descriptor()?.raw();

        self.get_virtual_method_recursive(name, descriptor)
            .ok_or(JvmError::NoSuchMethod(format!(
                "{}.{}{}",
                self.name(),
                name,
                descriptor
            )))
    }

    fn get_virtual_method_recursive(
        &self,
        name: &str,
        descriptor: &str,
    ) -> Option<(&Arc<Method>, &Arc<RuntimeConstantPool>)> {
        if let Some(m) = self.method_idx.get(name).and_then(|m| m.get(descriptor)) {
            let method = self.methods.get(*m).unwrap();
            return Some((method, &self.cp));
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
    ) -> Result<&Arc<Method>, JvmError> {
        self.get_virtual_method_recursive(name, descriptor)
            .map(|(m, _)| m)
            .ok_or(JvmError::NoSuchMethod(format!(
                "{}.{}{}",
                self.name(),
                name,
                descriptor
            )))
    }

    pub fn is_subclass_of(&self, name: &str) -> bool {
        if self.name() == name {
            return true;
        }
        match &self.super_class {
            Some(super_class) => super_class.is_subclass_of(name),
            None => false,
        }
    }

    //TODO: stub, need to cleanup
    pub fn default(class_name: Arc<str>, primitive: Option<ArrayType>) -> Arc<Self> {
        let clone_method_name: Arc<str> = Arc::from("clone");
        let raw_descriptor: &str = "()Ljava/lang/Object;";

        let resolved_descriptor = MethodDescriptor::try_from(raw_descriptor).unwrap();
        let clone_native_descriptor =
            MethodDescriptorReference::new(0, Arc::from(raw_descriptor), resolved_descriptor);

        let clone_method = Arc::new(Method::new_native(
            clone_method_name.clone(),
            Arc::new(clone_native_descriptor),
            MethodFlags::new(0),
        ));

        let method_idx: NatHashMap<usize> = HashMap::from([(
            clone_method_name.clone(),
            HashMap::from([(Arc::from(raw_descriptor), 0)]),
        )]);
        let methods = vec![clone_method];
        let class = Arc::new(Self {
            id: OnceCell::new(),
            primitive,
            name: class_name,
            access: ClassFlags::new(0),
            minor_version: 0,
            major_version: 0,
            super_class: None,
            fields: vec![],
            field_idx: HashMap::new(),
            static_fields: HashMap::new(),
            methods,
            method_idx,
            static_method_idx: HashMap::new(),
            static_methods: vec![],
            initializer: None,
            attributes: vec![],
            cp: Arc::new(RuntimeConstantPool::new(vec![])),
            state: RwLock::new(InitState::Initialized),
            mirror: OnceCell::new(),
        });
        for (i, method) in class.methods.iter().enumerate() {
            method.set_id(i).unwrap();
            method.set_class(class.clone()).unwrap();
        }
        class
    }
}
