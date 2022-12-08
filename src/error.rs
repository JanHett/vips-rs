use std::error::Error;

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

impl Error for VipsError {
    fn description(&self) -> &str { self.description.as_str() }
}

// === Implementation ===========================================================

impl VipsError {
    pub fn new<T: Into<String>>(description: T) -> VipsError {
        VipsError { description: description.into() }
    }
}
