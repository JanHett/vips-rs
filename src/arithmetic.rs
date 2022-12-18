use std::ops::{
    Add, Sub, Mul, Div,
    AddAssign, SubAssign, MulAssign, DivAssign
};

use crate::*;

// int vips_sum()
define_operator!(add, "left", struct Args<'a> {
    pub right: &'a VipsImage
});
define_operator!(subtract, "left", struct Args<'a> {
    pub right: &'a VipsImage
});
define_operator!(multiply, "left", struct Args<'a> {
    pub right: &'a VipsImage
});
define_operator!(divide, "left", struct Args<'a> {
    pub right: &'a VipsImage
});

// TODO: resolve naming conflict
// impl Add for &VipsImage {
//     type Output = VipsImage;
//     fn add(self, rhs: Self) -> Self::Output {
//         self.add(add::OpArgs{right: &rhs}).unwrap_or_default()
//     }
// }
impl AddAssign for &mut VipsImage {
    fn add_assign(&mut self, rhs: Self) {
        **self = self.add(add::OpArgs{right: &rhs}).unwrap_or_default();
    }
}
impl Sub<&VipsImage> for &VipsImage {
    type Output = VipsImage;
    fn sub(self, rhs: &VipsImage) -> Self::Output {
        self.subtract(subtract::OpArgs{right: &rhs}).unwrap_or_default()
    }
}
impl SubAssign<&VipsImage> for &mut VipsImage {
    fn sub_assign(&mut self, rhs: &VipsImage){
        **self = self.sub(rhs);
    }
}
impl Mul<&VipsImage> for &VipsImage {
    type Output = VipsImage;
    fn mul(self, rhs: &VipsImage) -> Self::Output {
        self.multiply(multiply::OpArgs{right: &rhs}).unwrap_or_default()
    }
}
impl MulAssign<&VipsImage> for &mut VipsImage {
    fn mul_assign(&mut self, rhs: &VipsImage){
        **self = self.mul(rhs);
    }
}
impl Div<&VipsImage> for &VipsImage {
    type Output = VipsImage;
    fn div(self, rhs: &VipsImage) -> Self::Output {
        self.divide(divide::OpArgs{right: &rhs}).unwrap_or_default()
    }
}
impl DivAssign<&VipsImage> for &mut VipsImage {
    fn div_assign(&mut self, rhs: &VipsImage){
        **self = self.div(rhs);
    }
}

// TODO: resolve naming conflict
// impl Add<&[f64]> for &VipsImage {
//     type Output = VipsImage;
//     fn add(self, rhs: &[f64]) -> Self::Output {
//         self.linear(linear::OpArgs{a: 1, b: &rhs}).unwrap_or_default()
//     }
// }
impl AddAssign<&[f64]> for &mut VipsImage {
    fn add_assign(&mut self, rhs: &[f64]) {
        let one: Vec<f64> = rhs.iter().map(|_| 1f64).collect();
        **self = self.linear(linear::OpArgs{a: &one, b: rhs}).unwrap_or_default();
    }
}
impl Sub<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn sub(self, rhs: &[f64]) -> Self::Output {
        let one: Vec<f64> = rhs.iter().map(|_| 1f64).collect();
        let inv: Vec<f64> = rhs.iter().map(|x| -x).collect();
        self.linear(linear::OpArgs{a: &one, b: &inv}).unwrap_or_default()
    }
}
impl SubAssign<&[f64]> for &mut VipsImage {
    fn sub_assign(&mut self, rhs: &[f64]){
        **self = self.sub(rhs);
    }
}
impl Mul<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn mul(self, rhs: &[f64]) -> Self::Output {
        let zero: Vec<f64> = rhs.iter().map(|_| 0f64).collect();
        self.linear(linear::OpArgs{a: rhs, b: &zero}).unwrap_or_default()
    }
}
impl MulAssign<&[f64]> for &mut VipsImage {
    fn mul_assign(&mut self, rhs: &[f64]){
        **self = self.mul(rhs);
    }
}
impl Div<&[f64]> for &VipsImage {
    type Output = VipsImage;
    fn div(self, rhs: &[f64]) -> Self::Output {
        let zero: Vec<f64> = rhs.iter().map(|_| 0f64).collect();
        let inv: Vec<f64> = rhs.iter().map(|x| 1f64/x).collect();
        self.linear(linear::OpArgs{a: &inv, b: &zero}).unwrap_or_default()
    }
}
impl DivAssign<&[f64]> for &mut VipsImage {
    fn div_assign(&mut self, rhs: &[f64]){
        **self = self.div(rhs);
    }
}

