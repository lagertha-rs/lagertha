#[cfg(feature = "debug-log")]
pub(crate) mod debug {
    use crate::VirtualMachine;
    use std::sync::atomic::{AtomicPtr, Ordering};

    static METHOD_AREA: AtomicPtr<VirtualMachine> = AtomicPtr::new(std::ptr::null_mut());

    pub fn init(ma: &VirtualMachine) {
        METHOD_AREA.store(
            ma as *const VirtualMachine as *mut VirtualMachine,
            Ordering::Release,
        );
    }

    pub fn with_vm<F, R>(f: F) -> Option<R>
    where
        F: FnOnce(&VirtualMachine) -> R,
    {
        let ptr = METHOD_AREA.load(Ordering::Acquire);
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { f(&*ptr) })
        }
    }
}

#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug-log")]
        {
            log::debug!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! debug_log_method {
    ($method_id:expr, $msg:expr) => {
        #[cfg(feature = "debug-log")]
        {
            crate::debug_log::debug::with_vm(|vm| {
                let method = vm.method_area.get_method($method_id);
                let class_name = vm
                    .interner()
                    .resolve(vm.method_area.get_class(&method.class_id()).get_name());
                let method_name = vm.interner().resolve(&method.name);
                let signature = vm
                    .method_area
                    .get_method_descriptor(&method.descriptor_id())
                    .to_java_signature(method_name);

                log::debug!("{}: {} of {}", $msg, signature, class_name);
            });
        }
    };
}

#[macro_export]
macro_rules! debug_log_instruction {
    ($instruction:expr, $thread_id:expr) => {
        #[cfg(feature = "debug-log")]
        {
            crate::debug_log::debug::with_vm(|vm| {
                let mut msg_chunks = vec![format!("{:?}", $instruction)];
                match $instruction {
                    common::instruction::Instruction::Getstatic(idx) => {
                        let cur_frame_method_id = vm
                            .get_stack($thread_id)
                            .unwrap()
                            .cur_frame()
                            .unwrap()
                            .method_id();
                        let target_field_view = vm
                            .method_area
                            .get_cp_by_method_id(&cur_frame_method_id)
                            .unwrap()
                            .get_field_view(&idx, vm.interner())
                            .unwrap();
                        msg_chunks.push(format!(
                            "Field {} {} of {}",
                            vm.interner()
                                .resolve(&target_field_view.name_and_type.descriptor_sym),
                            vm.interner()
                                .resolve(&target_field_view.name_and_type.name_sym),
                            vm.interner().resolve(&target_field_view.class_sym),
                        ));
                    }
                    common::instruction::Instruction::InvokeInterface(idx, count) => {
                        let cur_frame_method_id = vm
                            .get_stack($thread_id)
                            .unwrap()
                            .cur_frame()
                            .unwrap()
                            .method_id();
                        let target_method_view = vm
                            .method_area
                            .get_cp_by_method_id(&cur_frame_method_id)
                            .unwrap()
                            .get_interface_method_view(&idx, vm.interner())
                            .unwrap();
                        let object_ref = vm
                            .get_stack($thread_id)
                            .unwrap()
                            .peek_at(*count as usize - 1)
                            .unwrap()
                            .as_obj_ref()
                            .unwrap();
                        let actual_class_id = vm.heap.get_class_id(&object_ref).unwrap();
                        let actual_class_name =
                            vm.method_area.get_class(&actual_class_id).get_name();
                        msg_chunks.push(format!(
                            "Method {} of {} (for actual class {})",
                            vm.interner()
                                .resolve(&target_method_view.name_and_type.name_sym),
                            vm.interner().resolve(&target_method_view.class_sym),
                            vm.interner().resolve(&actual_class_name)
                        ));
                    }

                    _ => {}
                }

                log::debug!("Executing: {}", msg_chunks.join(" "));
            });
        }
    };
}
