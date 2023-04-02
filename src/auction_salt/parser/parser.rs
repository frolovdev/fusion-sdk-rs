use std::ops::Shr;
use std::str::FromStr;

use ruint::aliases::*;
use ruint::Uint;

use super::constants::*;

pub fn get_start_time(salt: &str) -> U256 {
    let val = (U256::from_str(salt).unwrap() & time_start_mask()).shr(&TIME_START_SHIFT);
    val
}

pub fn get_duration(salt: &str) -> U256 {
    let val = (U256::from_str(salt).unwrap() & duration_mask()).shr(&DURATION_SHIFT);
    val
}

pub fn get_initial_rate_bump(salt: &str) -> U256 {
    let val =
        (U256::from_str(salt).unwrap() & initial_rate_bump_mask()).shr(&INITIAL_RATE_BUMP_SHIFT);
    val
}

pub fn get_fee(salt: &str) -> U256 {
    let val = (U256::from_str(salt).unwrap() & fee_mask()).shr(&FEE_SHIFT);
    val
}

pub fn get_salt(salt: &str) -> U256 {
    let val = U256::from_str(salt).unwrap() & salt_mask();
    val
}
