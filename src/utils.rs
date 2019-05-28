//! General utility functions.

/// Allocate an owned buffer using the global allocator.
/// Only enabled with `std`.
#[cfg(feature = "std")]
pub fn unsafe_alloc_buffer(len: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(len);
    unsafe {
        ret.set_len(len);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let ret = unsafe_alloc_buffer(42);
        assert_eq!(ret.len(), 42);
    }
}
