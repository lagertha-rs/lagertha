use crate::VirtualMachine;
use crate::keys::FullyQualifiedMethodKey;
use crate::native::{NativeRegistry, NativeRet};
use crate::thread::JavaThreadState;
use crate::vm::Value;
use common::jtype::AllocationType;

pub(super) fn do_register_java_lang_ref_preregistered_natives(
    native_registry: &mut NativeRegistry,
) {
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/lang/ref/Reference",
            "refersTo0",
            "(Ljava/lang/Object;)Z",
            &native_registry.string_interner,
        ),
        java_lang_ref_reference_refers_to_0,
    )
}

fn java_lang_ref_reference_refers_to_0(
    vm: &mut VirtualMachine,
    _thread: &mut JavaThreadState,
    args: &[Value],
) -> NativeRet {
    let referent_ref = args[0].as_obj_ref()?;
    let referent_fk = vm.br.reference_referent_fk;
    let reference_class_id = vm
        .method_area_write()
        .get_class_id_or_load(vm.br.java_lang_ref_reference_sym)?;
    let referent_field_offset = vm
        .method_area_read()
        .get_instance_class(&reference_class_id)?
        .get_instance_field(&referent_fk)?
        .offset;
    let referent_value = vm.heap_read().read_field(
        referent_ref,
        referent_field_offset,
        AllocationType::Reference,
    )?;
    let o = args[1].as_obj_ref()?;
    Ok(Some(Value::Integer(if referent_value.as_obj_ref()? == o {
        1
    } else {
        0
    })))
}
