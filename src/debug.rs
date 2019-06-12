//! The native debug interface exposed to the ewasm contract. These functions are for testing
//! purposes only. On a live VM, any bytecode trying to import these symbols will be rejected.

/// The native interface for debugging functions.
#[cfg(debug_assertions)]
pub mod native {
    extern "C" {
        pub fn debug_print32(value: u32);
        pub fn debug_print64(value: u64);
        pub fn debug_printMem(offset: *const u32, len: u32);
        pub fn debug_printMemHex(offset: *const u32, len: u32);
        pub fn debug_printStorage(path_offset: *const u32);
        pub fn debug_printStorageHex(path_offset: *const u32);
    }
}

#[macro_export]
/// Prints an unsigned 32-bit int.
macro_rules! print32 {
    ($value:expr) => {
        #[cfg(debug_assertions)]
        {
            unsafe { $crate::debug::native::debug_print32($value) }
        }
    };
}

#[macro_export]
/// Prints an unsigned 64-bit int.
macro_rules! print64 {
    ($value:expr) => {
        #[cfg(debug_assertions)]
        {
            unsafe { $crate::debug::native::debug_print64($value) }
        }
    };
}

#[macro_export]
/// Prints the contents of a slice.
macro_rules! print_mem {
    ($slice:expr) => {
        #[cfg(debug_assertions)]
        {
            unsafe {
                $crate::debug::native::debug_printMem(
                    $slice.as_ptr() as *const u32,
                    $slice.len() as u32,
                )
            }
        }
    };
}

#[macro_export]
/// Prints the contents of a slice in hexadecimal format.
macro_rules! print_mem_hex {
    ($slice:expr) => {
        #[cfg(debug_assertions)]
        {
            unsafe {
                $crate::debug::native::debug_printMemHex(
                    $slice.as_ptr() as *const u32,
                    $slice.len() as u32,
                )
            }
        }
    };
}

#[macro_export]
/// Prints the value of a storage key.
macro_rules! print_storage {
    ($key:expr) => {
        #[cfg(debug_assertions)]
        {
            unsafe { $crate::debug::native::debug_printStorage($key.bytes.as_ptr() as *const u32) }
        }
    };
}

#[macro_export]
/// Prints the value of a storage key in hexadecimal format.
macro_rules! print_storage_hex {
    ($key:expr) => {
        #[cfg(debug_assertions)]
        {
            unsafe {
                $crate::debug::native::debug_printStorageHex($key.bytes.as_ptr() as *const u32)
            }
        }
    };
}
