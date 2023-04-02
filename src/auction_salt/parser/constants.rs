use std::str::FromStr;

use ethers::core::types::{U256};

pub fn time_start_mask() -> U256 {
    U256::from_str("0xFFFFFFFF00000000000000000000000000000000000000000000000000000000").unwrap()
}

pub fn duration_mask() -> U256 {
    U256::from_str("0x00000000FFFFFF00000000000000000000000000000000000000000000000000").unwrap()
}

pub fn initial_rate_bump_mask() -> U256 {
    U256::from_str("0x00000000000000FFFFFF00000000000000000000000000000000000000000000").unwrap()
}

pub fn fee_mask() -> U256 {
    U256::from_str("0x00000000000000000000FFFFFFFF000000000000000000000000000000000000").unwrap()
}

pub fn salt_mask() -> U256 {
    U256::from_str("0x0000000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap()
}

pub const TIME_START_SHIFT: usize = 224; // orderTimeMask 224-255
pub const DURATION_SHIFT: usize = 200; // durationMask 200-223
pub const INITIAL_RATE_BUMP_SHIFT: usize = 176; // initialRateMask 176-200
pub const FEE_SHIFT: usize = 144; // orderFee 144-175
