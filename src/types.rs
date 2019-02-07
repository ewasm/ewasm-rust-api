/// A little-endian unsigned 128-bit integer.
#[derive(Default, Copy, Clone)]
pub struct Uint128 {
    pub bytes: [u8; 16],
}

/// A little-endian unsigned 256-bit integer.
#[derive(Default, Copy, Clone)]
pub struct Uint256 {
    pub bytes: [u8; 32],
}

/// An array of 160 bits.
#[derive(Default, Copy, Clone)]
pub struct Bytes20 {
    pub bytes: [u8; 20],
}

/// An array of 256 bits.
#[derive(Default, Copy, Clone)]
pub struct Bytes32 {
    pub bytes: [u8; 32],
}

/// Type representing a value in wei.
pub type EtherValue = Uint128;

/// Type representing an address.
pub type Address = Bytes20;

/// Type representing a storage key.
pub type StorageKey = Bytes32;

/// Type representing a storage value.
pub type StorageValue = Bytes32;

/// Type representing a log topic.
pub type Topic = Bytes32;

/// Type representing a Keccak-256 or SHA-256 hash.
pub type Hash = Bytes32;

/// Type representing a block's difficulty.
pub type Difficulty = Uint256;
