use crate::error::JvmError;
use crate::rt::constant_pool::entry::{
    ClassEntry, FieldEntry, FieldEntryView, InvokeDynamicEntry, InvokeDynamicEntryView,
    MethodEntry, MethodEntryView, MethodHandleEntryView, NameAndTypeEntry, NameAndTypeEntryView,
    StringEntry, Utf8Entry,
};
use crate::{Symbol, build_exception, throw_exception};
use jclass::attribute::class::BootstrapMethodEntry;
use jclass::constant::ConstantInfo;
use lasso::ThreadedRodeo;
use std::fmt::Display;

pub mod entry;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RuntimeConstantType {
    Unused,
    Utf8,
    Integer,
    Float,
    Long,
    Double,
    Class,
    String,
    Method,
    Field,
    InvokeDynamic,
    InterfaceMethod,
    NameAndType,
    MethodNameAndType,
    FieldNameAndType,
    MethodType,
    MethodHandle,
}

impl Display for RuntimeConstantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = match self {
            RuntimeConstantType::Unused => "Unused",
            RuntimeConstantType::Utf8 => "Utf8",
            RuntimeConstantType::Integer => "Integer",
            RuntimeConstantType::Float => "Float",
            RuntimeConstantType::Long => "Long",
            RuntimeConstantType::Double => "Double",
            RuntimeConstantType::Class => "Class",
            RuntimeConstantType::String => "String",
            RuntimeConstantType::Method => "Method",
            RuntimeConstantType::Field => "Field",
            RuntimeConstantType::InvokeDynamic => "InvokeDynamic",
            RuntimeConstantType::InterfaceMethod => "InterfaceMethod",
            RuntimeConstantType::NameAndType => "NameAndType",
            RuntimeConstantType::MethodNameAndType => "MethodNameAndType",
            RuntimeConstantType::FieldNameAndType => "FieldNameAndType",
            RuntimeConstantType::MethodType => "MethodType",
            RuntimeConstantType::MethodHandle => "MethodHandle",
        };
        write!(f, "{}", type_str)
    }
}

pub enum MethodHandleType {
    GetField(u16),
    GetStatic(u16),
    PutField(u16),
    PutStatic(u16),
    InvokeVirtual(u16),
    InvokeStatic(u16),
    InvokeSpecial(u16),
    NewInvokeSpecial(u16),
    InvokeInterface(u16),
}

pub enum RuntimeConstant {
    Unused,
    Utf8(Utf8Entry),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(ClassEntry),
    String(StringEntry),
    Method(MethodEntry),
    Field(FieldEntry),
    InvokeDynamic(InvokeDynamicEntry),
    InterfaceMethod(MethodEntry),
    NameAndType(NameAndTypeEntry),
    MethodType,
    MethodHandle(MethodHandleType), // TODO: use our own struct
}

impl RuntimeConstant {
    pub fn get_type(&self) -> RuntimeConstantType {
        match self {
            RuntimeConstant::Unused => RuntimeConstantType::Unused,
            RuntimeConstant::Utf8(_) => RuntimeConstantType::Utf8,
            RuntimeConstant::Integer(_) => RuntimeConstantType::Integer,
            RuntimeConstant::Float(_) => RuntimeConstantType::Float,
            RuntimeConstant::Long(_) => RuntimeConstantType::Long,
            RuntimeConstant::Double(_) => RuntimeConstantType::Double,
            RuntimeConstant::Class(_) => RuntimeConstantType::Class,
            RuntimeConstant::String(_) => RuntimeConstantType::String,
            RuntimeConstant::Method(_) => RuntimeConstantType::Method,
            RuntimeConstant::Field(_) => RuntimeConstantType::Field,
            RuntimeConstant::InterfaceMethod(_) => RuntimeConstantType::InterfaceMethod,
            RuntimeConstant::NameAndType(_) => RuntimeConstantType::NameAndType,
            RuntimeConstant::InvokeDynamic(_) => RuntimeConstantType::InvokeDynamic,
            RuntimeConstant::MethodType => RuntimeConstantType::MethodType,
            RuntimeConstant::MethodHandle(_) => RuntimeConstantType::MethodHandle,
        }
    }
}

