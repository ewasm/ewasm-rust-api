//! The native debug interface exposed to the ewasm contract. These functions are for testing
//! purposes only. On a live VM, any bytecode trying to import these symbols will be rejected.

use crate::types::StorageKey;

/// The native interface for debugging functions.
mod native {
    #[link(wasm_import_module = "debug")]
    extern "C" {
        pub fn print32(value: u32);
        pub fn print64(value: u64);
        pub fn printMem(offset: *const u32, len: u32);
        pub fn printMemHex(offset: *const u32, len: u32);
        pub fn printStorage(pathOffset: *const u32);
        pub fn printStorageHex(pathOffset: *const u32);
    }
}

/// Prints a string.
pub fn log(msg: &str) {
    unsafe { native::printMem(msg.as_ptr() as *const u32, msg.len() as u32) }
}

/// Prints an unsigned 32-bit int.
pub fn print32(value: u32) {
    unsafe { native::print32(value) }
}

/// Prints an unsigned 64-bit int.
pub fn print64(value: u64) {
    unsafe { native::print64(value) }
}

/// Prints the contents of a slice.
pub fn print_mem(slice: &[u8]) {
    unsafe { native::printMem(slice.as_ptr() as *const u32, slice.len() as u32) }
}

/// Prints the contents of a slice in hexadecimal format.
pub fn print_mem_hex(slice: &[u8]) {
    unsafe { native::printMemHex(slice.as_ptr() as *const u32, slice.len() as u32) }
}

/// Prints the value of a storage key.
pub fn print_storage(key: &StorageKey) {
    unsafe { native::printStorage(key.bytes.as_ptr() as *const u32) }
}

/// Prints the value of a storage key in hexadecimal format.
pub fn print_storage_hex(key: &StorageKey) {
    unsafe { native::printStorageHex(key.bytes.as_ptr() as *const u32) }
}
