use std::path::Path;

use schematic::{ValidateError, ValidateResult};

pub(crate) fn validate_absolute<T, C>(path: &Path, _: &T, _: &C, _: bool) -> ValidateResult {
    if !path.is_absolute() {
        Err(ValidateError::new("path must be absolute"))
    } else {
        Ok(())
    }
}
