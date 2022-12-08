use std::ffi::{CString, c_void};
use std::path::PathBuf;

use crate::*;

// =============================================================================
// === VipsImage ===============================================================
// =============================================================================

/// Safe wrapper around the pointer to the internal `VipsImage*`
pub struct VipsImage {
    pub ptr: *mut vips_sys::VipsImage
}

// === Traits ==================================================================

impl Drop for VipsImage {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { vips_sys::g_object_unref(self.ptr as *mut c_void) };
        }
    }
}

// === Custom behaviour ========================================================

impl VipsImage {
    pub fn new() -> VipsImage {
        // calling vips_init since we are creating a VipsImage directly
        // vips_init(); // TODO: this crashes
        VipsImage{ptr: std::ptr::null_mut()}
    }

    pub fn from_c_ptr(p: *mut vips_sys::VipsImage) -> VipsImage {
        // calling vips_init since we are creating a VipsImage directly
        // vips_init(); // TODO: this crashes
        VipsImage{ptr: p}
    }

    pub fn from_file(file: PathBuf) -> Result<VipsImage, VipsError> {
        let path_str = match file.to_str() {
            Some(pstr) => pstr,
            None => return Err(VipsError::new("Could not convert path to string"))
        };

        let path_c_str = match CString::new(path_str) {
            Ok(c_str) => c_str,
            Err(_) => return Err(VipsError::new("Could not convert path to CString"))
        };

        Ok(VipsImage::from_c_ptr(unsafe {
            vips_sys::vips_image_new_from_file(path_c_str.as_ptr(), 0)
        }))
    }
}

// =============================================================================
// === Tests ===================================================================
// =============================================================================

#[cfg(test)]
mod image_tests {
    use crate::*;
    use std::path::PathBuf;

    #[test]
    fn image_new() {
        ensure_vips_init_or_exit();
        let image = VipsImage::new();
        assert_eq!(image.ptr, std::ptr::null_mut());
    }

    #[test]
    fn vips_image_io() {
        ensure_vips_init_or_exit();
        unsafe { vips_sys::vips_leak_set(1) };

        let img = VipsImage::from_file(PathBuf::from("./data/test.jpg"))
            .expect("Image could not be created from file");
        assert_ne!(img.ptr, std::ptr::null_mut());

        // TODO: write the file
    }
}
