use std::{collections::HashMap, str::FromStr};

use ethers::types::U256;

use crate::utils::trim_0x;

// pub struct ParsedInteractions {
//     pub maker_asset_data: String,
//     pub taker_asset_data: String,
//     pub get_making_amount: String,
//     pub get_taking_amount: String,
//     pub predicate: String,
//     pub permit: String,
//     pub pre_interaction: String,
//     pub post_interaction: String,
// }

type ParsedInteractions = HashMap<String, String>;

pub fn parse_interactions(offsets: &str, interactions: &str) -> ParsedInteractions {
    let offsets_u256 = U256::from_str(&offsets).unwrap();
    let trimmed_interactions = trim_0x(&interactions);

    let map = HashMap::from([
        ("maker_asset_data".to_string(), 0),
        ("taker_asset_data".to_string(), 1),
        ("get_making_amount".to_string(), 2),
        ("get_taking_amount".to_string(), 3),
        ("predicate".to_string(), 4),
        ("permit".to_string(), 5),
        ("pre_interaction".to_string(), 6),
        ("post_interaction".to_string(), 7),
    ]);

    let mut parsed_interactions = ParsedInteractions::new();

    for (key, value) in map {
        let interaction = parse_interaction_field(&offsets_u256, &trimmed_interactions, value);

        parsed_interactions.insert(key, interaction);
    }

    parsed_interactions
}

pub fn parse_interaction_field(offsets: &U256, interactions: &str, field: u8) -> String {
    let (from_byte, to_byte) = get_offset_for_interaction(offsets, field);

    let interaction = &interactions[from_byte..to_byte];

    return interaction.to_string();
}

pub fn get_offset_for_interaction(offsets: &U256, field: u8) -> (usize, usize) {
    let from_byte = if field == 0 {
        U256::from(0)
    } else {
        (offsets >> ((field - 1) * 32)) & U256::from(32)
    };

    let to_byte = (offsets >> (field * 32)) & U256::from(32);

    return (from_byte.as_usize(), to_byte.as_usize());
}
