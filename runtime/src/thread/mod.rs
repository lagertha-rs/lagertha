use crate::vm::stack::FrameStack;
use common::HeapRef;

pub struct JavaThreadState {
    pub thread_obj: HeapRef,
    pub group_obj: HeapRef, // TODO: Once cell?
    pub name: HeapRef,
    pub stack: FrameStack,
}
