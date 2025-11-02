use crate::native_deprecated::NativeRegistryDeprecated;
use crate::native_deprecated::preregistered::java_io::do_register_java_io_preregistered_natives;
use crate::native_deprecated::preregistered::java_lang::do_register_java_lang_preregistered_natives;
use crate::native_deprecated::preregistered::jdk_internal::do_register_jdk_internal_preregistered_natives;
use crate::native_deprecated::preregistered::vm_internal::do_register_vm_internal_preregistered_natives;

mod java_io;
mod java_lang;
mod jdk_internal;
mod vm_internal;

pub(super) fn preregister_natives(native_registry: &mut NativeRegistryDeprecated) {
    do_register_jdk_internal_preregistered_natives(native_registry);
    do_register_java_lang_preregistered_natives(native_registry);
    do_register_java_io_preregistered_natives(native_registry);
    do_register_vm_internal_preregistered_natives(native_registry);
}
