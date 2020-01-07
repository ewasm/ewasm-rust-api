//! ewasm_api is a library used to interface with Ethereum's EEI in [Ewasm](https://github.com/ewasm/design), a set of enhancements to
//! the Ethereum smart contract platform.
//! ewasm_api exposes both a set of unsafe "native" functions representing the actual EEI
//! functions, and a set of safe wrappers around them. It is recommended not to use the native
//! functions as they do not perform bounds-checking.
//!
//! To use ewasm_api, simply include it as a dependency in your project.
//! ewasm_api can be built with various feature sets:
//! - `default`: Builds with `wee_alloc` as the global allocator and with the Rust standard
//! library.
//! - `qimalloc`: Builds with [qimalloc](https://github.com/wasmx/qimalloc) as the global
//! allocator.
//! - `experimental`: Exposes the experimental bignum system library API.
//!
//! # Examples
//! ```
//! use ewasm_api::prelude::*;
//!
//! fn entry() {
//!     let a: Hash = block_hash(1);
//!     finish_data(&a.bytes);
//! }
//!
//! ewasm_entry_point!(entry);
//! ```
//!
//! Using lower-level primitives:
//! ```ignore
//! use ewasm_api::{types::Hash, block_hash, finish_data};
//!
//! #[cfg(target_arch = "wasm32")]
//! #[no_mangle]
//! pub extern "C" fn main() {
//!     let a: types::Hash = block_hash(1);
//!     finish_data(&a.bytes);
//! }
//! ```
//!

#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    } else if #[cfg(feature = "qimalloc")] {
        extern crate qimalloc;
        #[global_allocator]
        static ALLOC: qimalloc::QIMalloc = qimalloc::QIMalloc::INIT;
    }
}

mod native;
mod utils;

pub mod types;

pub mod debug;

#[cfg(feature = "experimental")]
pub mod bignum;

#[cfg(feature = "eth2")]
pub mod eth2;

#[cfg(not(feature = "std"))]
pub mod convert;

#[cfg(feature = "std")]
use std::vec::Vec;

use types::*;
#[cfg(feature = "std")]
use utils::*;

/// Re-export of all the basic features.
pub mod prelude {
    pub use crate::*;

    pub use crate::types::*;

    #[cfg(not(feature = "std"))]
    pub use crate::convert::*;

    pub use crate::debug;

    #[cfg(feature = "experimental")]
    pub use crate::bignum;

    #[cfg(feature = "eth2")]
    pub use crate::eth2;
}

/// Declare entry point for a contract. Expects a Rust function name to be executed.
/// This will only compile in when using the wasm32 target.
#[macro_export]
macro_rules! ewasm_entry_point {
    ($name:ident) => {
        #[cfg(target_arch = "wasm32")]
        #[no_mangle]
        pub extern "C" fn main() {
            $name()
        }
    };
}

/// Enum representing an error code for EEI calls. Currently used by `codeCopy`, `callDataCopy`,
/// `externalCodeCopy`, and `returnDataCopy`.
pub enum Error {
    OutOfBoundsCopy,
}

/// Enum describing the result of a call. Used by `call`, `callCode`, `callDelegate`, and
/// `callStatic`.
pub enum CallResult {
    Successful,
    Failure,
    Revert,
    Unknown,
}

/// Enum describing the result of `create`. On success, the data contained is the address of the
/// newly created contract.
pub enum CreateResult {
    Successful(Address),
    Failure,
    Revert,
    Unknown,
}

/// Subtracts the given amount from the VM's gas counter. This is usually injected by the metering
/// contract at deployment time, and hence is unneeded in most cases.
pub fn consume_gas(amount: u64) {
    unsafe {
        native::ethereum_useGas(amount);
    }
}

/// Returns the gas left in the current call.
pub fn gas_left() -> u64 {
    unsafe { native::ethereum_getGasLeft() }
}

