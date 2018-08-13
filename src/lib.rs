use std::vec::Vec;

extern "C" {
    fn ethereum_useGas(amount: u64);
    fn ethereum_getGasLeft() -> u64;
    fn ethereum_getAddress(resultOffset: *const u32);
    fn ethereum_getBalance(addressOffset: *const u32, resultOffset: *const u32);
    fn ethereum_getBlockCoinbase(resultOffset: *const u32);
    fn ethereum_getBlockDifficulty(resultOffset: *const u32);
    fn ethereum_getBlockGasLimit() -> u64;
    fn ethereum_getBlockHash(number: u64, resultOffset: *const u32) -> u32;
    fn ethereum_getBlockNumber() -> u64;
    fn ethereum_getBlockTimestamp() -> u64;
    fn ethereum_getTxGasPrice(valueOffset: *const u32);
    fn ethereum_getTxOrigin(resultOffset: *const u32);
    fn ethereum_log(dataOffset: *const u32, length: u32, numberOfTopics: u32, topic1: *const u32, topic2: *const u32, topic3: *const u32, topic4: *const u32);
    fn ethereum_call(gas: u64, addressOffset: *const u32, valueOffset: *const u32, dataOffset: *const u32, dataLength: u32) -> u32;
    fn ethereum_callCode(gas: u64, addressOffset: *const u32, valueOffset: *const u32, dataOffset: *const u32, dataLength: u32) -> u32;
    fn ethereum_callDelegate(gas: u64, addressOffset: *const u32, dataOffset: *const u32, dataLength: u32) -> u32;
    fn ethereum_callStatic(gas: u64, addressOffset: *const u32, dataOffset: *const u32, dataLength: u32) -> u32;
    fn ethereum_create(valueOffset: *const u32, dataOffset: *const u32, dataLength: u32, resultOffset: *const u32) -> u32;
    fn ethereum_returnDataCopy(resultOffset: *const u32, dataOffset: u32, length: u32);
    fn ethereum_getReturnDataSize() -> u32;
    fn ethereum_finish(dataOffset: *const u32, length: u32) -> !;
    fn ethereum_revert(dataOffset: *const u32, length: u32) -> !;
    fn ethereum_callDataCopy(resultOffset: *const u32, dataOffset: u32, length: u32);
    fn ethereum_getCallDataSize() -> u32;
    fn ethereum_getCaller(resultOffset: *const u32);
    fn ethereum_getCallValue(resultOffset: *const u32);
    fn ethereum_codeCopy(resultOffset: *const u32, codeOffset: u32, length: u32);
    fn ethereum_getCodeSize() -> u32;
    fn ethereum_externalCodeCopy(addressOffset: *const u32, resultOffset: *const u32, codeOffset: u32, length: u32);
    fn ethereum_getExternalCodeSize(addressOfset: *const u32) -> u32;
    fn ethereum_storageLoad(keyOffset: *const u32, resultOffset: *const u32);
    fn ethereum_storageStore(keyOffset: *const u32, valueOffset: *const u32);
    fn ethereum_selfDestruct(addressOffset: *const u32) -> !;
}

fn unsafe_alloc_buffer(len: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(len);
    unsafe {
        ret.set_len(len);
    }
    ret
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
        ethereum_useGas(amount);
    }
}

pub fn gas_left() -> u64 {
    unsafe {
        ethereum_getGasLeft()
    }
}

