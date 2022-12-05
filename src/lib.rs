#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// TODO: solve or suppress u128 isn't FFI-safe warning

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod vips_sys_tests {
    use super::*;

    use std::ffi::CString;

    #[test]
    fn test_vips_io() {
        unsafe {
            let init_arg = CString::new(std::env::args().next().unwrap()).unwrap();
            vips_init(init_arg.as_ptr());

            let test_img_path = CString::new("./data/test.jpg").unwrap();
            let in_img = vips_image_new_from_file(test_img_path.as_ptr(), 0);
            
            let input_width = vips_image_get_width(in_img);
            assert_eq!(input_width, 385);
            let input_height = vips_image_get_height(in_img);
            assert_eq!(input_height, 512);

            let mut resized: *mut VipsImage = std::ptr::null_mut();
            vips_resize(in_img, &mut resized as *mut*mut VipsImage, 0.25, 0);

            let resized_width = vips_image_get_width(resized);
            assert_eq!(resized_width, 96);
            let resized_height = vips_image_get_height(resized);
            assert_eq!(resized_height, 128);

            let out_filename_str = "./data/test_out.jpg";
            let out_filename = CString::new(out_filename_str).unwrap();
            vips_image_write_to_file(resized, out_filename.as_ptr(), 0);

            std::fs::remove_file(std::path::PathBuf::from(out_filename_str)).unwrap();
            
            vips_shutdown();
        }
    }
}
