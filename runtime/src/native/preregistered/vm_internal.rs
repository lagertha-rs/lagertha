use crate::VirtualMachine;
use crate::native::{MethodKey, NativeRegistry, NativeRet};
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

fn vm_internal_clone(vm: &mut VirtualMachine, args: &[Value]) -> NativeRet {
    debug!("TODO: Stub: internal clone");
    let obj = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("internal clone: expected object"),
    };
    let cloned = vm.heap.clone_object(obj);
    Some(Value::Ref(cloned))
}
