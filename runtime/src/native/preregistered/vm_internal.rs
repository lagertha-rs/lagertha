use crate::VirtualMachine;
use crate::native::{MethodKey, NativeRegistry, NativeRet};
use crate::stack::FrameStack;
use common::jtype::Value;
use log::debug;

pub(super) fn do_register_vm_internal_preregistered_natives(native_registry: &mut NativeRegistry) {
    native_registry.register(
        MethodKey::new_internal_with_str(
            "clone",
            "()Ljava/lang/Object;",
            &native_registry.string_interner,
        ),
        vm_internal_clone,
    );
}

fn vm_internal_clone(
    vm: &mut VirtualMachine,
    _frame_stack: &FrameStack,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: internal clone");
    let obj = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("internal clone: expected object"),
    };
    let cloned = vm.heap.clone_object(obj);
    Ok(Some(Value::Ref(cloned)))
}
