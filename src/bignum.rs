//! The bignum system library.

use super::*;

pub mod native {
    extern "C" {
        pub fn bignum_mul256(a: *const u32, b: *const u32, ret: *const u32);
        pub fn bignum_umulmod256(a: *const u32, b: *const u32, modulo: *const u32, ret: *const u32);
    }
}

pub fn mul256(a: &Uint256, b: &Uint256) -> Uint256 {
    let mut ret = Uint256::default();

    unsafe {
        native::bignum_mul256(
            a.bytes.as_ptr() as *const u32,
            b.bytes.as_ptr() as *const u32,
            ret.bytes.as_mut_ptr() as *const u32,
        )
    }

    ret
}

pub fn umulmod256(a: &Uint256, b: &Uint256, modulo: &Uint256) -> Uint256 {
    let mut ret = Uint256::default();

    unsafe {
        native::bignum_umulmod256(
            a.bytes.as_ptr() as *const u32,
            b.bytes.as_ptr() as *const u32,
            modulo.bytes.as_ptr() as *const u32,
            ret.bytes.as_mut_ptr() as *const u32,
        )
    }

    ret
}
