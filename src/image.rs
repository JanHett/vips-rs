use std::ffi::{CString, c_void};
use std::path::PathBuf;

use vips_sys as s;

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
    // --- Image creation ---

    // TODO: decide if this should exist
    // fn new() -> VipsImage {
    //     VipsImage{ptr: std::ptr::null_mut()}
    // }

    /// Wrap a `VipsImage` around a pointer to a `vips_sys::VipsImage` and take
    /// ownership of this pointer
    pub fn from_c_ptr(p: *mut vips_sys::VipsImage) -> Result<VipsImage, VipsError> {
        if p == std::ptr::null_mut() {
            return Err(VipsError::new("Cannot wrap nullptr in VipsImage"));
        }
        Ok(VipsImage{ptr: p})
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
        })?)
    }

    pub fn new_matrix(width: i32, height: i32) -> Result<VipsImage, VipsError> {
        Ok(VipsImage::from_c_ptr(unsafe {
            s::vips_image_new_matrix(width, height)
        })?)
    }

    // --- Image output ---

    pub fn write_to_file(&self, file: PathBuf) -> Result<(), VipsError> {
        let filename_str = match file.to_str() {
            Some(filename_str) => filename_str,
            None => return Err(VipsError::new("Could not convert path to string"))
        };
        let filename_c_str = CString::new(filename_str)?;
        unsafe {
            if s::vips_image_write_to_file(self.ptr, filename_c_str.as_ptr(), 0) != 0
            {
                return Err(VipsError::new_from_vips_state());
            }
        }

        Ok(())
    }
}

// =============================================================================
// === Tests ===================================================================
// =============================================================================

#[cfg(test)]
mod image_tests {
    use crate::*;
    use std::path::PathBuf;

    // #[test]
    // fn image_new() {
    //     ensure_vips_init_or_exit();
    //     let image = VipsImage::new();
    //     // this doesn't make sense - we shouldn't wrap a nullptr
    //     assert_eq!(image.ptr, std::ptr::null_mut());
    // }

    #[test]
    fn image_from_c_ptr() {
        ensure_vips_init_or_exit();

        match VipsImage::from_c_ptr(std::ptr::null_mut()) {
            Ok(_) => panic!("Creating image from nullptr should error"),
            Err(_) => {}
        }
    }

    #[test]
    fn vips_image_io() {
        ensure_vips_init_or_exit();
        unsafe { vips_sys::vips_leak_set(1) };

        let img = VipsImage::from_file(PathBuf::from("./data/test.jpg"))
            .expect("Image could not be created from file");
        assert_ne!(img.ptr, std::ptr::null_mut());

        img.write_to_file(PathBuf::from("./data/test_out.jpg"))
            .expect("Could not save image to file");
    }
}
