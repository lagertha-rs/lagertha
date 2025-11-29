use crate::heap::HeapRef;
use crate::vm::stack::FrameStack;

pub struct JavaThreadState {
    pub thread_obj: HeapRef,
    pub group_obj: HeapRef, // TODO: Once cell?
    pub name: HeapRef,
    pub stack: FrameStack,
}
