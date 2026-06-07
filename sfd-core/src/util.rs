use std::path::Path;

use schematic::{ValidateError, ValidateResult};

pub(crate) fn validate_absolute<T, C>(path: &Path, _: &T, _: &C, _: bool) -> ValidateResult {
    if !path.is_absolute() {
        Err(ValidateError::new("path must be absolute"))
    } else {
        Ok(())
    }
}

pub(crate) fn to_rel(path: &Path, root: impl AsRef<Path>) -> &Path {
    path.strip_prefix(root.as_ref())
        .expect("path is outside of project root")
}
