pub mod constants;

use ethers::core::types::U256;

use constants::*;

pub fn get_start_time(salt: &str) -> U256 {
    let val = (U256::from_dec_str(salt).unwrap() & time_start_mask()) >> U256::from(TIME_START_SHIFT);
    val
}

pub fn get_duration(salt: &str) -> U256 {
    let val = (U256::from_dec_str(salt).unwrap() & duration_mask()) >> U256::from(DURATION_SHIFT);
    val
}

pub fn get_initial_rate_bump(salt: &str) -> U256 {
    let val =
        (U256::from_dec_str(salt).unwrap() & initial_rate_bump_mask()) >> U256::from(INITIAL_RATE_BUMP_SHIFT);
    val
}

pub fn get_fee(salt: &str) -> U256 {
    let val = (U256::from_dec_str(salt).unwrap() & fee_mask()) >> U256::from(FEE_SHIFT);
    val
}

pub fn get_salt(salt: &str) -> U256 {
    let val = U256::from_dec_str(&salt).unwrap() & salt_mask();

    val
}