/// Returns the executing address.
pub fn current_address() -> Address {
    let mut ret = Address::default();

    unsafe {
        native::ethereum_getAddress(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Returns the balance of the address given.
pub fn external_balance(address: &Address) -> EtherValue {
    let mut ret = EtherValue::default();

    unsafe {
        native::ethereum_getExternalBalance(
            address.bytes.as_ptr() as *const u32,
            ret.bytes.as_mut_ptr() as *const u32,
        );
    }

    ret
}

/// Returns the beneficiary address for the block this transaction is in (current block)
pub fn block_coinbase() -> Address {
    let mut ret = Address::default();

    unsafe {
        native::ethereum_getBlockCoinbase(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Returns the difficulty of the most recent block.
pub fn block_difficulty() -> Difficulty {
    let mut ret = Difficulty::default();

    unsafe {
        native::ethereum_getBlockDifficulty(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Returns the gas limit of the most recent block.
pub fn block_gas_limit() -> u64 {
    unsafe { native::ethereum_getBlockGasLimit() }
}

/// Returns the hash of the `number`th most recent block.
pub fn block_hash(number: u64) -> Hash {
    let mut ret = Hash::default();

    unsafe {
        native::ethereum_getBlockHash(number, ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Returns the number of the most recent block.
pub fn block_number() -> u64 {
    unsafe { native::ethereum_getBlockNumber() }
}

/// Returns the timestamp of the most recent block.
pub fn block_timestamp() -> u64 {
    unsafe { native::ethereum_getBlockTimestamp() }
}

/// Returns the gas price of the currently executing call.
pub fn tx_gas_price() -> EtherValue {
    let mut ret = EtherValue::default();

    unsafe {
        native::ethereum_getTxGasPrice(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Returns the address of the original transaction sender.
pub fn tx_origin() -> Address {
    let mut ret = Address::default();

    unsafe {
        native::ethereum_getTxOrigin(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Appends log data to the transaction receipt, with a variable number of topics.
fn log(
    data: &[u8],
    topic_count: usize,
    topic1: *const u8,
    topic2: *const u8,
    topic3: *const u8,
    topic4: *const u8,
) {
    unsafe {
        native::ethereum_log(
            data.as_ptr() as *const u32,
            data.len() as u32,
            topic_count as u32,
            topic1 as *const u32,
            topic2 as *const u32,
            topic3 as *const u32,
            topic4 as *const u32,
        );
    }
}

/// Appends log data without a topic.
pub fn log0(data: &[u8]) {
    log(
        data,
        0,
        0 as *const u8,
        0 as *const u8,
        0 as *const u8,
        0 as *const u8,
    )
}

/// Appends log data with one topic.
pub fn log1(data: &[u8], topic1: &LogTopic) {
    log(
        data,
        1,
        topic1.bytes.as_ptr() as *const u8,
        0 as *const u8,
        0 as *const u8,
        0 as *const u8,
    )
}

/// Appends log data with two topics.
pub fn log2(data: &[u8], topic1: &LogTopic, topic2: &LogTopic) {
    log(
        data,
        2,
        topic1.bytes.as_ptr() as *const u8,
        topic2.bytes.as_ptr() as *const u8,
        0 as *const u8,
        0 as *const u8,
    )
}

/// Appends log data with three topics.
pub fn log3(data: &[u8], topic1: &LogTopic, topic2: &LogTopic, topic3: &LogTopic) {
    log(
        data,
        3,
        topic1.bytes.as_ptr() as *const u8,
        topic2.bytes.as_ptr() as *const u8,
        topic3.bytes.as_ptr() as *const u8,
        0 as *const u8,
    )
}

/// Appends log data with four topics.
pub fn log4(
    data: &[u8],
    topic1: &LogTopic,
    topic2: &LogTopic,
    topic3: &LogTopic,
    topic4: &LogTopic,
) {
    log(
        data,
        4,
        topic1.bytes.as_ptr() as *const u8,
        topic2.bytes.as_ptr() as *const u8,
        topic3.bytes.as_ptr() as *const u8,
        topic4.bytes.as_ptr() as *const u8,
    )
}

/// Executes a standard call to the specified address with the given gas limit, ether value, and
/// data.
pub fn call_mutable(
    gas_limit: u64,
    address: &Address,
    value: &EtherValue,
    data: &[u8],
) -> CallResult {
    let ret = unsafe {
        native::ethereum_call(
            gas_limit,
            address.bytes.as_ptr() as *const u32,
            value.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => CallResult::Unknown,
    }
}

/// Executes another account's code in the context of the caller.
pub fn call_code(gas_limit: u64, address: &Address, value: &EtherValue, data: &[u8]) -> CallResult {
    let ret = unsafe {
        native::ethereum_callCode(
            gas_limit,
            address.bytes.as_ptr() as *const u32,
            value.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => CallResult::Unknown,
    }
}

/// Executes a call similar to `call_code`, but retaining the currently executing call's sender
/// and value.
pub fn call_delegate(gas_limit: u64, address: &Address, data: &[u8]) -> CallResult {
    let ret = unsafe {
        native::ethereum_callDelegate(
            gas_limit,
            address.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => CallResult::Unknown,
    }
}

/// Executes a static call which cannot mutate the state.
pub fn call_static(gas_limit: u64, address: &Address, data: &[u8]) -> CallResult {
    let ret = unsafe {
        native::ethereum_callStatic(
            gas_limit,
            address.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => CallResult::Unknown,
    }
}

/// Creates a contract with the the given code, sending the specified ether value to its address.
pub fn create(value: &EtherValue, data: &[u8]) -> CreateResult {
    let mut address = Address::default();

    let ret = unsafe {
        native::ethereum_create(
            value.bytes.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
            address.bytes.as_mut_ptr() as *const u32,
        )
    };

    match ret {
        0 => CreateResult::Successful(address),
        1 => CreateResult::Failure,
        2 => CreateResult::Revert,
        _ => CreateResult::Unknown,
    }
}

/// Executes callDataCopy, but does not check for overflow.
pub fn unsafe_calldata_copy(from: usize, length: usize, ret: &mut [u8]) {
    unsafe {
        native::ethereum_callDataCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }
}

#[cfg(feature = "std")]
/// Returns a vector containing all data passed with the currently executing call.
pub fn calldata_acquire() -> Vec<u8> {
    let length = calldata_size();

    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);
    unsafe_calldata_copy(0, length, &mut ret);
    ret
}

/// Returns the segment of call data beginning at `from`, and continuing for `length` bytes.
pub fn calldata_copy(from: usize, length: usize, ret: &mut [u8]) -> Result<(), Error> {
    let size = calldata_size();

    if (size < from) || ((size - from) < length) || (ret.len() < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        unsafe_calldata_copy(from, length, ret);
        Ok(())
    }
}

/// Returns the length of the call data supplied with the currently executing call.
pub fn calldata_size() -> usize {
    unsafe { native::ethereum_getCallDataSize() as usize }
}

/// Returns the sender of the currently executing call.
pub fn caller() -> Address {
    let mut ret = Address::default();

    unsafe {
        native::ethereum_getCaller(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Returns the value sent with the currently executing call.
pub fn callvalue() -> EtherValue {
    let mut ret = EtherValue::default();

    unsafe {
        native::ethereum_getCallValue(ret.bytes.as_mut_ptr() as *const u32);
    }

    ret
}

/// Executes codeCopy, but does not check for overflow.
pub fn unsafe_code_copy(from: usize, length: usize, ret: &mut [u8]) {
    unsafe {
        native::ethereum_codeCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }
}

#[cfg(feature = "std")]
/// Returns the currently executing code.
pub fn code_acquire() -> Vec<u8> {
    let length = code_size();

    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);
    unsafe_code_copy(0, length, &mut ret);
    ret
}

/// Copies the segment of running code beginning at `from` and continuing for `length` bytes.
pub fn code_copy(from: usize, length: usize, ret: &mut [u8]) -> Result<(), Error> {
    let size = code_size();

    if (size < from) || ((size - from) < length) || (ret.len() < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        unsafe_code_copy(from, length, ret);
        Ok(())
    }
}

/// Returns the size of the currently executing code.
pub fn code_size() -> usize {
    unsafe { native::ethereum_getCodeSize() as usize }
}

/// Executes externalCodeCopy, but does not check for overflow.
pub fn unsafe_external_code_copy(address: &Address, from: usize, length: usize, ret: &mut [u8]) {
    unsafe {
        native::ethereum_externalCodeCopy(
            address.bytes.as_ptr() as *const u32,
            ret.as_mut_ptr() as *const u32,
            from as u32,
            length as u32,
        );
    }
}

#[cfg(feature = "std")]
/// Returns the code at the specified address.
pub fn external_code_acquire(address: &Address) -> Vec<u8> {
    let length = external_code_size(address);

    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);
    unsafe_external_code_copy(address, 0, length, &mut ret);
    ret
}

/// Returns the segment of code at `address` beginning at `from` and continuing for `length` bytes.
pub fn external_code_copy(
    address: &Address,
    from: usize,
    length: usize,
    ret: &mut [u8],
) -> Result<(), Error> {
    let size = external_code_size(address);

    if (size < from) || ((size - from) < length) || (ret.len() < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        unsafe_external_code_copy(address, from, length, ret);
        Ok(())
    }
}

/// Returns the size of the code at the specified address.
pub fn external_code_size(address: &Address) -> usize {
    unsafe { native::ethereum_getExternalCodeSize(address.bytes.as_ptr() as *const u32) as usize }
}

/// Executes returnDataCopy, but does not check for overflow.
pub fn unsafe_returndata_copy(from: usize, length: usize, ret: &mut [u8]) {
    unsafe {
        native::ethereum_returnDataCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }
}

#[cfg(feature = "std")]
/// Returns the data in the VM's return buffer.
pub fn returndata_acquire() -> Vec<u8> {
    let length = returndata_size();

    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);
    unsafe_returndata_copy(0, length, &mut ret);
    ret
}

/// Returns the segment of return buffer data beginning at `from` and continuing for `length` bytes.
pub fn returndata_copy(from: usize, length: usize, ret: &mut [u8]) -> Result<(), Error> {
    let size = returndata_size();

    if (size < from) || ((size - from) < length) || (ret.len() < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        unsafe_returndata_copy(from, length, ret);
        Ok(())
    }
}

/// Returns the length of the data in the VM's return buffer.
pub fn returndata_size() -> usize {
    unsafe { native::ethereum_getReturnDataSize() as usize }
}

/// Halts execution, reverts all changes to the state and consumes all gas.
pub fn abort() -> ! {
    // TODO: use assembly block with `unreachable`
    panic!()
}

/// Halts execution and reverts all changes to the state.
pub fn revert() -> ! {
    unsafe {
        native::ethereum_revert(0 as *const u32, 0 as u32);
    }
}

/// Fills the return buffer with the given data and halts execution, reverting all state changes.
pub fn revert_data(data: &[u8]) -> ! {
    unsafe {
        native::ethereum_revert(data.as_ptr() as *const u32, data.len() as u32);
    }
}

/// Ends execution, signalling success.
pub fn finish() -> ! {
    unsafe {
        native::ethereum_finish(0 as *const u32, 0 as u32);
    }
}

/// Fills the return buffer with the given data and halts execution, signalling success.
pub fn finish_data(data: &[u8]) -> ! {
    unsafe {
        native::ethereum_finish(data.as_ptr() as *const u32, data.len() as u32);
    }
}

/// Accesses the storage data at the specified key.
pub fn storage_load(key: &StorageKey) -> StorageValue {
    let mut ret = StorageValue::default();

    unsafe {
        native::ethereum_storageLoad(
            key.bytes.as_ptr() as *const u32,
            ret.bytes.as_mut_ptr() as *const u32,
        );
    }

    ret
}

/// Sets the storage data at the specified key.
pub fn storage_store(key: &StorageKey, value: &StorageValue) {
    unsafe {
        native::ethereum_storageStore(
            key.bytes.as_ptr() as *const u32,
            value.bytes.as_ptr() as *const u32,
        );
    }
}

/// Self-destructs the running contract, sending all its ether to a specified beneficiary address.
pub fn selfdestruct(address: &Address) -> ! {
    unsafe {
        native::ethereum_selfDestruct(address.bytes.as_ptr() as *const u32);
    }
}
