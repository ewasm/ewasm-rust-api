use std::vec::Vec;

mod native {
    extern "C" {
        pub fn ethereum_useGas(amount: u64);
        pub fn ethereum_getGasLeft() -> u64;
        pub fn ethereum_getAddress(resultOffset: *const u32);
        pub fn ethereum_getBalance(addressOffset: *const u32, resultOffset: *const u32);
        pub fn ethereum_getBlockCoinbase(resultOffset: *const u32);
        pub fn ethereum_getBlockDifficulty(resultOffset: *const u32);
        pub fn ethereum_getBlockGasLimit() -> u64;
        pub fn ethereum_getBlockHash(number: u64, resultOffset: *const u32) -> u32;
        pub fn ethereum_getBlockNumber() -> u64;
        pub fn ethereum_getBlockTimestamp() -> u64;
        pub fn ethereum_getTxGasPrice(valueOffset: *const u32);
        pub fn ethereum_getTxOrigin(resultOffset: *const u32);
        pub fn ethereum_log(dataOffset: *const u32, length: u32, numberOfTopics: u32, topic1: *const u32, topic2: *const u32, topic3: *const u32, topic4: *const u32);
        pub fn ethereum_call(gas: u64, addressOffset: *const u32, valueOffset: *const u32, dataOffset: *const u32, dataLength: u32) -> u32;
        pub fn ethereum_callCode(gas: u64, addressOffset: *const u32, valueOffset: *const u32, dataOffset: *const u32, dataLength: u32) -> u32;
        pub fn ethereum_callDelegate(gas: u64, addressOffset: *const u32, dataOffset: *const u32, dataLength: u32) -> u32;
        pub fn ethereum_callStatic(gas: u64, addressOffset: *const u32, dataOffset: *const u32, dataLength: u32) -> u32;
        pub fn ethereum_create(valueOffset: *const u32, dataOffset: *const u32, dataLength: u32, resultOffset: *const u32) -> u32;
        pub fn ethereum_returnDataCopy(resultOffset: *const u32, dataOffset: u32, length: u32);
        pub fn ethereum_getReturnDataSize() -> u32;
        pub fn ethereum_finish(dataOffset: *const u32, length: u32) -> !;
        pub fn ethereum_revert(dataOffset: *const u32, length: u32) -> !;
        pub fn ethereum_callDataCopy(resultOffset: *const u32, dataOffset: u32, length: u32);
        pub fn ethereum_getCallDataSize() -> u32;
        pub fn ethereum_getCaller(resultOffset: *const u32);
        pub fn ethereum_getCallValue(resultOffset: *const u32);
        pub fn ethereum_codeCopy(resultOffset: *const u32, codeOffset: u32, length: u32);
        pub fn ethereum_getCodeSize() -> u32;
        pub fn ethereum_externalCodeCopy(addressOffset: *const u32, resultOffset: *const u32, codeOffset: u32, length: u32);
        pub fn ethereum_getExternalCodeSize(addressOfset: *const u32) -> u32;
        pub fn ethereum_storageLoad(keyOffset: *const u32, resultOffset: *const u32);
        pub fn ethereum_storageStore(keyOffset: *const u32, valueOffset: *const u32);
        pub fn ethereum_selfDestruct(addressOffset: *const u32) -> !;
    }
}

fn unsafe_alloc_buffer(len: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(len);
    unsafe {
        ret.set_len(len);
    }
    ret
}

pub enum Error {
    OutOfBoundsCopy
}

pub enum CallResult {
    Successful,
    Failure,
    Revert
}

pub enum CreateResult {
    Successful([u8;20]),
    Failure,
    Revert
}

pub fn consume_gas(amount: u64) {
    unsafe {
        native::ethereum_useGas(amount);
    }
}

pub fn gas_left() -> u64 {
    unsafe {
        native::ethereum_getGasLeft()
    }
}

