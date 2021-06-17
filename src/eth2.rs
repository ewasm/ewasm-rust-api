//! This is the work in progress interface for Eth 2 Phase 2.
//!
//! Please check Ewasm [Scout](https://github.com/ewasm/scout) for more details.
//!
//! # Examples
//! ```
//! use ewasm_api::prelude::*;
//!
//! fn process_block(pre_state_root: &Bytes32, block_data: &[u8]) -> (Bytes32, Vec<u8>) {
//!     unimplemented!()
//! }
//!
//! shard_script!(process_block);
//! ```

use super::*;

mod native {
    #[link(wasm_import_module = "eth2")]
    extern "C" {
        pub fn loadPreStateRoot(offset: *const u32);
        pub fn blockDataSize() -> u32;
        pub fn blockDataCopy(outputOfset: *const u32, offset: u32, length: u32);
        pub fn savePostStateRoot(offset: *const u32);
        pub fn pushNewDeposit(offset: *const u32, length: u32);
    }
}

/// Load current state root.
pub fn load_pre_state_root() -> Bytes32 {
    let mut ret = Bytes32::default();

    unsafe { native::loadPreStateRoot(ret.bytes.as_mut_ptr() as *const u32) }

    ret
}

/// Returns the length of the "block data" supplied with the current block.
pub fn block_data_size() -> usize {
    unsafe { native::blockDataSize() as usize }
}

/// Copies a slices from the "block data", but does not check for overflow.
pub fn unsafe_block_data_copy(from: usize, length: usize, ret: &mut [u8]) {
    unsafe {
        native::blockDataCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }
}

#[cfg(feature = "std")]
/// Returns a vector containing the entire "block data".
pub fn acquire_block_data() -> Vec<u8> {
    let length = block_data_size();

    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);
    unsafe_block_data_copy(0, length, &mut ret);
    ret
}

/// Returns the segment of "block data" beginning at `from`, and continuing for `length` bytes.
pub fn block_data_copy(from: usize, length: usize, ret: &mut [u8]) -> Result<(), Error> {
    let size = block_data_size();

    if (size < from) || ((size - from) < length) || (ret.len() < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        unsafe_block_data_copy(from, length, ret);
        Ok(())
    }
}

/// Push new deposit receipt.
pub fn push_new_deposit(deposit: &[u8]) {
    unsafe { native::pushNewDeposit(deposit.as_ptr() as *const u32, deposit.len() as u32) }
}

/// Save new state root.
pub fn save_post_state_root(state: &Bytes32) {
    unsafe { native::savePostStateRoot(state.bytes.as_ptr() as *const u32) }
}

/// Create shard script entry point. Expects a function to process blocks with the signature:
/// ```ignore
/// fn process_block(pre_state_root: &Bytes32, block_data: &[u8]) -> (Bytes32, Vec<u8>) {}
/// ```
#[macro_export]
macro_rules! shard_script {
    ($process_block:ident) => {
        #[cfg(target_arch = "wasm32")]
        #[no_mangle]
        pub extern "C" fn main() {
            let pre_state_root = eth2::load_pre_state_root();
            // TODO: avoid using Vec here
            let block_data = eth2::acquire_block_data();
            let (post_state_root, deposits) = $process_block(&pre_state_root, &block_data);
            eth2::push_new_deposit(&deposits);
            eth2::save_post_state_root(&post_state_root)
        }
    };
}
