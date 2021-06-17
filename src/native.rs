//! The low-level bindings for the Ethereum Environment Interface (EEI). There is a safe set of wrappers for these functions, so use
//! those unless you are certain you know what you're doing.

#[link(wasm_import_module = "ethereum")]
extern "C" {
    pub fn useGas(amount: u64);
    pub fn getGasLeft() -> u64;
    pub fn getAddress(resultOffset: *const u32);
    pub fn getExternalBalance(addressOffset: *const u32, resultOffset: *const u32);
    pub fn getBlockCoinbase(resultOffset: *const u32);
    pub fn getBlockDifficulty(resultOffset: *const u32);
    pub fn getBlockGasLimit() -> u64;
    pub fn getBlockHash(number: u64, resultOffset: *const u32) -> u32;
    pub fn getBlockNumber() -> u64;
    pub fn getBlockTimestamp() -> u64;
    pub fn getTxGasPrice(valueOffset: *const u32);
    pub fn getTxOrigin(resultOffset: *const u32);
    pub fn log(
        dataOffset: *const u32,
        length: u32,
        numberOfTopics: u32,
        topic1: *const u32,
        topic2: *const u32,
        topic3: *const u32,
        topic4: *const u32,
    );
    pub fn call(
        gas: u64,
        addressOffset: *const u32,
        valueOffset: *const u32,
        dataOffset: *const u32,
        dataLength: u32,
    ) -> u32;
    pub fn callCode(
        gas: u64,
        addressOffset: *const u32,
        valueOffset: *const u32,
        dataOffset: *const u32,
        dataLength: u32,
    ) -> u32;
    pub fn callDelegate(
        gas: u64,
        addressOffset: *const u32,
        dataOffset: *const u32,
        dataLength: u32,
    ) -> u32;
    pub fn callStatic(
        gas: u64,
        addressOffset: *const u32,
        dataOffset: *const u32,
        dataLength: u32,
    ) -> u32;
    pub fn create(
        valueOffset: *const u32,
        dataOffset: *const u32,
        dataLength: u32,
        resultOffset: *const u32,
    ) -> u32;
    pub fn returnDataCopy(resultOffset: *const u32, dataOffset: u32, length: u32);
    pub fn getReturnDataSize() -> u32;
    pub fn finish(dataOffset: *const u32, length: u32) -> !;
    pub fn revert(dataOffset: *const u32, length: u32) -> !;
    pub fn callDataCopy(resultOffset: *const u32, dataOffset: u32, length: u32);
    pub fn getCallDataSize() -> u32;
    pub fn getCaller(resultOffset: *const u32);
    pub fn getCallValue(resultOffset: *const u32);
    pub fn codeCopy(resultOffset: *const u32, codeOffset: u32, length: u32);
    pub fn getCodeSize() -> u32;
    pub fn externalCodeCopy(
        addressOffset: *const u32,
        resultOffset: *const u32,
        codeOffset: u32,
        length: u32,
    );
    pub fn getExternalCodeSize(addressOfset: *const u32) -> u32;
    pub fn storageLoad(keyOffset: *const u32, resultOffset: *const u32);
    pub fn storageStore(keyOffset: *const u32, valueOffset: *const u32);
    pub fn selfDestruct(addressOffset: *const u32) -> !;
}
