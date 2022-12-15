// most colour space conversion function names become unreadable in snake case
#![allow(non_snake_case)]

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

define_operator!(LabQ2sRGB);
define_operator!(rad2float);
define_operator!(float2rad);
define_operator!(LabS2LabQ);
define_operator!(LabQ2LabS);
define_operator!(LabQ2Lab);
define_operator!(Lab2LabQ);
define_operator!(LCh2Lab);
define_operator!(Lab2LCh);
define_operator!(Yxy2Lab);
define_operator!(CMC2XYZ);
define_operator!(Lab2XYZ);
define_operator!(XYZ2Lab);
define_operator!(XYZ2scRGB);
define_operator!(scRGB2sRGB);
define_operator!(scRGB2BW);
define_operator!(sRGB2scRGB);
define_operator!(scRGB2XYZ);
define_operator!(HSV2sRGB);
define_operator!(sRGB2HSV);
define_operator!(LCh2CMC);
define_operator!(CMC2LCh);
define_operator!(XYZ2Yxy);
define_operator!(Yxy2XYZ);
define_operator!(LabS2Lab);
define_operator!(Lab2LabS);
define_operator!(CMYK2XYZ);
define_operator!(XYZ2CMYK);

// TODO: int      vips_profile_load()
// TODO: int      vips_icc_present()
// TODO: int      vips_icc_transform()
// TODO: int      vips_icc_import()
// TODO: int      vips_icc_export()
// TODO: int      vips_icc_ac2rc()
// TODO: gboolean vips_icc_is_compatible_profile()
// TODO: int      vips_dE76()
// TODO: int      vips_dE00()
// TODO: int      vips_dECMC()
// TODO: void     vips_col_Lab2XYZ()
// TODO: void     vips_col_XYZ2Lab()
// TODO: double   vips_col_ab2h()
// TODO: void     vips_col_ab2Ch()
// TODO: void     vips_col_Ch2ab()
// TODO: float    vips_col_L2Lcmc()
// TODO: float    vips_col_C2Ccmc()
// TODO: float    vips_col_Ch2hcmc()
// TODO: void     vips_col_make_tables_CMC()
// TODO: float    vips_col_Lcmc2L()
// TODO: float    vips_col_Ccmc2C()
// TODO: float    vips_col_Chcmc2h()
// TODO: int      vips_col_sRGB2scRGB_8()
// TODO: int      vips_col_sRGB2scRGB_16()
// TODO: int      vips_col_sRGB2scRGB_8_noclip()
// TODO: int      vips_col_sRGB2scRGB_16_noclip()
// TODO: int      vips_col_scRGB2XYZ()
// TODO: int      vips_col_XYZ2scRGB()
// TODO: int      vips_col_scRGB2sRGB_8()
// TODO: int      vips_col_scRGB2sRGB_16()
// TODO: int      vips_col_scRGB2BW_16()
// TODO: int      vips_col_scRGB2BW_8()
// TODO: float    vips_pythagoras()
// TODO: float    vips_col_dE00()

#[cfg(test)]
mod colour_tests {
    use std::path::PathBuf;
    use vips_sys as s;
    use super::*;
    use crate::ensure_vips_init_or_exit;

    #[test]
    fn colourspace_issupported() {
        ensure_vips_init_or_exit();

        let img = VipsImage::new_from_file(PathBuf::from("./data/test.jpg"))
            .expect("Image could not be created from file");
        assert_ne!(img.ptr, std::ptr::null_mut());

        assert!(img.colourspace_issupported());
    }

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
