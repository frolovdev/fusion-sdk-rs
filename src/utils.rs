use std::str::FromStr;

use crate::constants::NATIVE_CURRENCY;

pub fn is_native_currency(address: &str) -> bool {
    address.to_lowercase() == NATIVE_CURRENCY
}

pub fn to_sec<T: Into<u64>>(time: T) -> u64 {
    let t = time.into();
    t / 1000
}

// pub fn to_bn<T: AsRef<str>>(val: T) -> BigUint {
//     let val = val.as_ref();
//     if let Ok(n) = val.parse::<u64>() {
//         return n.to_biguint().expect("Failed to convert to biguint");
//     }
//     let val = trim_0x(val);
//     BigUint::from_str(val).expect("Invalid hex string")
// }

pub fn trim_0x(data: &str) -> &str {
    if data.starts_with("0x") {
        &data[2..]
    } else {
        data
    }
}

pub fn add_0x(data: &str) -> String {
    if data.contains("0x") {
        data.to_string()
    } else {
        "0x".to_owned() + data
    }
}