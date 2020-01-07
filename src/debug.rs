//! The native debug interface exposed to the ewasm contract. These functions are for testing
//! purposes only. On a live VM, any bytecode trying to import these symbols will be rejected.

use crate::types::StorageKey;

/// The native interface for debugging functions.
mod native {
    extern "C" {
        pub fn debug_print32(value: u32);
        pub fn debug_print64(value: u64);
        pub fn debug_printMem(offset: *const u32, len: u32);
        pub fn debug_printMemHex(offset: *const u32, len: u32);
        pub fn debug_printStorage(pathOffset: *const u32);
        pub fn debug_printStorageHex(pathOffset: *const u32);
    }
}

/// Prints a string.
pub fn log(msg: &str) {
    unsafe { native::debug_printMem(msg.as_ptr() as *const u32, msg.len() as u32) }
}

/// Prints an unsigned 32-bit int.
fn print32(value: u32) {
    unsafe { native::debug_print32(value) }
}

/// Prints an unsigned 64-bit int.
fn print64(value: u64) {
    unsafe { native::debug_print64(value) }
}

/// Prints the contents of a slice.
fn print_mem(slice: &[u8]) {
    unsafe { native::debug_printMem(slice.as_ptr() as *const u32, slice.len() as u32) }
}

/// Prints the contents of a slice in hexadecimal format.
fn print_mem_hex(slice: &[u8]) {
    unsafe { native::debug_printMemHex(slice.as_ptr() as *const u32, slice.len() as u32) }
}

/// Prints the value of a storage key.
fn print_storage(key: &StorageKey) {
    unsafe { native::debug_printStorage(key.bytes.as_ptr() as *const u32) }
}

/// Prints the value of a storage key in hexadecimal format.
fn print_storage_hex(key: &StorageKey) {
    unsafe { native::debug_printStorageHex(key.bytes.as_ptr() as *const u32) }
}

#[macro_export]
macro_rules! print32 {
    ($e:expr) => {
        #[cfg(debug_assertions)]
        $crate::debug::native::print32($e)
    };
}

#[macro_export]
macro_rules! print64 {
    ($e:expr) => {
        #[cfg(debug_assertions)]
        $crate::debug::native::print64($e)
    };
}

#[macro_export]
macro_rules! print_mem {
    ($e:expr) => {
        #[cfg(debug_assertions)]
        $crate::debug::native::print_mem($e)
    };
}

#[macro_export]
macro_rules! print_mem_hex {
    ($e:expr) => {
        #[cfg(debug_assertions)]
        $crate::debug::native::print_mem_hex($e)
    };
}
#[macro_export]
macro_rules! print_storage {
    ($e:expr) => {
        #[cfg(debug_assertions)]
        $crate::debug::native::print_storage($e)
    };
}
#[macro_export]
macro_rules! print_storage_hex {
    ($e:expr) => {
        #[cfg(debug_assertions)]
        $crate::debug::native::print_storage_hex($e)
    };
}
