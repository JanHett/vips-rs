use std::error::Error;
use std::ffi::CStr;

// =============================================================================
// === VipsError ===============================================================
// =============================================================================

#[derive(Debug)]
pub struct VipsError {
    description: String
}

// === Traits ==================================================================

impl std::fmt::Display for VipsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.description))
    }
}

impl From<std::ffi::NulError> for VipsError {
    fn from(err: std::ffi::NulError) -> Self {
        VipsError::new(err.to_string())
    }
}

// === Implementation ===========================================================

impl VipsError {
    pub fn description(&self) -> &str { self.description.as_str() }

    pub fn new<T: Into<String>>(description: T) -> VipsError {
        VipsError { description: description.into() }
    }

    pub fn new_from_vips_state() -> VipsError {
        let err_descr = unsafe {
            CStr::from_ptr(vips_sys::vips_error_buffer())
        };
        match err_descr.to_str() {
            Ok(err_str) => VipsError::new(err_str),
            Err(e) => VipsError::new(format!("Error parsing Vips error buffer: `{e}`"))
        }
    }
}
