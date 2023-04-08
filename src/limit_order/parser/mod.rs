use std::{cmp::min, collections::HashMap, ops::BitAnd, str::FromStr};

use ethers::types::{Bytes, U256};

type ParsedInteractions = HashMap<String, Bytes>;

pub fn parse_interactions(offsets: &U256, interactions: &Bytes) -> ParsedInteractions {
    let layout = vec![
        "maker_asset_data".to_string(),
        "taker_asset_data".to_string(),
        "get_making_amount".to_string(),
        "get_taking_amount".to_string(),
        "predicate".to_string(),
        "permit".to_string(),
        "pre_interaction".to_string(),
        "post_interaction".to_string(),
    ];

    let mut parsed_interactions = ParsedInteractions::new();

    for (index, element) in layout.iter().enumerate() {
        let interaction = parse_interaction_field(&offsets, &interactions, index as u8);

        parsed_interactions.insert(element.to_owned(), interaction);
    }

    parsed_interactions
}

pub fn parse_interaction_field(offsets: &U256, interactions: &Bytes, field: u8) -> Bytes {
    let (from_byte, to_byte) = get_offset_for_interaction(offsets, field);

    let interaction = &interactions[from_byte..to_byte];

    return Bytes::from(interaction.to_owned());
}

fn maskn(n: &U256, bits: usize) -> U256 {
    let mut n = n.to_owned();
    assert!(n >= U256::from(0), "maskn works only with positive numbers");

    let r = bits % 26;
    let s = (bits - r) / 26;

    let n_len = n.bits();
    if n_len <= s {
        return n.to_owned();
    }

    let s = if r != 0 { s + 1 } else { s };

    n = n & ((U256::from(1) << (26 * s)) - 1);

    if r != 0 {
        let mask = 0x3ffffff ^ ((0x3ffffff >> r) << r);
        let last_word = n >> (26 * (s - 1));
        let new_last_word = last_word & U256::from(mask);
        n = n - ((last_word - new_last_word) << (26 * (s - 1)));
    }

    n
}

pub fn get_offset_for_interaction(offsets: &U256, field: u8) -> (usize, usize) {
    let from_byte = if field == 0 {
        U256::from(0)
    } else {
        maskn(&(offsets >> ((field - 1) * 32)), 32)
    };

    let to_byte = maskn(&(offsets >> (field * 32)), 32);

    return (from_byte.as_usize(), to_byte.as_usize());
}
