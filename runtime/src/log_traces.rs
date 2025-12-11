#[cfg(feature = "log-runtime-traces")]
pub(crate) mod debug {
    use crate::VirtualMachine;
    use std::sync::atomic::{AtomicPtr, Ordering};

    static VM: AtomicPtr<VirtualMachine> = AtomicPtr::new(std::ptr::null_mut());

    pub fn init(ma: &VirtualMachine) {
        VM.store(
            ma as *const VirtualMachine as *mut VirtualMachine,
            Ordering::Release,
        );
    }

    pub fn with_vm<F, R>(f: F) -> Option<R>
    where
        F: FnOnce(&VirtualMachine) -> R,
    {
        let ptr = VM.load(Ordering::Acquire);
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
        #[cfg(feature = "log-runtime-traces")]
        {
            tracing_log::log::debug!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! debug_error_log {
    ($($arg:tt)*) => {
        #[cfg(feature = "log-runtime-traces")]
        {
            tracing_log::log::error!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! debug_log_method {
    ($method_id:expr, $msg:expr) => {
        #[cfg(feature = "log-runtime-traces")]
        {
            crate::log_traces::debug::with_vm(|vm| {
                let ma = vm.method_area_read();
                let method = ma.get_method($method_id);
                let class_name = vm
                    .interner()
                    .resolve(&ma.get_class(&method.class_id()).get_name());
                let method_name = vm.interner().resolve(&method.name);
                let signature = ma
                    .get_method_descriptor(&method.descriptor_id())
                    .to_java_signature(method_name, class_name);

                tracing_log::log::debug!("{}: {} of {}", $msg, signature, class_name);
            });
        }
    };
}

#[macro_export]
macro_rules! error_log_method {
    ($method_id:expr, $exception:expr, $msg:expr) => {
        #[cfg(feature = "log-runtime-traces")]
        {
            crate::log_traces::debug::with_vm(|vm| {
                let ma = vm.method_area_read();
                let method = ma.get_method($method_id);
                let class_name = vm
                    .interner()
                    .resolve(&ma.get_class(&method.class_id()).get_name());
                let method_name = vm.interner().resolve(&method.name);
                let signature = ma
                    .get_method_descriptor(&method.descriptor_id())
                    .to_java_signature(method_name, class_name);

                let exp_class_name = if let JvmError::JavaExceptionThrown(hr) = $exception {
                    let excp_id = vm.heap_read().get_class_id(*hr).unwrap();
                    let excp_class_name = ma.get_class(&excp_id).get_name();
                    vm.interner().resolve(&excp_class_name).to_string()
                } else {
                    format!("{:?}", $exception)
                };

                tracing_log::log::error!(
                    "{}. Exception: {} in {} of {}",
                    $msg,
                    exp_class_name,
                    signature,
                    class_name
                );
            });
        }
    };
}

#[macro_export]
macro_rules! debug_log_instruction {
    ($instruction:expr, $thread_id:expr) => {
        #[cfg(feature = "log-runtime-traces")]
        {
            crate::log_traces::debug::with_vm(|vm| {
                let mut msg_chunks = vec![format!("{:?}", $instruction)];
                match $instruction {
                    common::instruction::Instruction::Getstatic(idx) => {
                        let cur_frame_method_id = vm
                            .get_stack($thread_id)
                            .unwrap()
                            .cur_java_frame()
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
                    common::instruction::Instruction::InvokeSpecial(idx) => {
                        let cur_frame_method_id = vm
                            .get_stack($thread_id)
                            .unwrap()
                            .cur_java_frame()
                            .unwrap()
                            .method_id();
                        let target_method_view = vm
                            .method_area
                            .get_cp_by_method_id(&cur_frame_method_id)
                            .unwrap()
                            .get_method_view(&idx, vm.interner())
                            .unwrap();
                        msg_chunks.push(format!(
                            "Method {} {} of {}",
                            vm.interner()
                                .resolve(&target_method_view.name_and_type.name_sym),
                            vm.interner()
                                .resolve(&target_method_view.name_and_type.descriptor_sym),
                            vm.interner().resolve(&target_method_view.class_sym),
                        ));
                    }
                    common::instruction::Instruction::InvokeVirtual(idx) => {
                        let cur_frame_method_id = vm
                            .get_stack($thread_id)
                            .unwrap()
                            .cur_java_frame()
                            .unwrap()
                            .method_id();
                        let target_method_view = vm
                            .method_area
                            .get_cp_by_method_id(&cur_frame_method_id)
                            .unwrap()
                            .get_method_view(&idx, vm.interner())
                            .unwrap();
                        msg_chunks.push(format!(
                            "Method {} {} of {}",
                            vm.interner()
                                .resolve(&target_method_view.name_and_type.name_sym),
                            vm.interner()
                                .resolve(&target_method_view.name_and_type.descriptor_sym),
                            vm.interner().resolve(&target_method_view.class_sym),
                        ));
                    }
                    common::instruction::Instruction::InvokeStatic(idx) => {
                        let cur_frame_method_id = vm
                            .get_stack($thread_id)
                            .unwrap()
                            .cur_java_frame()
                            .unwrap()
                            .method_id();
                        let target_method_view = vm
                            .method_area
                            .get_cp_by_method_id(&cur_frame_method_id)
                            .unwrap()
                            .get_method_or_interface_method_view(&idx, vm.interner())
                            .unwrap();
                        msg_chunks.push(format!(
                            "Method {} {} of {}",
                            vm.interner()
                                .resolve(&target_method_view.name_and_type.name_sym),
                            vm.interner()
                                .resolve(&target_method_view.name_and_type.descriptor_sym),
                            vm.interner().resolve(&target_method_view.class_sym),
                        ));
                    }
                    common::instruction::Instruction::InvokeInterface(idx, count) => {
                        let cur_frame_method_id = vm
                            .get_stack($thread_id)
                            .unwrap()
                            .cur_java_frame()
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
                            .peek_operand_at(*count as usize - 1)
                            .unwrap()
                            .as_obj_ref()
                            .unwrap();
                        if object_ref != 0 {
                        let actual_class_id = vm.heap.get_class_id(object_ref).unwrap();
                        let actual_class_name =
                            vm.method_area.get_class(&actual_class_id).get_name();
                        msg_chunks.push(format!(
                            "Method {} of {} (for actual class {})",
                            vm.interner()
                                .resolve(&target_method_view.name_and_type.name_sym),
                            vm.interner().resolve(&target_method_view.class_sym),
                            vm.interner().resolve(&actual_class_name)
                        ));
                            } else {
                        msg_chunks.push(format!("Object ref is zero, because of the stub of java lang access I guess"));
                        }
                    }

                    _ => {}
                }
                tracing_log::log::debug!(target: "bytecode","Executing: {}", msg_chunks.join(" "));
            });
        }
    };
}
