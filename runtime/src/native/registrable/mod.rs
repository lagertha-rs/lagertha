use crate::native::MethodKey;
use crate::native::registrable::java_lang_class::java_lang_class_register_natives;
use crate::native::registrable::java_lang_system::java_lang_system_register_natives;
use crate::native::registrable::java_lang_thread::java_lang_thread_register_natives;
use crate::native::registrable::jdk_internal_misc_scoped_memory_access::jdk_internal_misc_scoped_memory_access_register_natives;
use crate::native::registrable::jdk_internal_misc_unsafe::jdk_internal_misc_unsafe_register_natives;

mod java_lang_class;
mod java_lang_system;
mod java_lang_thread;
mod jdk_internal_misc_scoped_memory_access;
mod jdk_internal_misc_unsafe;

pub(super) fn add_registrable_natives(native_registry: &mut crate::native::NativeRegistry) {
    native_registry.register(
        MethodKey::new_with_str(
            "java/lang/System",
            "registerNatives",
            "()V",
            &native_registry.string_interner,
        ),
        java_lang_system_register_natives,
    );
    native_registry.register(
        MethodKey::new_with_str(
            "java/lang/Class",
            "registerNatives",
            "()V",
            &native_registry.string_interner,
        ),
        java_lang_class_register_natives,
    );

    native_registry.register(
        MethodKey::new_with_str(
            "jdk/internal/misc/Unsafe",
            "registerNatives",
            "()V",
            &native_registry.string_interner,
        ),
        jdk_internal_misc_unsafe_register_natives,
    );
    native_registry.register(
        MethodKey::new_with_str(
            "jdk/internal/misc/ScopedMemoryAccess",
            "registerNatives",
            "()V",
            &native_registry.string_interner,
        ),
        jdk_internal_misc_scoped_memory_access_register_natives,
    );
    native_registry.register(
        MethodKey::new_with_str(
            "java/lang/Thread",
            "registerNatives",
            "()V",
            &native_registry.string_interner,
        ),
        java_lang_thread_register_natives,
    )
}
