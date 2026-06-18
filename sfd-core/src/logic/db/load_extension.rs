use std::sync::OnceLock;

/// Proof that the sqlite-vec extension has been loaded.
///
/// Can only be obtained via [`load`].
#[derive(Debug, Clone, Copy)]
pub struct VecExtLoadProof(());

static PROOF: OnceLock<VecExtLoadProof> = OnceLock::new();

/// Loads the sqlite-vec extension, returning [`VecExtLoadProof`] as proof.
pub fn load() -> VecExtLoadProof {
    *PROOF.get_or_init(|| {
        unsafe {
            libsqlite3_sys::sqlite3_auto_extension(Some(std::mem::transmute::<
                *const (),
                unsafe extern "C" fn(
                    *mut libsqlite3_sys::sqlite3,
                    *mut *mut i8,
                    *const libsqlite3_sys::sqlite3_api_routines,
                ) -> i32,
            >(
                sqlite_vec::sqlite3_vec_init as *const (),
            )));
        }
        VecExtLoadProof(())
    })
}
