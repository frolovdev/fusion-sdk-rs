pub mod constants;

use ethers::core::types::U256;

use constants::*;

pub fn get_start_time(salt: &U256) -> U256 {
    let val = (salt.to_owned() & time_start_mask()) >> U256::from(TIME_START_SHIFT);
    val
}

pub fn get_duration(salt: &U256) -> U256 {
    let val = (salt.to_owned() & duration_mask()) >> U256::from(DURATION_SHIFT);
    val
}

pub fn get_initial_rate_bump(salt: &U256) -> U256 {
    let val =
        (salt.to_owned() & initial_rate_bump_mask()) >> U256::from(INITIAL_RATE_BUMP_SHIFT);
    val
}

pub fn get_fee(salt: &U256) -> U256 {
    let val = (salt.to_owned() & fee_mask()) >> U256::from(FEE_SHIFT);
    val
}

pub fn get_salt(salt: &U256) -> U256 {
    let val = salt.to_owned() & salt_mask();

    val
}
