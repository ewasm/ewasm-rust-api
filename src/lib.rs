use std::vec::Vec;

extern "C" {
    fn ethereum_useGas(amount: u64);
    fn ethereum_getAddress(resultOffset: *const u32);
    fn ethereum_getBalance(addressOffset: *const u32, resultOffset: *const u32);
    fn ethereum_revert(dataOffset: *const u32, length: u32) -> !;
    fn ethereum_return(dataOffset: *const u32, length: u32) -> !;
    fn ethereum_callDataCopy(resultOffset: *const u32, dataOffset: u32, length: u32);
    fn ethereum_callDataSize() -> u32;
    fn ethereum_storageLoad(keyOffset: *const u32, resultOffset: *const u32);
    fn ethereum_storageStore(keyOffset: *const u32, valueOffset: *const u32);
}

pub fn consume_gas(amount: u64) {
    unsafe {
        ethereum_useGas(amount);
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
        return ethereum_callDataSize() as usize;
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
        ethereum_return(0 as *const u32, 0 as u32);
    }
}

pub fn finish_data(data: Vec<u8>) -> ! {
    unsafe {
        ethereum_return(data.as_ptr() as *const u32, data.len() as u32);
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
