/// Utilities for interacting with Vips

use std::ffi::{CString, CStr};
use std::sync::Once;

use crate::*;

// =============================================================================
// === INIT/SHUTDOWN ===========================================================
// =============================================================================

/// Manually initialize Vips
/// 
/// If the intialization fails, the Err will contain a `VipsError` wrapping the
/// Vips error buffer.
/// 
/// Tread carefully when calling this function as it does not check if Vips has
/// already been initialized before. Initializing Vips twice is undefined
/// behaviour.
pub fn vips_init() -> Result<(), VipsError> {
    let argv_0 = std::env::args().next();
    let c_argv_0 = match argv_0 {
        Some(argv) => CString::new(argv).unwrap_or_default(),
        None => CString::default()
    };

    if unsafe { vips_sys::vips_init(c_argv_0.as_ptr()) } != 0 {
        let err_descr = unsafe {
            CStr::from_ptr(vips_sys::vips_error_buffer())
        };
        match err_descr.to_str() {
            Ok(err_str) => Err(VipsError::new(err_str)),
            Err(e) => Err(VipsError::new(format!("Error parsing Vips error buffer: `{e}`")))
        }
    } else {
        Ok(())
    }
}

/// Manually initialize Vips and exit if the initialization fails
/// 
/// Tread carefully when calling this function as it does not check if Vips has
/// already been initialized before. Initializing Vips twice is undefined
/// behaviour.
pub fn vips_init_or_exit() {
    let argv_0 = std::env::args().next();
    let c_argv_0 = match argv_0 {
        Some(argv) => CString::new(argv).unwrap_or_default(),
        None => CString::default()
    };

    if unsafe { vips_sys::vips_init(c_argv_0.as_ptr()) } != 0 {
        unsafe { vips_sys::vips_error_exit(std::ptr::null()); }
    }
}

static VIPS_INIT: Once = Once::new();
/// Initializes Vips, making sure the internal init function gets called exactly
/// once
pub fn ensure_vips_init_or_exit() {
    VIPS_INIT.call_once(vips_init_or_exit);
}

pub fn vips_shutdown() {
    unsafe { vips_sys::vips_shutdown(); }
}

/// Initializes Vips when created with `VipsHandle::new()` and calls the
/// shutdown routine when it is dropped
/// 
/// There should only ever be one instance of this struct and it must live for
/// as long as you use Vips objects and their operations.
pub struct VipsHandle {}

impl VipsHandle {
    pub fn new() -> VipsHandle {
        let argv_0 = std::env::args().next();
        let c_argv_0 = match argv_0 {
            Some(argv) => CString::new(argv).unwrap_or_default(),
            None => CString::default()
        };

        if unsafe { vips_sys::vips_init(c_argv_0.as_ptr()) } != 0 {
            unsafe { vips_sys::vips_error_exit(std::ptr::null()); }
        }

        VipsHandle{}
    }
}

impl Drop for VipsHandle {
    fn drop(&mut self) {
        vips_shutdown();
    }
}
