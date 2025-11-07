use crate::native::{NativeRegistry, NativeRet};
use crate::stack_deprecated::FrameStackDeprecated;
use crate::{FullyQualifiedMethodKey, ThreadId, VirtualMachine};
use common::jtype::Value;
use log::debug;

pub(super) fn do_register_jdk_internal_preregistered_natives(native_registry: &mut NativeRegistry) {
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
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.util.SystemProps$Raw.platformProperties");
    let string_class_sym = vm.method_area.br().java_lang_string_sym;
    // TODO: create a registry for interned common strings
    let empty_string_sym = vm.method_area.interner().get_or_intern("");
    let string_class_id = vm.method_area.get_class_id_or_load(string_class_sym)?;
    let empty_string_stub = vm
        .heap
        .get_or_new_string(empty_string_sym, &mut vm.method_area)?;
    let h = vm.heap.alloc_array_with_default_value(
        string_class_id,
        Value::Ref(empty_string_stub),
        40,
    )?;
    let utf8_sym = vm.method_area.interner().get_or_intern("UTF-8");
    let enc = vm.heap.get_or_new_string(utf8_sym, &mut vm.method_area)?;
    let line_sep_sym = vm.method_area.interner().get_or_intern("\n");
    let line_separator_value = vm
        .heap
        .get_or_new_string(line_sep_sym, &mut vm.method_area)?;
    vm.heap
        .write_array_element(h, 19, Value::Ref(line_separator_value))?;
    vm.heap.write_array_element(h, 27, Value::Ref(enc))?;
    vm.heap.write_array_element(h, 28, Value::Ref(enc))?;
    vm.heap.write_array_element(h, 34, Value::Ref(enc))?;

    Ok(Some(Value::Ref(h)))
}

fn jdk_internal_util_system_props_raw_vm_properties(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.util.SystemProps$Raw.vmProperties");
    let string_class_sym = vm.method_area.br().java_lang_string_sym;
    let string_class = vm.method_area.get_class_id_or_load(string_class_sym)?;
    //TODO: same here, it needs a registry for common interned strings
    let h = vm
        .heap
        .alloc_array_with_default_value(string_class, Value::Null, 4)?;
    let java_home_key = vm.heap.get_or_new_string(
        vm.interner().get_or_intern("java.home"),
        &mut vm.method_area,
    )?;
    let java_home_value = vm.heap.get_or_new_string(
        vm.interner()
            .get_or_intern(vm.config.home.to_str().unwrap()),
        &mut vm.method_area,
    )?;
    let sun_page_align_stub = vm.heap.get_or_new_string(
        vm.interner().get_or_intern("sun.nio.PageAlignDirectMemory"),
        &mut vm.method_area,
    )?;
    let false_str = vm
        .heap
        .get_or_new_string(vm.interner().get_or_intern("false"), &mut vm.method_area)?;
    vm.heap
        .write_array_element(h, 0, Value::Ref(java_home_key))?;
    vm.heap
        .write_array_element(h, 1, Value::Ref(java_home_value))?;
    vm.heap
        .write_array_element(h, 2, Value::Ref(sun_page_align_stub))?;
    vm.heap.write_array_element(h, 3, Value::Ref(false_str))?;
    Ok(Some(Value::Ref(h)))
}

fn jdk_internal_misc_vm_initialize(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.VM.initialize");
    Ok(None)
}

fn jdk_internal_misc_signal_find_signal_0(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    /*
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
     */
    todo!()
}

fn jdk_internal_misc_signal_handle_0(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.Signal.handle0");
    Ok(Some(Value::Long(1)))
}

fn jdk_internal_misc_cds_get_cds_config_status(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,

    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.CDS.getCDSConfigStatus");
    Ok(Some(Value::Integer(0)))
}

fn jdk_internal_misc_cds_initialize_from_archive(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,

    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.CDS.initializeFromArchive");
    Ok(None)
}
