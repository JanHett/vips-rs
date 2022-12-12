//! # Safe Rust bindings for [`libvips`](https://github.com/libvips/libvips)
//! 
//! ## Example
//! 
//! ```rs
//! // automatically initialize and shut down Vips
//! let VipsHandle = VipsHandle::new();
//! 
//! let img = VipsImage::from_file(PathBuf::from("./data/test.jpg"))
//!     .expect("Could not read image file");
//! ```

pub mod vips;
pub mod image;
pub mod error;
pub mod operator;

// =============================================================================
// === EXPORTED SYMBOLS ========================================================
// =============================================================================

pub use crate::vips::{
    vips_init_or_exit,
    ensure_vips_init_or_exit,
    vips_init,
    vips_shutdown,
    VipsHandle
};

pub use crate::error::VipsError;
pub use crate::image::VipsImage;

pub use crate::operator::*;
