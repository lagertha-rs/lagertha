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
macro_rules! debug_print_method {
    ($method_id:expr) => {
        #[cfg(feature = "debug-log")]
        {
            debug_log::debug::with_method_area(|ma| {
                let method = ma.get_method(&$method_id);
                log::debug!("Method: {}", method.name_display(ma));
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
