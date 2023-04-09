use std::collections::HashMap;

use ethers::types::{Bytes, U256};

use crate::utils::Maskn;

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

pub fn get_offset_for_interaction(offsets: &U256, field: u8) -> (usize, usize) {
    let from_byte = if field == 0 {
        U256::from(0)
    } else {
        (offsets >> ((field - 1) * 32)).maskn(32)
    };

    let to_byte = (offsets >> (field * 32)).maskn(32);

    return (from_byte.as_usize(), to_byte.as_usize());
}
