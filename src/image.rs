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

impl Default for VipsImage {
    fn default() -> Self {
        VipsImage::new()
    }
}

impl Clone for VipsImage {
    fn clone(&self) -> Self {
        unsafe { s::g_object_ref(self.ptr as *mut c_void) };

        VipsImage{ ptr: self.ptr }
    }
}

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

    /// Wrap a `VipsImage` around a pointer to a `vips_sys::VipsImage`. If this
    /// pointer has not yet been reffed, you must `vips_sys::g_object_ref(p)`
    /// before passing it to this function. `VipsImage` will take care of
    /// unreffing the underlying `*mut vips_sys::VipsImage` when the `VipsImage`
    /// instance is dropped.
    pub fn from_c_ptr(p: *mut vips_sys::VipsImage) -> Result<VipsImage, VipsError> {
        if p == std::ptr::null_mut() {
            return Err(VipsError::new("Cannot wrap nullptr in VipsImage"));
        }
        Ok(VipsImage{ptr: p})
    }

    pub fn new() -> VipsImage {
        //  TODO: make sure that vips_image_new really doesn't ever return a nullptr
        VipsImage::from_c_ptr(unsafe {s::vips_image_new()})
            .expect("Unexpected nullptr returned from `vips_image_new` -
            something is very wrong because this should never happen")
    }

    // TODO: vips_image_new_memory()
    // TODO: vips_image_memory()

    pub fn new_from_file(file: PathBuf) -> Result<VipsImage, VipsError> {
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

    // TODO: vips_image_new_from_file_RW()
    // TODO: vips_image_new_from_file_raw()

    pub fn new_from_memory<'a>(
        data: &'a [u8],
        width: i32,
        height: i32,
        bands: i32,
        band_format: s::VipsBandFormat
    ) -> Result<VipsImage, VipsError>{
        VipsImage::from_c_ptr(unsafe { s::vips_image_new_from_memory(
            data.as_ptr() as *const c_void, data.len(),
            width, height, bands, band_format
        ) })
    }

    pub fn new_from_memory_copy(
        data: &[u8],
        width: i32,
        height: i32,
        bands: i32,
        band_format: s::VipsBandFormat
    ) -> Result<VipsImage, VipsError>{
        VipsImage::from_c_ptr(unsafe { s::vips_image_new_from_memory_copy(
            data.as_ptr() as *const c_void, data.len(),
            width, height, bands, band_format
        ) })
    }

    // TODO: vips_image_new_from_buffer()
    // TODO: vips_image_new_from_source()

    pub fn new_matrix(width: i32, height: i32) -> Result<VipsImage, VipsError> {
        Ok(VipsImage::from_c_ptr(unsafe {
            s::vips_image_new_matrix(width, height)
        })?)
    }

    // TODO: vips_image_new_matrixv()
    pub fn new_matrix_from_array(width: i32, height: i32, array: &[f64]) -> Result<VipsImage, VipsError> {
        let arr_len:i32 = match array.len().try_into() {
            Ok(l) => l,
            Err(e) => return Err(VipsError::new(
                format!("Could not convert convert `array.len()` to i32: {e}")
            ))
        };

        Ok(VipsImage::from_c_ptr(unsafe {
            s::vips_image_new_matrix_from_array(width, height, array.as_ptr(), arr_len)
        })?)
    }

    pub fn new_from_image(image: &VipsImage, c: &[f64]) -> Result<VipsImage, VipsError> {
        let arr_len:i32 = match c.len().try_into() {
            Ok(l) => l,
            Err(e) => return Err(VipsError::new(
                format!("Could not convert convert `array.len()` to i32: {e}")
            ))
        };

        Ok(VipsImage::from_c_ptr(unsafe {
            s::vips_image_new_from_image(image.ptr, c.as_ptr(), arr_len)
        })?)
    }
    
    pub fn new_from_image1(image: &VipsImage, c: f64) -> Result<VipsImage, VipsError> {
        Ok(VipsImage::from_c_ptr(unsafe {
            s::vips_image_new_from_image1(image.ptr, c)
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

    // --- Image Properties ---

    pub fn nbands(&self) -> usize {
        unsafe {
            s::vips_image_get_bands(self.ptr) as usize
        }
    }
}

// =============================================================================
// === Tests ===================================================================
// =============================================================================

#[cfg(test)]
mod tests {
    use crate::*;
    use std::path::PathBuf;

    #[test]
    fn image_new() {
        ensure_vips_init_or_exit();
        let image = VipsImage::new();
        assert_ne!(image.ptr, std::ptr::null_mut());
    }

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

        let img = VipsImage::new_from_file(PathBuf::from("./data/test.jpg"))
            .expect("Image could not be created from file");
        assert_ne!(img.ptr, std::ptr::null_mut());

        img.write_to_file(PathBuf::from("./data/test_out.jpg"))
            .expect("Could not save image to file");
    }
}
