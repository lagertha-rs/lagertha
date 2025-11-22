use lasso::{Spur, ThreadedRodeo};
use std::num::NonZeroU32;

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ThreadId(NonZeroU32);

impl ThreadId {
    pub fn new(val: NonZeroU32) -> Self {
        ThreadId(val)
    }
    pub fn from_usize(index: usize) -> Self {
        ThreadId(NonZeroU32::new(index as u32).unwrap())
    }

    pub fn from_index(index: usize) -> Self {
        ThreadId(NonZeroU32::new((index as u32) + 1).unwrap())
    }

    pub fn into_inner(self) -> NonZeroU32 {
        self.0
    }

    pub fn as_usize(&self) -> usize {
        self.0.get() as usize
    }
    pub fn to_index(&self) -> usize {
        (self.0.get() - 1) as usize
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MethodId(NonZeroU32);

impl MethodId {
    pub fn from_usize(index: usize) -> Self {
        MethodId(NonZeroU32::new(index as u32).unwrap())
    }
    pub fn to_index(&self) -> usize {
        (self.0.get() - 1) as usize
    }

    //TODO: bad
    pub fn to_i32(&self) -> i32 {
        self.0.get() as i32
    }

    //TODO: also need but needs for previous bad :D
    pub fn from_i32(index: i32) -> Self {
        MethodId(NonZeroU32::new(index as u32).unwrap())
    }

    pub fn into_inner(self) -> NonZeroU32 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct MethodDescriptorId(NonZeroU32);

impl MethodDescriptorId {
    pub fn from_usize(index: usize) -> Self {
        MethodDescriptorId(NonZeroU32::new(index as u32).unwrap())
    }
    pub fn to_index(&self) -> usize {
        (self.0.get() - 1) as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FieldDescriptorId(NonZeroU32);

impl FieldDescriptorId {
    pub fn from_usize(index: usize) -> Self {
        FieldDescriptorId(NonZeroU32::new(index as u32).unwrap())
    }
    pub fn to_index(&self) -> usize {
        (self.0.get() - 1) as usize
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ClassId(NonZeroU32);

impl ClassId {
    pub fn new(val: NonZeroU32) -> Self {
        ClassId(val)
    }
    pub fn from_usize(index: usize) -> Self {
        ClassId(NonZeroU32::new(index as u32).unwrap())
    }
    pub fn to_index(&self) -> usize {
        (self.0.get() - 1) as usize
    }

    //TODO: bad
    pub fn to_i32(&self) -> i32 {
        self.0.get() as i32
    }

    //TODO: also need but needs for previous bad :D
    pub fn from_i32(index: i32) -> Self {
        ClassId(NonZeroU32::new(index as u32).unwrap())
    }
    pub fn into_inner(self) -> NonZeroU32 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct FieldId(NonZeroU32);

impl FieldId {
    pub fn from_usize(index: usize) -> Self {
        FieldId(NonZeroU32::new(index as u32).unwrap())
    }
    pub fn to_index(&self) -> usize {
        (self.0.get() - 1) as usize
    }
}

pub type Symbol = Spur;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct FullyQualifiedMethodKey {
    pub class: Option<Symbol>,
    pub name: Symbol,
    pub desc: Symbol,
}

impl FullyQualifiedMethodKey {
    pub fn new(class: Symbol, name: Symbol, desc: Symbol) -> Self {
        Self {
            class: Some(class),
            name,
            desc,
        }
    }

    pub fn new_internal(name: Symbol, desc: Symbol) -> Self {
        Self {
            class: None,
            name,
            desc,
        }
    }

    pub fn new_internal_with_str(name: &str, desc: &str, interner: &ThreadedRodeo) -> Self {
        Self {
            class: None,
            name: interner.get_or_intern(name),
            desc: interner.get_or_intern(desc),
        }
    }

    pub fn new_with_str(class: &str, name: &str, desc: &str, interner: &ThreadedRodeo) -> Self {
        Self {
            class: Some(interner.get_or_intern(class)),
            name: interner.get_or_intern(name),
            desc: interner.get_or_intern(desc),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct MethodKey {
    pub name: Symbol,
    pub desc: Symbol,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FieldKey {
    pub name: Symbol,
    pub desc: Symbol,
}

impl FieldKey {
    pub fn new(name: Symbol, desc: Symbol) -> Self {
        Self { name, desc }
    }
}
