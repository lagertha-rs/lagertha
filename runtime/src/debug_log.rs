#[cfg(feature = "debug-log")]
pub(crate) mod debug {
    use crate::MethodArea;
    use std::sync::atomic::{AtomicPtr, Ordering};

    static METHOD_AREA: AtomicPtr<MethodArea> = AtomicPtr::new(std::ptr::null_mut());

    pub fn init(ma: &MethodArea) {
        METHOD_AREA.store(
            ma as *const MethodArea as *mut MethodArea,
            Ordering::Release,
        );
    }

    pub fn with_method_area<F, R>(f: F) -> Option<R>
    where
        F: FnOnce(&MethodArea) -> R,
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
            crate::debug_log::debug::with_method_area(|ma| {
                let method = ma.get_method(&$method_id);
                let class_name = ma
                    .interner()
                    .resolve(ma.get_class(&method.class_id()).get_name());
                let method_name = ma.interner().resolve(&method.name);
                let signature = ma
                    .get_method_descriptor(&method.descriptor_id())
                    .to_java_signature(method_name);

                log::debug!("{}: {} of {}", $msg, signature, class_name);
            });
        }
    };
}

#[macro_export]
macro_rules! debug_print_string {
    ($interned:expr) => {
        #[cfg(feature = "debug-log")]
        {
            debug_log::debug::with_method_area(|ma| {
                log::debug!("{}", ma.interner().resolve(&$interned));
            });
        }
    };
}
