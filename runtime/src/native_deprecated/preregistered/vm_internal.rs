use crate::native_deprecated::{NativeRegistryDeprecated, NativeRetDeprecated};
use crate::stack_deprecated::FrameStackDeprecated;
use crate::{FullyQualifiedMethodKey, VirtualMachineDeprecated};
use common::jtype::Value;
use tracing_log::log::debug;

pub(super) fn do_register_vm_internal_preregistered_natives(
    native_registry: &mut NativeRegistryDeprecated,
) {
    native_registry.register(
        FullyQualifiedMethodKey::new_internal_with_str(
            "clone",
            "()Ljava/lang/Object;",
            &native_registry.string_interner,
        ),
        vm_internal_clone,
    );
}

fn vm_internal_clone(
    vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,
    args: &[Value],
) -> NativeRetDeprecated {
    debug!("TODO: Stub: internal clone");
    let obj = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("internal clone: expected object"),
    };
    let cloned = vm.heap.clone_object(obj);
    Ok(Some(Value::Ref(cloned)))
}