pub fn current_address() -> [u8;20] {
    let mut ret = [0u8;20];

    unsafe {
        native::ethereum_getAddress(ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn external_balance(address: &[u8;20]) -> [u8;16] {
    let mut ret = [0u8;16];

    unsafe {
        native::ethereum_getBalance(address.as_ptr() as *const u32, ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn block_coinbase() -> [u8;20] {
    let mut ret = [0u8;20];

    unsafe {
        native::ethereum_getBlockCoinbase(ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn block_difficulty() -> [u8;32] {
    let mut ret = [0u8;32];

    unsafe {
        native::ethereum_getBlockDifficulty(ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn block_gas_limit() -> u64 {
    unsafe {
        native::ethereum_getBlockGasLimit()
    }
}

pub fn block_hash(number: u64) -> [u8;32] {
    let mut ret = [0u8;32];

    unsafe {
        native::ethereum_getBlockHash(number, ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn block_number() -> u64 {
    unsafe {
        native::ethereum_getBlockNumber()
    }
}

pub fn block_timestamp() -> u64 {
    unsafe {
        native::ethereum_getBlockTimestamp()
    }
}

pub fn tx_gas_price() -> [u8;16] {
    let mut ret = [0u8;16];

    unsafe {
        native::ethereum_getTxGasPrice(ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn tx_origin() -> [u8;20] {
    let mut ret = [0u8;20];

    unsafe {
        native::ethereum_getTxOrigin(ret.as_mut_ptr() as *const u32);
    }
    ret
}

fn log(data: &[u8], topic_count: usize, topic1: *const u8, topic2: *const u8, topic3: *const u8, topic4: *const u8) {
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

pub fn log0(data: &[u8]) {
    log(data, 0, 0 as *const u8, 0 as *const u8, 0 as *const u8, 0 as *const u8)
}

pub fn log1(data: &[u8], topic1: [u8;32]) {
    log(data, 1, topic1.as_ptr() as *const u8, 0 as *const u8, 0 as *const u8, 0 as *const u8)
}

pub fn log2(data: &[u8], topic1: [u8;32], topic2: [u8;32]) {
    log(data, 2, topic1.as_ptr() as *const u8, topic2.as_ptr() as *const u8, 0 as *const u8, 0 as *const u8)
}

pub fn log3(data: &[u8], topic1: [u8;32], topic2: [u8;32], topic3: [u8;32]) {
    log(data, 3, topic1.as_ptr() as *const u8, topic2.as_ptr() as *const u8, topic3.as_ptr() as *const u8, 0 as *const u8)
}

pub fn log4(data: &[u8], topic1: [u8;32], topic2: [u8;32], topic3: [u8;32], topic4: [u8;32]) {
    log(data, 4, topic1.as_ptr() as *const u8, topic2.as_ptr() as *const u8, topic3.as_ptr() as *const u8, topic4.as_ptr() as *const u8)
}

pub fn call_mutable(gas_limit: u64, address: &[u8;20], value: &[u8;16], data: &[u8]) -> CallResult {
    let ret = unsafe {
        native::ethereum_call(
            gas_limit,
            address.as_ptr() as *const u32,
            value.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => panic!()
    }
}

pub fn call_code(gas_limit: u64, address: &[u8;20], value: &[u8;16], data: &[u8]) -> CallResult {
    let ret = unsafe {
        native::ethereum_callCode(
            gas_limit,
            address.as_ptr() as *const u32,
            value.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => panic!()
    }
}

pub fn call_delegate(gas_limit: u64, address: &[u8;20], data: &[u8]) -> CallResult {
    let ret = unsafe {
        native::ethereum_callDelegate(
            gas_limit,
            address.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32
        )
    };


    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => panic!()
    }
}

pub fn call_static(gas_limit: u64, address: &[u8;20], data: &[u8]) -> CallResult {
    let ret = unsafe {
        native::ethereum_callStatic(
            gas_limit,
            address.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32
        )
    };

    match ret {
        0 => CallResult::Successful,
        1 => CallResult::Failure,
        2 => CallResult::Revert,
        _ => panic!()
    }
}

pub fn create(value: &[u8;16], data: &[u8]) -> CreateResult {
    let mut result = [0u8;20];

    let ret = unsafe {
         native::ethereum_create(
            value.as_ptr() as *const u32,
            data.as_ptr() as *const u32,
            data.len() as u32,
            result.as_mut_ptr() as *const u32
        )
    };

    match ret {
        0 => CreateResult::Successful(result),
        1 => CreateResult::Failure,
        2 => CreateResult::Revert,
        _ => panic!()
    }
}

pub fn unsafe_calldata_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        native::ethereum_callDataCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }
    ret
}

pub fn calldata_copy(from: usize, length: usize) -> Result<Vec<u8>, Error> {
    let size = calldata_size();

    if (size < from) || ((size - from) < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        Ok(unsafe_calldata_copy(from, length))
    }
}

pub fn calldata_size() -> usize {
    unsafe {
        return native::ethereum_getCallDataSize() as usize;
    }
}

pub fn caller() -> [u8;20] {
    let mut ret = [0u8;20];

    unsafe {
        native::ethereum_getCaller(ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn callvalue() -> [u8;16] {
    let mut ret = [0u8;16];

    unsafe {
        native::ethereum_getCallValue(ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn unsafe_code_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        native::ethereum_codeCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }
    ret
}

pub fn code_copy(from: usize, length: usize) -> Result<Vec<u8>, Error> {
    let size = code_size();

    if (size < from) || ((size - from) < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        Ok(unsafe_code_copy(from, length))
    }
}

pub fn code_size() -> usize {
    unsafe {
        native::ethereum_getCodeSize() as usize
    }
}


pub fn unsafe_external_code_copy(address: &[u8;20], from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        native::ethereum_externalCodeCopy(address.as_ptr() as *const u32, ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }

    ret
}

pub fn external_code_copy(address: &[u8;20], from: usize, length: usize) -> Result<Vec<u8>, Error> {
    let size = external_code_size(address);

    if (size < from) || ((size - from) < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        Ok(unsafe_external_code_copy(address, from, length))
    }
}

pub fn external_code_size(address: &[u8;20]) -> usize {
    unsafe {
        native::ethereum_getExternalCodeSize(address.as_ptr() as *const u32) as usize
    }
}

pub fn unsafe_returndata_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        native::ethereum_returnDataCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }

    ret
}

pub fn returndata_copy(from: usize, length: usize) -> Result<Vec<u8>, Error> {
    let size = returndata_size();

    if (size < from) || ((size - from) < length) {
        Err(Error::OutOfBoundsCopy)
    } else {
        Ok(unsafe_returndata_copy(from, length))
    }
}

pub fn returndata_size() -> usize {
    unsafe {
        native::ethereum_getReturnDataSize() as usize
    }
}

pub fn revert() -> ! {
    unsafe {
        native::ethereum_revert(0 as *const u32, 0 as u32);
    }
}

pub fn revert_data(data: &[u8]) -> ! {
    unsafe {
        native::ethereum_revert(data.as_ptr() as *const u32, data.len() as u32);
    }
}

pub fn finish() -> ! {
    unsafe {
        native::ethereum_finish(0 as *const u32, 0 as u32);
    }
}

pub fn finish_data(data: &[u8]) -> ! {
    unsafe {
        native::ethereum_finish(data.as_ptr() as *const u32, data.len() as u32);
    }
}

pub fn storage_load(key: &[u8;32]) -> [u8;32] {
    let mut ret = [0u8;32];

    unsafe {
        native::ethereum_storageLoad(key.as_ptr() as *const u32, ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn storage_store(key: &[u8;32], value: &[u8;32]) {
    unsafe {
        native::ethereum_storageStore(key.as_ptr() as *const u32, value.as_ptr() as *const u32);
    }
}

pub fn selfdestruct(address: &[u8; 20]) -> ! {
    unsafe {
        native::ethereum_selfDestruct(address.as_ptr() as *const u32);
    }
}
