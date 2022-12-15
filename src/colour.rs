use vips_sys as s;

use crate::{
    VipsImage,
    define_operator
};

impl VipsImage {
    pub fn colourspace_issupported(&self) -> bool {
        unsafe {
            s::vips_colourspace_issupported(self.ptr) != 0
        }
    }
}

define_operator!(colourspace, pub struct ColourSpaceArgs {
    pub space: s::VipsInterpretation,
    pub source_space: Option<s::VipsInterpretation>
});

#[cfg(test)]
mod colour_tests {
    use std::path::PathBuf;
    use vips_sys as s;
    use super::*;
    use crate::ensure_vips_init_or_exit;

    #[test]
    fn colourspace() {
        ensure_vips_init_or_exit();

        let img = VipsImage::new_from_file(PathBuf::from("./data/test.jpg"))
            .expect("Image could not be created from file");
        assert_ne!(img.ptr, std::ptr::null_mut());

        let result = img.colourspace(colourspace::OpArgs{
            space: s::VipsInterpretation_VIPS_INTERPRETATION_RGB16,
            source_space: None
        })
        .expect("Could not change colourspace");

        result.write_to_file(PathBuf::from("./data/test_result.tif"))
            .expect("Could not save image to file");
    }
}