pub struct RuntimeConstantPool {
    entries: Vec<RuntimeConstant>,
    bootstrap_entries: Vec<BootstrapMethodEntry>,
}

impl RuntimeConstantPool {
    pub fn new(entries: Vec<ConstantInfo>, bootstrap_methods: Vec<BootstrapMethodEntry>) -> Self {
        let mut rt_entries = Vec::with_capacity(entries.len());
        for entry in entries {
            let rt_entry = match entry {
                ConstantInfo::Unused => RuntimeConstant::Unused,
                ConstantInfo::Utf8(utf8) => RuntimeConstant::Utf8(Utf8Entry::new(utf8)),
                ConstantInfo::Integer(v) => RuntimeConstant::Integer(v),
                ConstantInfo::Float(v) => RuntimeConstant::Float(v),
                ConstantInfo::Long(v) => RuntimeConstant::Long(v),
                ConstantInfo::Double(v) => RuntimeConstant::Double(v),
                ConstantInfo::Class(idx) => RuntimeConstant::Class(ClassEntry::new(idx)),
                ConstantInfo::String(idx) => RuntimeConstant::String(StringEntry::new(idx)),
                ConstantInfo::MethodRef(ref_info) => RuntimeConstant::Method(MethodEntry::new(
                    ref_info.class_index,
                    ref_info.name_and_type_index,
                )),
                ConstantInfo::FieldRef(ref_info) => RuntimeConstant::Field(FieldEntry::new(
                    ref_info.class_index,
                    ref_info.name_and_type_index,
                )),
                ConstantInfo::NameAndType(nat_info) => RuntimeConstant::NameAndType(
                    NameAndTypeEntry::new(nat_info.name_index, nat_info.descriptor_index),
                ),
                ConstantInfo::InterfaceMethodRef(ref_info) => RuntimeConstant::InterfaceMethod(
                    MethodEntry::new(ref_info.class_index, ref_info.name_and_type_index),
                ),
                ConstantInfo::InvokeDynamic(dynamic_info) => {
                    RuntimeConstant::InvokeDynamic(InvokeDynamicEntry::new(
                        dynamic_info.bootstrap_method_attr_index,
                        dynamic_info.name_and_type_index,
                    ))
                }
                ConstantInfo::MethodType(_) => RuntimeConstant::MethodType,
                // TODO: handle could have already mapped MethodHandleKind enum instead of u8
                ConstantInfo::MethodHandle(handle) => {
                    let method_handle_type = match handle.reference_kind {
                        1 => MethodHandleType::GetField(handle.reference_index),
                        2 => MethodHandleType::GetStatic(handle.reference_index),
                        3 => MethodHandleType::PutField(handle.reference_index),
                        4 => MethodHandleType::PutStatic(handle.reference_index),
                        5 => MethodHandleType::InvokeVirtual(handle.reference_index),
                        6 => MethodHandleType::InvokeStatic(handle.reference_index),
                        7 => MethodHandleType::InvokeSpecial(handle.reference_index),
                        8 => MethodHandleType::NewInvokeSpecial(handle.reference_index),
                        9 => MethodHandleType::InvokeInterface(handle.reference_index),
                        other => {
                            unimplemented!(
                                "MethodHandle reference kind {} not implemented yet",
                                other
                            )
                        }
                    };
                    RuntimeConstant::MethodHandle(method_handle_type)
                }
                other => unimplemented!("{:?} not implemented yet", other),
            };
            rt_entries.push(rt_entry);
        }
        Self {
            entries: rt_entries,
            bootstrap_entries: bootstrap_methods,
        }
    }

