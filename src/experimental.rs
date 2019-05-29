//! Experimental methods.

use super::*;

mod native {
    extern "C" {
        pub fn experimental_isAccountEmpty(addressOffset: *const u32) -> u32;
    }
}

pub fn is_account_empty(address: &Address) -> bool {
    let ret = unsafe { native::experimental_isAccountEmpty(address.bytes.as_ptr() as *const u32) };
    if ret != 0 && ret != 1 {
        panic!();
    }

    ret == 1
}
