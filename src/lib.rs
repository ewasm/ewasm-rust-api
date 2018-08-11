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

pub fn consume_gas(amount: u64) {
    unsafe {
        ethereum_useGas(amount);
    }
}

pub fn gas_left() -> u64 {
    unsafe {
        return ethereum_getGasLeft();
    }
}

pub fn current_address() -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(20);

    unsafe {
        ethereum_getAddress(ret.as_mut_ptr() as *const u32);
        ret.set_len(20);
    }

    return ret;
}

pub fn external_balance(address: Vec<u8>) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(16);

    unsafe {
        ethereum_getBalance(address.as_ptr() as *const u32, ret.as_mut_ptr() as *const u32);
        ret.set_len(16);
    }

    return ret;
}

pub fn block_coinbase() -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(20);

    unsafe {
        ethereum_getBlockCoinbase(ret.as_mut_ptr() as *const u32);
        ret.set_len(20);
    }

    return ret;
}

pub fn block_difficulty() -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(32);

    unsafe {
        ethereum_getBlockDifficulty(ret.as_mut_ptr() as *const u32);
        ret.set_len(32);
    }

    return ret;
}

pub fn block_gas_limit() -> u64 {
    unsafe {
        return ethereum_getBlockGasLimit();
    }
}

pub fn block_hash(number: u64) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(32);

    unsafe {
        ethereum_getBlockHash(number, ret.as_mut_ptr() as *const u32);
        ret.set_len(32);
    }

    return ret;
}

pub fn block_number() -> u64 {
    unsafe {
        return ethereum_getBlockNumber();
    }
}

pub fn block_timestamp() -> u64 {
    unsafe {
        return ethereum_getBlockTimestamp();
    }
}

pub fn tx_gas_price() -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(16);

    unsafe {
        ethereum_getTxGasPrice(ret.as_mut_ptr() as *const u32);
        ret.set_len(16);
    }

    return ret;
}

pub fn tx_origin() -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(20);

    unsafe {
        ethereum_getTxOrigin(ret.as_mut_ptr() as *const u32);
        ret.set_len(20);
    }

    return ret;
}

#[warn(non_snake_case)]
pub fn calldata_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(length);

    unsafe {
        ethereum_callDataCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
        ret.set_len(length);
    }

    return ret;
}

pub fn calldata_size() -> usize {
    unsafe {
        return ethereum_getCallDataSize() as usize;
    }
}

pub fn caller() -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(20);

    unsafe {
        ethereum_getCaller(ret.as_mut_ptr() as *const u32);
        ret.set_len(20);
    }

    return ret;
}

pub fn callvalue() -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(16);

    unsafe {
        ethereum_getCallValue(ret.as_mut_ptr() as *const u32);
        ret.set_len(16);
    }

    return ret;
}

pub fn code_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(length);

    unsafe {
        ethereum_codeCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
        ret.set_len(length);
    }

    return ret;
}

pub fn code_size() -> usize {
    unsafe {
        return ethereum_getCodeSize() as usize;
    }
}

pub fn external_code_copy(address: Vec<u8>, from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(length);

    unsafe {
        ethereum_externalCodeCopy(address.as_ptr() as *const u32, ret.as_mut_ptr() as *const u32, from as u32, length as u32);
        ret.set_len(length);
    }

    return ret;
}

pub fn external_code_size(address: Vec<u8>) -> usize {
    unsafe {
        return ethereum_getExternalCodeSize(address.as_ptr() as *const u32) as usize;
    }
}

pub fn returndata_copy(from: usize, length: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(length);

    unsafe {
        ethereum_returnDataCopy(ret.as_mut_ptr() as *const u32, from as u32, length as u32);
        ret.set_len(length);
    }

    return ret;
}

pub fn returndata_size() -> usize {
    unsafe {
        return ethereum_getReturnDataSize() as usize;
    }
}

pub fn revert() -> ! {
    unsafe {
        ethereum_revert(0 as *const u32, 0 as u32);
    }
}

pub fn revert_data(data: Vec<u8>) -> ! {
    unsafe {
        ethereum_revert(data.as_ptr() as *const u32, data.len() as u32);
    }
}

pub fn finish() -> ! {
    unsafe {
        ethereum_finish(0 as *const u32, 0 as u32);
    }
}

pub fn finish_data(data: Vec<u8>) -> ! {
    unsafe {
        ethereum_finish(data.as_ptr() as *const u32, data.len() as u32);
    }
}

pub fn storage_load(key: Vec<u8>) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::with_capacity(32);

    unsafe {
        ethereum_storageLoad(key.as_ptr() as *const u32, ret.as_mut_ptr() as *const u32);
        ret.set_len(32);
    }

    return ret;
}

pub fn storage_store(key: Vec<u8>, value: Vec<u8>) {
    unsafe {
        ethereum_storageStore(key.as_ptr() as *const u32, value.as_ptr() as *const u32);
    }
}

pub fn selfdestruct(address: Vec<u8>) -> ! {
    unsafe {
        ethereum_selfDestruct(address.as_ptr() as *const u32);
    }
}
