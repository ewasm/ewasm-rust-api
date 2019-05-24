use super::*;

// FIXME: this is a duplicate from types.rs
#[cfg(feature = "std")]
fn unsafe_alloc_buffer(len: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(len);
    unsafe {
        ret.set_len(len);
    }
    ret
}

mod native {
    extern "C" {
        pub fn eth2_loadPreState(offset: *const u32);
        pub fn eth2_blockDataSize() -> u32;
        pub fn eth2_blockDataCopy(outputOfset: *const u32, offset: u32, length: u32);
        pub fn eth2_savePostState(offset: *const u32);
        pub fn eth2_pushNewDeposit(offset: *const u32);
    }
}

pub fn load_pre_state() -> Bytes32 {
    let mut ret = Bytes32::default();

    unsafe { native::eth2_loadPreState(ret.bytes.as_mut_ptr() as *const u32) }

    ret
}

pub fn block_data_size() -> usize {
    unsafe { native::eth2_blockDataSize() as usize }
}

/// Executes callDataCopy, but does not check for overflow.
pub fn unsafe_block_data_copy(from: usize, length: usize, ret: &mut [u8]) {
    unsafe {
        native::eth2_blockDataCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }
}

#[cfg(feature = "std")]
/// Returns a vector containing all data passed with the currently executing call.
pub fn acquire_block_data() -> Vec<u8> {
    let length = block_data_size();

    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);
    unsafe_block_data_copy(0, length, &mut ret);
    ret
}

/// Returns the segment of call data beginning at `from`, and continuing for `length` bytes.
pub fn block_data_copy(from: usize, length: usize, ret: &mut [u8]) -> Result<(), Error> {
    let size = block_data_size();

    if (size < from) || ((size - from) < length) || (ret.len() < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        unsafe_block_data_copy(from, length, ret);
        Ok(())
    }
}

pub fn save_post_state(state: Bytes32) {
    unsafe { native::eth2_savePostState(state.bytes.as_ptr() as *const u32) }
}
