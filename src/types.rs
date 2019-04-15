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
pub type LogTopic = Bytes32;

/// Type representing a Keccak-256 or SHA-256 hash.
pub type Hash = Bytes32;

/// Type representing a block's difficulty.
pub type Difficulty = Uint256;

macro_rules! from_primitive_impl {
    ($f:ident, $size:expr, $to:ident) => {
        impl From<[$f; $size]> for $to {
            fn from(a: [$f; $size]) -> Self {
                $to { bytes: a }
            }
        }
    };
}

macro_rules! from_primitive_ref_impl {
    ($f:ident, $size:expr, $to:ident) => {
        impl From<&[$f; $size]> for $to {
            fn from(a: &[$f; $size]) -> Self {
                $to { bytes: a.clone() }
            }
        }
    };
}

macro_rules! from_type_for_primitive_impl {
    ($f:ident, $to:ident, $size:expr) => {
        impl From<$f> for [$to; $size] {
            fn from(a: $f) -> Self {
                a.bytes
            }
        }
    };
}

from_primitive_impl!(u8, 16, Uint128);
from_primitive_impl!(u8, 32, Uint256);
from_primitive_impl!(u8, 20, Bytes20);
from_primitive_impl!(u8, 32, Bytes32);

from_primitive_ref_impl!(u8, 16, Uint128);
from_primitive_ref_impl!(u8, 32, Uint256);
from_primitive_ref_impl!(u8, 20, Bytes20);
from_primitive_ref_impl!(u8, 32, Bytes32);

from_type_for_primitive_impl!(Uint128, u8, 16);
from_type_for_primitive_impl!(Uint256, u8, 32);
from_type_for_primitive_impl!(Bytes20, u8, 20);
from_type_for_primitive_impl!(Bytes32, u8, 32);

#[cfg(test)]
mod tests {
    use super::{Bytes20, Bytes32, Uint128, Uint256};

    macro_rules! test_conversions {
        ($type: ident, $size: expr, $test_name: ident) => {
            #[test]
            fn $test_name() {
                let raw = [1; $size];

                let uint = $type::from(raw);
                assert_eq!(uint.bytes[$size - 1], 1);
                let uint = $type::from(&raw);
                assert_eq!(uint.bytes[$size - 1], 1);

                let uint: $type = raw.into();
                assert_eq!(uint.bytes[$size - 1], 1);
                let uint: $type = (&raw).into();
                assert_eq!(uint.bytes[$size - 1], 1);

                let r: [u8; $size] = uint.into();
                assert_eq!(r[$size - 1], 1);
            }
        };
    }

    test_conversions!(Uint128, 16, test_uint128);
    test_conversions!(Uint256, 32, test_uint256);
    test_conversions!(Bytes20, 20, test_bytes20);
    test_conversions!(Bytes32, 32, test_bytes32);
}
