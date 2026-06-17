use std::path::Path;

use schematic::{ValidateError, ValidateResult};

/// Asserts that a path is absolute.
pub(crate) fn validate_absolute<T, C>(path: &Path, _: &T, _: &C, _: bool) -> ValidateResult {
    if !path.is_absolute() {
        Err(ValidateError::new("path must be absolute"))
    } else {
        Ok(())
    }
}

/// Turns a path into a relative one.
pub(crate) fn to_rel(path: &Path, root: impl AsRef<Path>) -> &Path {
    path.strip_prefix(root.as_ref())
        .expect("path is outside of project root")
}
