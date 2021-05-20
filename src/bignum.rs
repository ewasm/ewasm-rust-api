//! The bignum system library.
use crate::types::Uint256;

/// The low-level interface to the system library. Use the wrapper functions unless you know what
/// you're doing.
pub mod native {
    #[link(wasm_import_module = "bignum")]
    extern "C" {
        pub fn mul256(a: *const u32, b: *const u32, ret: *const u32);
        pub fn umulmod256(a: *const u32, b: *const u32, modulo: *const u32, ret: *const u32);
    }
}

/// Unsigned 256-bit multiplication.
pub fn mul256(a: &Uint256, b: &Uint256) -> Uint256 {
    let mut ret = Uint256::default();

    unsafe {
        native::mul256(
            a.bytes.as_ptr() as *const u32,
            b.bytes.as_ptr() as *const u32,
            ret.bytes.as_mut_ptr() as *const u32,
        )
    }

    ret
}

/// Unsigned 256-bit multiplication modulo n.
pub fn umulmod256(a: &Uint256, b: &Uint256, modulo: &Uint256) -> Uint256 {
    let mut ret = Uint256::default();

    unsafe {
        native::umulmod256(
            a.bytes.as_ptr() as *const u32,
            b.bytes.as_ptr() as *const u32,
            modulo.bytes.as_ptr() as *const u32,
            ret.bytes.as_mut_ptr() as *const u32,
        )
    }

    ret
}
