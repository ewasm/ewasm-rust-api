/// The native debug interface exposed to the ewasm contract. These functions are for testing
/// purposes only. On a live VM, any bytecode trying to import these symbols will be rejected.
extern "C" {
    pub fn debug_print32(value: u32);
    pub fn debug_print64(value: u64);
    pub fn debug_printMem(offset: *const u32, len: u32);
    pub fn debug_printMemHex(offset: *const u32, len: u32);
    pub fn debug_printStorage(pathOffset: *const u32);
    pub fn debug_printStorageHex(pathOffset: *const u32);
}