    pub fn get_constant(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<&RuntimeConstant, JvmError> {
        let entry = self.entry(idx)?;
        match entry {
            RuntimeConstant::Class(_) => {
                self.get_class_sym(idx, interner)?;
            }
            RuntimeConstant::String(_) => {
                self.get_string_sym(idx, interner)?;
            }
            RuntimeConstant::Method(_) => {
                self.get_method_view(idx, interner)?;
            }
            RuntimeConstant::Field(_) => {
                self.get_field_view(idx, interner)?;
            }
            _ => {}
        };
        Ok(entry)
    }

    fn entry(&self, idx: &u16) -> Result<&RuntimeConstant, JvmError> {
        self.entries.get(*idx as usize).ok_or(build_exception!(
            ClassFormatError,
            "Invalid constant pool index: {}",
            *idx
        ))
    }

    fn bootstrap_entry(&self, idx: &u16) -> Result<&BootstrapMethodEntry, JvmError> {
        self.bootstrap_entries
            .get(*idx as usize)
            .ok_or(build_exception!(
                ClassFormatError,
                "Invalid bootstrap methods index: {}",
                *idx
            ))
    }

    pub fn get_utf8_sym(&self, idx: &u16, interner: &ThreadedRodeo) -> Result<Symbol, JvmError> {
        match self.entry(idx)? {
            RuntimeConstant::Utf8(entry) => Ok(*entry
                .utf8_sym
                .get_or_init(|| interner.get_or_intern(&entry.value))),
            other => throw_exception!(
                IncompatibleClassChangeError,
                pool_idx: *idx,
                expected: RuntimeConstantType::Utf8,
                actual: other.get_type()
            ),
        }
    }

    pub fn get_nat_view(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<NameAndTypeEntryView, JvmError> {
        match self.entry(idx)? {
            RuntimeConstant::NameAndType(entry) => {
                let name_sym = *entry
                    .name_sym
                    .get_or_try_init(|| self.get_utf8_sym(&entry.name_idx, interner))?;
                let descriptor_sym = *entry
                    .descriptor_sym
                    //TODO: delete explicit type?
                    .get_or_try_init(|| self.get_utf8_sym(&entry.descriptor_idx, interner))?;
                Ok(NameAndTypeEntryView::new(name_sym, descriptor_sym))
            }
            other => throw_exception!(
                IncompatibleClassChangeError,
                pool_idx: *idx,
                expected: RuntimeConstantType::MethodNameAndType,
                actual: other.get_type()
            ),
        }
    }

    pub fn get_method_view(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<MethodEntryView, JvmError> {
        match self.entry(idx)? {
            RuntimeConstant::Method(entry) => {
                let class_sym = *entry
                    .class_sym
                    .get_or_try_init(|| self.get_class_sym(&entry.class_idx, interner))?;
                let nat_view = self.get_nat_view(&entry.nat_idx, interner)?;
                Ok(MethodEntryView::new(class_sym, nat_view))
            }
            other => throw_exception!(
                IncompatibleClassChangeError,
                pool_idx: *idx,
                expected: RuntimeConstantType::Method,
                actual: other.get_type()
            ),
        }
    }

    pub fn get_interface_method_view(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<MethodEntryView, JvmError> {
        match self.entry(idx)? {
            RuntimeConstant::InterfaceMethod(entry) => {
                let class_sym = *entry
                    .class_sym
                    .get_or_try_init(|| self.get_class_sym(&entry.class_idx, interner))?;
                let nat_view = self.get_nat_view(&entry.nat_idx, interner)?;
                Ok(MethodEntryView::new(class_sym, nat_view))
            }
            other => throw_exception!(
                IncompatibleClassChangeError,
                pool_idx: *idx,
                expected: RuntimeConstantType::InterfaceMethod,
                actual: other.get_type()
            ),
        }
    }

    pub fn get_field_view(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<FieldEntryView, JvmError> {
        match self.entry(idx)? {
            RuntimeConstant::Field(entry) => {
                let class_sym = *entry
                    .class_sym
                    .get_or_try_init(|| self.get_class_sym(&entry.class_idx, interner))?;
                let nat_view = self.get_nat_view(&entry.nat_idx, interner)?;
                Ok(FieldEntryView::new(class_sym, nat_view))
            }
            other => throw_exception!(
                IncompatibleClassChangeError,
                pool_idx: *idx,
                expected: RuntimeConstantType::Field,
                actual: other.get_type()
            ),
        }
    }

    pub fn get_method_handle_view(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<MethodHandleEntryView, JvmError> {
        match self.entry(idx)? {
            RuntimeConstant::MethodHandle(entry) => {
                let res = match entry {
                    MethodHandleType::GetField(idx) => {
                        MethodHandleEntryView::GetField(self.get_field_view(idx, interner)?)
                    }
                    MethodHandleType::GetStatic(idx) => {
                        MethodHandleEntryView::GetStatic(self.get_field_view(idx, interner)?)
                    }
                    MethodHandleType::PutField(idx) => {
                        MethodHandleEntryView::PutField(self.get_field_view(idx, interner)?)
                    }
                    MethodHandleType::PutStatic(idx) => {
                        MethodHandleEntryView::PutStatic(self.get_field_view(idx, interner)?)
                    }
                    MethodHandleType::InvokeVirtual(idx) => {
                        MethodHandleEntryView::InvokeVirtual(self.get_method_view(idx, interner)?)
                    }
                    MethodHandleType::InvokeStatic(idx) => {
                        MethodHandleEntryView::InvokeStatic(self.get_method_view(idx, interner)?)
                    }
                    MethodHandleType::InvokeSpecial(idx) => {
                        MethodHandleEntryView::InvokeSpecial(self.get_method_view(idx, interner)?)
                    }
                    MethodHandleType::NewInvokeSpecial(idx) => {
                        MethodHandleEntryView::NewInvokeSpecial(
                            self.get_method_view(idx, interner)?,
                        )
                    }
                    MethodHandleType::InvokeInterface(idx) => {
                        MethodHandleEntryView::InvokeInterface(
                            self.get_interface_method_view(idx, interner)?,
                        )
                    }
                };
                Ok(res)
            }
            other => throw_exception!(
                IncompatibleClassChangeError,
                pool_idx: *idx,
                expected: RuntimeConstantType::MethodHandle,
                actual: other.get_type()
            ),
        }
    }

    pub fn get_invoke_dynamic_view(
        &self,
        idx: &u16,
        interner: &ThreadedRodeo,
    ) -> Result<InvokeDynamicEntryView, JvmError> {
        match self.entry(idx)? {
            // TODO: need to review all structs, for method handle as well
            RuntimeConstant::InvokeDynamic(entry) => {
                let bootstrap_entry = self.bootstrap_entry(&entry.bootstrap_idx)?;
                let method_handle_view =
                    self.get_method_handle_view(&bootstrap_entry.bootstrap_method_idx, interner)?;
                let nat_view = self.get_nat_view(&entry.nat_idx, interner)?;
                Ok(InvokeDynamicEntryView::new(
                    method_handle_view,
                    bootstrap_entry.bootstrap_arguments.clone(),
                    nat_view,
                ))
            }
            other => throw_exception!(
                IncompatibleClassChangeError,
                pool_idx: *idx,
                expected: RuntimeConstantType::InvokeDynamic,
                actual: other.get_type()
            ),
        }
    }

    pub fn get_string_sym(&self, idx: &u16, interner: &ThreadedRodeo) -> Result<Symbol, JvmError> {
        match self.entry(idx)? {
            RuntimeConstant::String(entry) => entry
                .string_sym
                .get_or_try_init(|| self.get_utf8_sym(&entry.string_idx, interner))
                .copied(),
            other => throw_exception!(
                IncompatibleClassChangeError,
                pool_idx: *idx,
                expected: RuntimeConstantType::String,
                actual: other.get_type()
            ),
        }
    }

    pub fn get_class_sym(&self, idx: &u16, interner: &ThreadedRodeo) -> Result<Symbol, JvmError> {
        match self.entry(idx)? {
            RuntimeConstant::Class(entry) => entry
                .name_sym
                .get_or_try_init(|| self.get_utf8_sym(&entry.name_idx, interner))
                .copied(),
            other => throw_exception!(
                IncompatibleClassChangeError,
                pool_idx: *idx,
                expected: RuntimeConstantType::Class,
                actual: other.get_type()
            ),
        }
    }
}
