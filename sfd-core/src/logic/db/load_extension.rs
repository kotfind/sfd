use std::sync::OnceLock;

/// Proof that the sqlite-vec extension has been loaded.
///
/// Can only be obtained via [`load`].
#[derive(Debug, Clone, Copy)]
pub struct VecExtLoadedProof(());

static PROOF: OnceLock<VecExtLoadedProof> = OnceLock::new();

/// Loads the sqlite-vec extension, returning [`VecExtLoadedProof`] as proof.
pub fn load() -> VecExtLoadedProof {
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
        VecExtLoadedProof(())
    })
}
