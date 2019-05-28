//! The low-level bindings for the Ethereum Environment Interface (EEI). There is a safe set of wrappers for these functions, so use
//! those unless you are certain you know what you're doing.

extern "C" {
    pub fn ethereum_useGas(amount: u64);
    pub fn ethereum_getGasLeft() -> u64;
    pub fn ethereum_getAddress(resultOffset: *const u32);
    pub fn ethereum_getExternalBalance(addressOffset: *const u32, resultOffset: *const u32);
    pub fn ethereum_getBlockCoinbase(resultOffset: *const u32);
    pub fn ethereum_getBlockDifficulty(resultOffset: *const u32);
    pub fn ethereum_getBlockGasLimit() -> u64;
    pub fn ethereum_getBlockHash(number: u64, resultOffset: *const u32) -> u32;
    pub fn ethereum_getBlockNumber() -> u64;
    pub fn ethereum_getBlockTimestamp() -> u64;
    pub fn ethereum_getTxGasPrice(valueOffset: *const u32);
    pub fn ethereum_getTxOrigin(resultOffset: *const u32);
    pub fn ethereum_log(
        dataOffset: *const u32,
        length: u32,
        numberOfTopics: u32,
        topic1: *const u32,
        topic2: *const u32,
        topic3: *const u32,
        topic4: *const u32,
    );
    pub fn ethereum_call(
        gas: u64,
        addressOffset: *const u32,
        valueOffset: *const u32,
        dataOffset: *const u32,
        dataLength: u32,
    ) -> u32;
    pub fn ethereum_callCode(
        gas: u64,
        addressOffset: *const u32,
        valueOffset: *const u32,
        dataOffset: *const u32,
        dataLength: u32,
    ) -> u32;
    pub fn ethereum_callDelegate(
        gas: u64,
        addressOffset: *const u32,
        dataOffset: *const u32,
        dataLength: u32,
    ) -> u32;
    pub fn ethereum_callStatic(
        gas: u64,
        addressOffset: *const u32,
        dataOffset: *const u32,
        dataLength: u32,
    ) -> u32;
    pub fn ethereum_create(
        valueOffset: *const u32,
        dataOffset: *const u32,
        dataLength: u32,
        resultOffset: *const u32,
    ) -> u32;
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
    pub fn ethereum_externalCodeCopy(
        addressOffset: *const u32,
        resultOffset: *const u32,
        codeOffset: u32,
        length: u32,
    );
    pub fn ethereum_getExternalCodeSize(addressOfset: *const u32) -> u32;
    pub fn ethereum_storageLoad(keyOffset: *const u32, resultOffset: *const u32);
    pub fn ethereum_storageStore(keyOffset: *const u32, valueOffset: *const u32);
    pub fn ethereum_selfDestruct(addressOffset: *const u32) -> !;
}
