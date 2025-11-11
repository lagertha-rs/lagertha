use crate::native::{NativeRegistry, NativeRet};
use crate::stack_deprecated::FrameStackDeprecated;
use crate::{FieldKey, FullyQualifiedMethodKey, ThreadId, VirtualMachine};
use common::jtype::Value;
use log::debug;

pub(super) fn do_register_java_io_preregistered_natives(native_registry: &mut NativeRegistry) {
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/io/FileOutputStream",
            "writeBytes",
            "([BIIZ)V",
            &native_registry.string_interner,
        ),
        java_io_file_output_stream_write_bytes,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/io/FileInputStream",
            "initIDs",
            "()V",
            &native_registry.string_interner,
        ),
        java_io_file_input_stream_init_ids,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/io/FileDescriptor",
            "initIDs",
            "()V",
            &native_registry.string_interner,
        ),
        java_io_file_descriptor_init_ids,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/io/FileDescriptor",
            "getHandle",
            "(I)J",
            &native_registry.string_interner,
        ),
        java_io_file_descriptor_get_handle,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/io/FileDescriptor",
            "getAppend",
            "(I)Z",
            &native_registry.string_interner,
        ),
        java_io_file_descriptor_get_append,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/io/FileOutputStream",
            "initIDs",
            "()V",
            &native_registry.string_interner,
        ),
        java_io_file_output_stream_init_ids,
    );
}

fn java_io_file_output_stream_write_bytes(
    vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Partial implementation: java.io.FileOutputStream.writeBytes");
    let output_stream_ref = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("java.io.FileOutputStream.writeBytes: expected FileDescriptor object"),
    };
    let bytes_array = match &args[1] {
        Value::Ref(h) => *h,
        _ => panic!("java.io.FileOutputStream.writeBytes: expected byte array"),
    };
    let offset = match args[2] {
        Value::Integer(i) if i >= 0 => i as usize,
        _ => panic!("java.io.FileOutputStream.writeBytes: expected non-negative offset"),
    };
    let length = match args[3] {
        Value::Integer(i) if i >= 0 => i as usize,
        _ => panic!("java.io.FileOutputStream.writeBytes: expected non-negative length"),
    };

    let output_stream_class_id = vm.heap.get_class_id(&output_stream_ref)?;
    let output_stream_fd_field_offset = vm
        .method_area
        .get_instance_class(&output_stream_class_id)?
        .get_instance_field_offset(&vm.method_area.br().file_output_stream_fd_fk)?;
    let fd_obj = vm
        .heap
        .get_instance(&output_stream_ref)?
        .get_element(output_stream_fd_field_offset as i32)?
        .as_obj_ref()?;
    let fd_class_id = vm.heap.get_class_id(&fd_obj)?;
    let fd_fd_field_offset = vm
        .method_area
        .get_instance_class(&fd_class_id)?
        .get_instance_field_offset(&vm.method_area.br().fd_fd_fk)?;
    let fd_val = vm
        .heap
        .get_instance(&fd_obj)?
        .get_element(fd_fd_field_offset as i32)?
        .as_int()?;
    let array = vm.heap.get_array(&bytes_array)?;
    for i in offset..offset + length {
        let byte = match array.get_element(i as i32).unwrap() {
            Value::Integer(b) => b,
            _ => panic!("java.io.FileOutputStream.writeBytes: expected byte element"),
        };
        if fd_val == 1 {
            print!("{}", *byte as u8 as char);
        } else if fd_val == 2 {
            eprint!("{}", *byte as u8 as char);
        } else {
            unimplemented!(
                "java.io.FileOutputStream.writeBytes: only stdout and stderr are supported"
            );
        }
    }

    Ok(None)
}

fn java_io_file_input_stream_init_ids(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.io.FileInputStream.initIDs");
    Ok(None)
}

fn java_io_file_descriptor_init_ids(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.io.FileDescriptor.initIDs");
    Ok(None)
}

fn java_io_file_descriptor_get_handle(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.io.FileDescriptor.getHandle");
    Ok(Some(Value::Long(0)))
}

fn java_io_file_descriptor_get_append(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.io.FileDescriptor.getAppend");
    Ok(Some(Value::Integer(0)))
}

fn java_io_file_output_stream_init_ids(
    _vm: &mut VirtualMachine,
    _thread_id: ThreadId,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.io.FileInputStream.initIDs");
    Ok(None)
}
