mod class_loader;
mod heap;
mod method_area;
mod native_registry;
pub mod stack;
mod string_pool;

type HeapAddr = usize;

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Ref(HeapAddr),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ClassId(pub usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MethodId(pub usize);

#[derive(Debug, Eq, PartialEq)]
pub enum ClassState {
    Loaded,
    Initialized,
}

pub fn run(_main: Vec<u8>) {}