// TODO: resolve naming conflict
// impl Add<f64> for &VipsImage {
//     type Output = VipsImage;
//     fn add(self, rhs: f64) -> Self::Output {
//         self.linear(linear::OpArgs{a: 1, b: &rhs}).unwrap_or_default()
//     }
// }
impl AddAssign<f64> for &mut VipsImage {
    fn add_assign(&mut self, rhs: f64) {
        **self = self.linear1(linear1::OpArgs{a: 1f64, b: rhs}).unwrap_or_default();
    }
}
impl Sub<f64> for &VipsImage {
    type Output = VipsImage;
    fn sub(self, rhs: f64) -> Self::Output {
        self.linear1(linear1::OpArgs{a: 1f64, b: -rhs}).unwrap_or_default()
    }
}
impl SubAssign<f64> for &mut VipsImage {
    fn sub_assign(&mut self, rhs: f64){
        **self = self.sub(rhs);
    }
}
impl Mul<f64> for &VipsImage {
    type Output = VipsImage;
    fn mul(self, rhs: f64) -> Self::Output {
        self.linear1(linear1::OpArgs{a: rhs, b: 0f64}).unwrap_or_default()
    }
}
impl MulAssign<f64> for &mut VipsImage {
    fn mul_assign(&mut self, rhs: f64){
        **self = self.mul(rhs);
    }
}
impl Div<f64> for &VipsImage {
    type Output = VipsImage;
    fn div(self, rhs: f64) -> Self::Output {
        self.linear1(linear1::OpArgs{a: 1f64/rhs, b: 0f64}).unwrap_or_default()
    }
}
impl DivAssign<f64> for &mut VipsImage {
    fn div_assign(&mut self, rhs: f64){
        **self = self.div(rhs);
    }
}

/// `result = input * a + b`
define_operator!(linear, struct Args<'a> {
    pub a: &'a[f64],
    pub b: &'a[f64]
});

