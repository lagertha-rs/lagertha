use crate::stack::FrameStack;
use common::jtype::HeapAddr;

pub struct JavaThreadState {
    pub thread_obj: HeapAddr,
    pub group_obj: HeapAddr,
    pub name: String,
    pub stack: FrameStack,
}

pub struct RustThreadState {
    pub stack: FrameStack,
}
