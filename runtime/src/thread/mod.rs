use common::jtype::HeapAddr;

pub struct ThreadState {
    pub thread_obj: HeapAddr,
    pub group_obj: HeapAddr,
    pub name: String,
}
