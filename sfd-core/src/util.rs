use std::path::PathBuf;

use schematic::{ValidateError, ValidateResult};

pub(crate) fn validate_absolute<T, C>(path: &PathBuf, _: &T, _: &C, _: bool) -> ValidateResult {
    if !path.is_absolute() {
        Err(ValidateError::new("path must be absolute"))
    } else {
        Ok(())
    }
}
