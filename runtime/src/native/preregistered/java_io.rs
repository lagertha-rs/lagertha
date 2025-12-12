use crate::keys::FullyQualifiedMethodKey;
use crate::native::{NativeRegistry, NativeRet};
use crate::thread::JavaThreadState;
use crate::vm::Value;
use crate::{ThreadId, VirtualMachine, throw_exception};
use common::jtype::AllocationType;
use tracing_log::log::debug;

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
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/io/UnixFileSystem",
            "initIDs",
            "()V",
            &native_registry.string_interner,
        ),
        java_io_unix_file_system_init_ids,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/io/UnixFileSystem",
            "canonicalize0",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &native_registry.string_interner,
        ),
        java_io_unix_file_system_canonicalize_0,
    );
    native_registry.register(
        FullyQualifiedMethodKey::new_with_str(
            "java/io/UnixFileSystem",
            "getBooleanAttributes0",
            "(Ljava/io/File;)I",
            &native_registry.string_interner,
        ),
        java_io_unix_file_system_get_boolean_attributes_0,
    )
}

fn java_io_file_output_stream_write_bytes(
    vm: &VirtualMachine,
    _thread: &mut JavaThreadState,
    args: &[Value],
) -> NativeRet {
    debug!("TODO: Partial implementation: java.io.FileOutputStream.writeBytes");
    let output_stream_ref = match &args[0] {
        Value::Ref(h) => *h,
        _ => panic!("java.io.FileOutputStream.writeBytes: expected FileOutputStream object"),
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

    let output_stream_class_id = vm.heap_read().get_class_id(output_stream_ref)?;
    let output_stream_fd_field_offset = vm
        .method_area_read()
        .get_instance_class(&output_stream_class_id)?
        .get_instance_field(&vm.br().file_output_stream_fd_fk)?
        .offset;
    let fd_obj = vm
        .heap_read()
        .read_field(
            output_stream_ref,
            output_stream_fd_field_offset,
            AllocationType::Reference,
        )?
        .as_obj_ref()?;
    let fd_class_id = vm.heap_read().get_class_id(fd_obj)?;
    let fd_fd_field_offset = vm
        .method_area_read()
        .get_instance_class(&fd_class_id)?
        .get_instance_field(&vm.br().fd_fd_fk)?
        .offset;
    let fd_val = vm
        .heap_read()
        .read_field(fd_obj, fd_fd_field_offset, AllocationType::Int)?
        .as_int()?;

    let heap_read = vm.heap_read();
    let byte_slice = heap_read.get_byte_array_slice(bytes_array)?;

    if offset + length > byte_slice.len() {
        panic!("writeBytes: offset + length exceeds array bounds");
    }
    let bytes_to_write = &byte_slice[offset..offset + length];

    let unsigned_bytes: Vec<u8> = bytes_to_write.iter().map(|&b| b as u8).collect();

    use std::io::Write;
    if fd_val == 1 {
        std::io::stdout()
            .write_all(&unsigned_bytes)
            .expect("Failed to write to stdout");
        std::io::stdout().flush().expect("Failed to flush stdout");
    } else if fd_val == 2 {
        std::io::stderr()
            .write_all(&unsigned_bytes)
            .expect("Failed to write to stderr");
        std::io::stderr().flush().expect("Failed to flush stderr");
    } else {
        unimplemented!("java.io.FileOutputStream.writeBytes: only stdout and stderr are supported");
    }

    Ok(None)
}

fn java_io_file_input_stream_init_ids(
    _vm: &VirtualMachine,
    _thread: &mut JavaThreadState,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.io.FileInputStream.initIDs");
    Ok(None)
}

fn java_io_file_descriptor_init_ids(
    _vm: &VirtualMachine,
    _thread: &mut JavaThreadState,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.io.FileDescriptor.initIDs");
    Ok(None)
}

fn java_io_file_descriptor_get_handle(
    _vm: &VirtualMachine,
    _thread: &mut JavaThreadState,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.io.FileDescriptor.getHandle");
    Ok(Some(Value::Long(0)))
}

fn java_io_file_descriptor_get_append(
    _vm: &VirtualMachine,
    _thread: &mut JavaThreadState,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.io.FileDescriptor.getAppend");
    Ok(Some(Value::Integer(0)))
}

fn java_io_file_output_stream_init_ids(
    _vm: &VirtualMachine,
    _thread: &mut JavaThreadState,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.io.FileInputStream.initIDs");
    Ok(None)
}

fn java_io_unix_file_system_init_ids(
    _vm: &VirtualMachine,
    _thread: &mut JavaThreadState,
    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: java.io.UnixFileSystem.initIDs");
    Ok(None)
}

fn java_io_unix_file_system_canonicalize_0(
    vm: &VirtualMachine,
    _thread: &mut JavaThreadState,
    args: &[Value],
) -> NativeRet {
    let str_ref = args[1].as_obj_ref()?;
    let path = vm.heap_read().get_rust_string_from_java_string(str_ref)?;
    match std::fs::canonicalize(&path) {
        Ok(canonical) => {
            let res = canonical.to_string_lossy().to_string();
            let res_ref = vm.heap_write().alloc_string(&res)?;
            Ok(Some(Value::Ref(res_ref)))
        }
        Err(e) => {
            throw_exception!(IOException, e.to_string())
        }
    }
}

fn java_io_unix_file_system_get_boolean_attributes_0(
    vm: &VirtualMachine,
    _thread: &mut JavaThreadState,
    args: &[Value],
) -> NativeRet {
    const BA_EXISTS: i32 = 0x01;
    const BA_REGULAR: i32 = 0x02;
    const BA_DIRECTORY: i32 = 0x04;
    const BA_HIDDEN: i32 = 0x08;

    let file_class_id = vm
        .method_area_write()
        .get_class_id_or_load(vm.br.java_io_file_sym)?;
    let path_field_offset = vm
        .method_area_read()
        .get_instance_class(&file_class_id)?
        .get_instance_field(&vm.br.file_path_fk)?
        .offset;
    let file_ref = args[1].as_obj_ref()?;
    let path_ref = vm
        .heap_read()
        .read_field(file_ref, path_field_offset, AllocationType::Reference)?
        .as_obj_ref()?;
    let path_str = vm.heap_read().get_rust_string_from_java_string(path_ref)?;
    let path = std::path::Path::new(&path_str);

    let mut attrs = 0;

    if let Ok(metadata) = std::fs::metadata(path) {
        attrs |= BA_EXISTS;
        if metadata.is_file() {
            attrs |= BA_REGULAR;
        }
        if metadata.is_dir() {
            attrs |= BA_DIRECTORY;
        }
    }

    // Unix hidden = starts with '.'
    if let Some(name) = path.file_name() {
        if name.to_string_lossy().starts_with('.') {
            attrs |= BA_HIDDEN;
        }
    }

    Ok(Some(Value::Integer(attrs)))
}
