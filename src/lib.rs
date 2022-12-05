#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::ffi::CString;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_vips_io() {
        unsafe {
            let init_arg = CString::new("").unwrap();
            vips_init(init_arg.as_ptr());

            let test_img_path = CString::new("./data/test.jpg").unwrap();
            let in_img = vips_image_new_from_file(test_img_path.as_ptr());
            
            let width = vips_image_get_width(in_img);
            println!("width: {}", width);
            // assert_eq!(width, 128);

            let mut resized: *mut _VipsImage = std::ptr::null_mut();
            vips_resize(in_img, &mut resized as *mut*mut _VipsImage, 0.25);

            let out_filename = CString::new("./data/test_out.jpg").unwrap();
            vips_image_write_to_file(resized, out_filename.as_ptr());
            
            vips_shutdown();
        }
    }
}