// linear1 needs to be implemented manually since it's not technically an
// operation
mod linear1 {
    pub struct OpArgs {
        pub a: f64,
        pub b: f64
    }
    impl crate::VipsImage {
        /// `result = input * a + b`
        pub fn linear1(&self, args: OpArgs) -> Result<crate::VipsImage, crate::VipsError> {
            let a_vec = [1..self.nbands()].map(|_| args.a);
            let b_vec = [1..self.nbands()].map(|_| args.b);

            self.linear(crate::arithmetic::linear::OpArgs{a: &a_vec, b: &b_vec})
        }
    }
}
// int vips_remainder()
// int vips_remainder_const()
// int vips_remainder_const1()
// int vips_invert()
// int vips_abs()
// int vips_sign()
// int vips_round()
// int vips_floor()
// int vips_ceil()
// int vips_rint()
// int vips_math()
// int vips_sin()
// int vips_cos()
// int vips_tan()
// int vips_asin()
// int vips_acos()
// int vips_atan()
// int vips_exp()
// int vips_exp10()
// int vips_log()
// int vips_log10()
// int vips_sinh()
// int vips_cosh()
// int vips_tanh()
// int vips_asinh()
// int vips_acosh()
// int vips_atanh()
// int vips_complex()
// int vips_polar()
// int vips_rect()
// int vips_conj()
// int vips_complex2()
// int vips_cross_phase()
// int vips_complexget()
// int vips_real()
// int vips_imag()
// int vips_complexform()
// int vips_relational()
// int vips_equal()
// int vips_notequal()
// int vips_less()
// int vips_lesseq()
// int vips_more()
// int vips_moreeq()
// int vips_relational_const()
// int vips_equal_const()
// int vips_notequal_const()
// int vips_less_const()
// int vips_lesseq_const()
// int vips_more_const()
// int vips_moreeq_const()
// int vips_relational_const1()
// int vips_equal_const1()
// int vips_notequal_const1()
// int vips_less_const1()
// int vips_lesseq_const1()
// int vips_more_const1()
// int vips_moreeq_const1()
// int vips_boolean()
// int vips_andimage()
// int vips_orimage()
// int vips_eorimage()
// int vips_lshift()
// int vips_rshift()
// int vips_boolean_const()
// int vips_andimage_const()
// int vips_orimage_const()
// int vips_eorimage_const()
// int vips_lshift_const()
// int vips_rshift_const()
// int vips_boolean_const1()
// int vips_andimage_const1()
// int vips_orimage_const1()
// int vips_eorimage_const1()
// int vips_lshift_const1()
// int vips_rshift_const1()
// int vips_math2()
// int vips_pow()
// int vips_wop()
// int vips_atan2()
// int vips_math2_const()
// int vips_pow_const()
// int vips_wop_const()
// int vips_atan2_const()
// int vips_math2_const1()
// int vips_pow_const1()
// int vips_wop_const1()
// int vips_atan2_const1()
// int vips_avg()
// int vips_deviate()
// int vips_min()
// int vips_max()
// int vips_stats()
// int vips_measure()
// int vips_find_trim()
// int vips_getpoint()
// int vips_hist_find()
// int vips_hist_find_ndim()
// int vips_hist_find_indexed()
// int vips_hough_line()
// int vips_hough_circle()
// int vips_project()
// int vips_profile()

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ensure_vips_init_or_exit;
    use std::path::PathBuf;

    #[test]
    fn add() {
        ensure_vips_init_or_exit();

        let img = VipsImage::new_from_file(PathBuf::from("./data/test.jpg"))
            .expect("Image could not be created from file");
        assert_ne!(img.ptr, std::ptr::null_mut());

        let _added = img.add(add::OpArgs{right: &img})
            .expect("Could not add image");

        // TODO: check if pixel values make sense
    }

    #[test]
    fn subtract() {
        ensure_vips_init_or_exit();

        let img = VipsImage::new_from_file(PathBuf::from("./data/test.jpg"))
            .expect("Image could not be created from file");
        assert_ne!(img.ptr, std::ptr::null_mut());

        let _subtracted = img.subtract(subtract::OpArgs{right: &img})
            .expect("Could not subtract image");

        // TODO: check if pixel values make sense
    }

    #[test]
    fn multiply() {
        ensure_vips_init_or_exit();

        let img = VipsImage::new_from_file(PathBuf::from("./data/test.jpg"))
            .expect("Image could not be created from file");
        assert_ne!(img.ptr, std::ptr::null_mut());

        let _multiplied = img.multiply(multiply::OpArgs{right: &img})
            .expect("Could not multiply image");

        // TODO: check if pixel values make sense
    }

    #[test]
    fn divide() {
        ensure_vips_init_or_exit();

        let img = VipsImage::new_from_file(PathBuf::from("./data/test.jpg"))
            .expect("Image could not be created from file");
        assert_ne!(img.ptr, std::ptr::null_mut());

        let _divided = img.divide(divide::OpArgs{right: &img})
            .expect("Could not divide image");

        // TODO: check if pixel values make sense
    }

    #[test]
    fn linear() {
        ensure_vips_init_or_exit();

        let img = VipsImage::new_from_file(PathBuf::from("./data/test.jpg"))
            .expect("Image could not be created from file");
        assert_ne!(img.ptr, std::ptr::null_mut());

        let _transformed = &(&img - 30.) / vec![0.9, 1.1, 1.].as_slice();

        // TODO: check if pixel values make sense

        // _transformed.write_to_file(PathBuf::from("./data/test_linear.jpg"))
        //     .expect("Could not write result to file");
    }
}