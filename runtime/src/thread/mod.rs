use crate::stack::FrameStack;
use common::jtype::HeapRef;

pub struct JavaThreadState {
    pub thread_obj: HeapRef,
    pub group_obj: HeapRef, // TODO: Once cell?
    pub name: HeapRef,
    pub stack: FrameStack,
}

pub struct RustThreadState {
    pub stack: FrameStack,
}