pub fn current_address() -> [u8;20] {
    let mut ret = [0u8;20];

    unsafe {
        ethereum_getAddress(ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn external_balance(address: &[u8;20]) -> [u8;16] {
    let mut ret = [0u8;16];

    unsafe {
        ethereum_getBalance(address.as_ptr() as *const u32, ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn block_coinbase() -> [u8;20] {
    let mut ret = [0u8;20];

    unsafe {
        ethereum_getBlockCoinbase(ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn block_difficulty() -> [u8;32] {
    let mut ret = [0u8;32];

    unsafe {
        ethereum_getBlockDifficulty(ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn block_gas_limit() -> u64 {
    unsafe {
        ethereum_getBlockGasLimit()
    }
}

pub fn block_hash(number: u64) -> [u8;32] {
    let mut ret = [0u8;32];

    unsafe {
        ethereum_getBlockHash(number, ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn block_number() -> u64 {
    unsafe {
        ethereum_getBlockNumber()
    }
}

pub fn block_timestamp() -> u64 {
    unsafe {
        ethereum_getBlockTimestamp()
    }
}

pub fn tx_gas_price() -> [u8;16] {
    let mut ret = [0u8;16];

    unsafe {
        ethereum_getTxGasPrice(ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn tx_origin() -> [u8;20] {
    let mut ret = [0u8;20];

    unsafe {
        ethereum_getTxOrigin(ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn log(data: &[u8], topics: &Vec<[u8;32]>) {
    let topics_count = topics.len();
    assert!(topics_count <= 4);

    let mut topic_ptrs = [0 as *const u8;4];
    for i in 0..topics_count {
        topic_ptrs[i] = topics[i].as_ptr()
    }

    unsafe {
        ethereum_log(
            data.as_ptr() as *const u32,
            data.len() as u32,
            topics_count as u32,
            topic_ptrs[0] as *const u32,
            topic_ptrs[1] as *const u32,
            topic_ptrs[2] as *const u32,
            topic_ptrs[3] as *const u32,
        );
    }
}

pub fn call_mutable(gas_limit: u64, address: &[u8;20], value: &[u8;16], data: &[u8]) -> CallResult {
    let ret = unsafe {
        ethereum_call(
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
        ethereum_callCode(
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
        ethereum_callDelegate(
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
        ethereum_callStatic(
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
         ethereum_create(
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

pub fn calldata_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        ethereum_callDataCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }
    ret
}

pub fn calldata_size() -> usize {
    unsafe {
        return ethereum_getCallDataSize() as usize;
    }
}

pub fn caller() -> [u8;20] {
    let mut ret = [0u8;20];

    unsafe {
        ethereum_getCaller(ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn callvalue() -> [u8;16] {
    let mut ret = [0u8;16];

    unsafe {
        ethereum_getCallValue(ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn code_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        ethereum_codeCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }
    ret
}

pub fn code_size() -> usize {
    unsafe {
        ethereum_getCodeSize() as usize
    }
}

pub fn external_code_copy(address: &[u8;20], from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        ethereum_externalCodeCopy(address.as_ptr() as *const u32, ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }
    ret
}

pub fn external_code_size(address: &[u8;20]) -> usize {
    unsafe {
        ethereum_getExternalCodeSize(address.as_ptr() as *const u32) as usize
    }
}

pub fn returndata_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = unsafe_alloc_buffer(length);

    unsafe {
        ethereum_returnDataCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
    }
    ret
}

pub fn returndata_size() -> usize {
    unsafe {
        ethereum_getReturnDataSize() as usize
    }
}

pub fn revert() -> ! {
    unsafe {
        ethereum_revert(0 as *const u32, 0 as u32);
    }
}

pub fn revert_data(data: &[u8]) -> ! {
    unsafe {
        ethereum_revert(data.as_ptr() as *const u32, data.len() as u32);
    }
}

pub fn finish() -> ! {
    unsafe {
        ethereum_finish(0 as *const u32, 0 as u32);
    }
}

pub fn finish_data(data: &[u8]) -> ! {
    unsafe {
        ethereum_finish(data.as_ptr() as *const u32, data.len() as u32);
    }
}

pub fn storage_load(key: &[u8;32]) -> [u8;32] {
    let mut ret = [0u8;32];

    unsafe {
        ethereum_storageLoad(key.as_ptr() as *const u32, ret.as_mut_ptr() as *const u32);
    }
    ret
}

pub fn storage_store(key: &[u8;32], value: &[u8;32]) {
    unsafe {
        ethereum_storageStore(key.as_ptr() as *const u32, value.as_ptr() as *const u32);
    }
}

pub fn selfdestruct(address: &[u8; 20]) -> ! {
    unsafe {
        ethereum_selfDestruct(address.as_ptr() as *const u32);
    }
}
