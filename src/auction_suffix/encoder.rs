use std::borrow::Borrow;

use ethers::{
    abi::AbiEncode,
    types::{Address, U256},
};

use crate::{
    constants::{zero_number, ZERO_ADDRESS},
    utils::{trim_0x, PadStart, Substring},
};

use super::parser::types::{AuctionPoint, AuctionWhitelistItem};

pub fn encode_auction_params(points: &Vec<AuctionPoint>) -> String {
    points
        .iter()
        .map(|p| {
            p.delay.encode_hex().pad_start(4, '0')
                + p.coefficient.encode_hex().pad_start(6, '0').borrow()
        })
        .collect()
}

pub fn encode_whitelist(whitelist: &Vec<AuctionWhitelistItem>) -> String {
    whitelist
        .iter()
        .map(|w| w.allowance.encode_hex().pad_start(8, '0') + &trim_0x(&format!("{:?}", w.address)))
        .collect()
}

pub fn encode_public_resolving_deadline(deadline: U256) -> String {
    deadline.encode_hex().pad_start(8, '0')
}

pub fn encode_taking_fee_data(taker_fee_receiver: &Address, taker_fee_ratio: &U256) -> String {
    if taker_fee_receiver == &ZERO_ADDRESS || taker_fee_ratio == &zero_number() {
        return "".to_string();
    }

    let taker_fee_ratio_hex = taker_fee_ratio.encode_hex();
    taker_fee_ratio_hex
        .substring(2, taker_fee_ratio_hex.len())
        .pad_start(24, '0')
        + &trim_0x(&format!("{:?}", taker_fee_receiver))
}

pub fn encode_flags(
    whitelist: &Vec<AuctionWhitelistItem>,
    points: &Vec<AuctionPoint>,
    taking_fee_data: &str,
) -> String {
    if points.len() > 8 {
        panic!("points count cannot be greater than 8");
    }

    let mut flags = (whitelist.len() << 3) | points.len();

    if taking_fee_data != "" {
        flags |= 0x80;
    } else {
    }

    (flags as u8).encode_hex().pad_start(2, '0')
}
