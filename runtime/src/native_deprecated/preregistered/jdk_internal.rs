use crate::native_deprecated::{NativeRegistryDeprecated, NativeRetDeprecated};
use crate::stack_deprecated::FrameStackDeprecated;
use crate::{FullyQualifiedMethodKey, VirtualMachineDeprecated};
use common::jtype::Value;
use tracing_log::log::debug;

pub(super) fn do_register_jdk_internal_preregistered_natives(
    native_registry: &mut NativeRegistryDeprecated,
) {
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Signal",
            "findSignal0",
            "(Ljava/lang/String;)I",
            &native_registry.string_interner,
        ),
        jdk_internal_misc_signal_find_signal_0,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/Signal",
            "handle0",
            "(IJ)J",
            &native_registry.string_interner,
        ),
        jdk_internal_misc_signal_handle_0,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/CDS",
            "getCDSConfigStatus",
            "()I",
            &native_registry.string_interner,
        ),
        jdk_internal_misc_cds_get_cds_config_status,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/CDS",
            "initializeFromArchive",
            "(Ljava/lang/Class;)V",
            &native_registry.string_interner,
        ),
        jdk_internal_misc_cds_initialize_from_archive,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/misc/VM",
            "initialize",
            "()V",
            &native_registry.string_interner,
        ),
        jdk_internal_misc_vm_initialize,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/util/SystemProps$Raw",
            "platformProperties",
            "()[Ljava/lang/String;",
            &native_registry.string_interner,
        ),
        jdk_internal_util_system_props_raw_platform_properties,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "jdk/internal/util/SystemProps$Raw",
            "vmProperties",
            "()[Ljava/lang/String;",
            &native_registry.string_interner,
        ),
        jdk_internal_util_system_props_raw_vm_properties,
    );
}

fn jdk_internal_util_system_props_raw_platform_properties(
    vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,

    _args: &[Value],
) -> NativeRetDeprecated {
    debug!("TODO: Stub: jdk.internal.util.SystemProps$Raw.platformProperties");
    let string_class = vm
        .method_area_deprecated
        .get_class_or_load_by_name("java/lang/String")
        .unwrap();
    let empty_string_stub = vm.heap.get_or_new_string("");
    let h = vm
        .heap
        .alloc_array_with_value(string_class, 40, Value::Ref(empty_string_stub))
        .unwrap();
    let enc = vm.heap.get_or_new_string("UTF-8");
    let line_separator_value = vm.heap.get_or_new_string("\n");
    vm.heap
        .write_array_element(h, 19, Value::Ref(line_separator_value))
        .unwrap();
    vm.heap.write_array_element(h, 27, Value::Ref(enc)).unwrap();
    vm.heap.write_array_element(h, 28, Value::Ref(enc)).unwrap();
    vm.heap.write_array_element(h, 34, Value::Ref(enc)).unwrap();

    Ok(Some(Value::Ref(h)))
}

fn jdk_internal_util_system_props_raw_vm_properties(
    vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,
    _args: &[Value],
) -> NativeRetDeprecated {
    debug!("TODO: Stub: jdk.internal.util.SystemProps$Raw.vmProperties");
    let string_class = vm
        .method_area_deprecated
        .get_class_or_load_by_name("java/lang/String")
        .unwrap();
    let h = vm.heap.alloc_array(string_class, 4).unwrap();
    let java_home_key = vm.heap.get_or_new_string("java.home");
    let java_home_value = vm.heap.get_or_new_string(&vm.config.home.to_str().unwrap());
    let sun_page_align_stub = vm.heap.get_or_new_string("sun.nio.PageAlignDirectMemory");
    let false_str = vm.heap.get_or_new_string("false");
    vm.heap
        .write_array_element(h, 0, Value::Ref(java_home_key))
        .unwrap();
    vm.heap
        .write_array_element(h, 1, Value::Ref(java_home_value))
        .unwrap();
    vm.heap
        .write_array_element(h, 2, Value::Ref(sun_page_align_stub))
        .unwrap();
    vm.heap
        .write_array_element(h, 3, Value::Ref(false_str))
        .unwrap();
    Ok(Some(Value::Ref(h)))
}

fn jdk_internal_misc_vm_initialize(
    _vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,
    _args: &[Value],
) -> NativeRetDeprecated {
    debug!("TODO: Stub: jdk.internal.misc.VM.initialize");
    Ok(None)
}

fn jdk_internal_misc_signal_find_signal_0(
    vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,
    args: &[Value],
) -> NativeRetDeprecated {
    debug!("TODO: Stub: jdk.internal.misc.Signal.findSignal0");
    let signal_name = match &args[0] {
        Value::Ref(h) => {
            let s = vm.heap.get_string(*h).unwrap();
            s.to_string()
        }
        _ => panic!("jdk.internal.misc.Signal.findSignal0: expected signal name string"),
    };
    let signal_number = match signal_name.as_str() {
        "HUP" => 1,
        "INT" => 2,
        "QUIT" => 3,
        "ILL" => 4,
        "ABRT" => 6,
        "FPE" => 8,
        "KILL" => 9,
        "SEGV" => 11,
        "PIPE" => 13,
        "ALRM" => 14,
        "TERM" => 15,
        "USR1" => 10,
        "USR2" => 12,
        "CHLD" => 17,
        "CONT" => 18,
        "STOP" => 19,
        "TSTP" => 20,
        "TTIN" => 21,
        "TTOU" => 22,
        _ => -1,
    };
    Ok(Some(Value::Integer(signal_number)))
}

fn jdk_internal_misc_signal_handle_0(
    _vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,
    _args: &[Value],
) -> NativeRetDeprecated {
    debug!("TODO: Stub: jdk.internal.misc.Signal.handle0");
    Ok(Some(Value::Long(1)))
}

fn jdk_internal_misc_cds_get_cds_config_status(
    _vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,

    _args: &[Value],
) -> NativeRetDeprecated {
    debug!("TODO: Stub: jdk.internal.misc.CDS.getCDSConfigStatus");
    Ok(Some(Value::Integer(0)))
}

fn jdk_internal_misc_cds_initialize_from_archive(
    _vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,

    _args: &[Value],
) -> NativeRetDeprecated {
    debug!("TODO: Stub: jdk.internal.misc.CDS.initializeFromArchive");
    Ok(None)
}
