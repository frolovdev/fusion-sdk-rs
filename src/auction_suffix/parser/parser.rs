use std::vec;

use ethers::types::U256;

use crate::{constants::ZERO_ADDRESS, utils::add_0x};

use super::constants::*;

pub fn parse_interactions_suffix() {}

struct InteractionFlags {
    taking_fee_enabled: bool,
    resolvers_count: u8,
    points_count: u8,
}



struct  TakerFeeData {
    taker_fee_ratio: U256,
    taker_fee_receiver: String,
    interactions: String,
}

struct  PrivateAuctionDeadline {
    deadline: u32,
    interactions: String,
}

fn parse_taking_fee_and_return_remaining_interactions(flags: InteractionFlags, interactions: &str) -> TakerFeeData {
    if !flags.taking_fee_enabled {
        return TakerFeeData {
            interactions: interactions.to_string(),
            taker_fee_receiver: ZERO_ADDRESS.to_string(),
            taker_fee_ratio: U256::from(0),
        };
    } else {}

    let taker_fee_data_len = TAKER_FEE_RECEIVER_LENGTH + TAKER_FEE_RATIO_LENGTH;


    let taker_fee_data = interactions[interactions.len() - taker_fee_data_len..interactions.len()].to_string();

    let taker_fee_receiver_hex = taker_fee_data[TAKER_FEE_RATIO_LENGTH..].to_string();

    let taker_fee_receiver = add_0x(&taker_fee_receiver_hex);

    if taker_fee_receiver == ZERO_ADDRESS {
        panic!("taker_fee_receiver cannot be zero address");
    }

    let taker_fee_ratio_hex = taker_fee_data [0..TAKER_FEE_RATIO_LENGTH].to_string();

    let taker_fee_ratio = U256::from_str_radix(&add_0x(&taker_fee_ratio_hex), 16).unwrap();

    if taker_fee_ratio > U256::from(CONTRACT_TAKER_FEE_PRECISION) {
        panic!("taker_fee_ratio cannot be greater than 100%");
    } else {}

    TakerFeeData {
        interactions: interactions[0..interactions.len() - taker_fee_data_len].to_string(),
        taker_fee_receiver,
        taker_fee_ratio
    }
}

fn parse_private_auction_deadline(interactions: &str) -> PrivateAuctionDeadline {
    let private_auction_deadline_hex = interactions[interactions.len() - PRIVATE_AUCTION_DEADLINE_LENGTH..interactions.len()].to_string();

    let private_auction_deadline = u32::from_str_radix(&private_auction_deadline_hex, 16).expect("Invalid public resolving deadline in interactions");

    PrivateAuctionDeadline { deadline: private_auction_deadline, interactions: interactions[0..interactions.len() - PRIVATE_AUCTION_DEADLINE_LENGTH].to_string() }
}

struct  AuctionWhitelistItem {
    address: String,
    allowance: u32,
}

struct  ResolverWhitelist {
    whitelist: Vec<AuctionWhitelistItem>,
    interactions: String,
}

fn parse_resolver_white_list(flags: InteractionFlags, interactions: &str) -> ResolverWhitelist {
    let mut whitelist: Vec<AuctionWhitelistItem> = vec![];

    let allowed_ts_and_resolvers_len = ADDRESS_LENGTH + ALLOWED_TIMESTAMP_LENGTH;

    let addresses_packed = interactions[interactions.len() - (flags.resolvers_count as usize * allowed_ts_and_resolvers_len)..interactions.len()].to_string();

    if addresses_packed.len() % allowed_ts_and_resolvers_len == 0 {
        panic!("Invalid whitelist addresses in interactions");
    } else {

    }

    for i in (0..addresses_packed.len()).step_by(allowed_ts_and_resolvers_len) {
        let ts_and_address = addresses_packed[i..i+allowed_ts_and_resolvers_len].to_string();

        let timestamp_hex = ts_and_address[0..ALLOWED_TIMESTAMP_LENGTH].to_owned();

        let address = ts_and_address[ALLOWED_TIMESTAMP_LENGTH..].to_owned();

        let timestamp = u32::from_str_radix(&timestamp_hex, 16).expect("Invalid resolver allowance timestamp");

        whitelist.push(AuctionWhitelistItem {
            address: add_0x(&address).to_lowercase(),
            allowance: timestamp,
        });
    }


    ResolverWhitelist {
        whitelist,
        interactions: interactions[0..interactions.len() - (flags.resolvers_count as usize * allowed_ts_and_resolvers_len)].to_string(),
    }


}

struct AuctionPoint {
    delay: u32,
    coefficient: u32,
}

struct  ParsedAuctionParams {
    interactions: String,
    points: Vec<AuctionPoint>,
}

fn parse_auction_params(flags: InteractionFlags, interactions: &str) -> ParsedAuctionParams {
    if flags.points_count == 0 {
        return ParsedAuctionParams {
            interactions: interactions.to_string(),
            points: vec![],
        };
    } else {}

    let mut points: Vec<AuctionPoint> = vec![];

    let auction_params_len = AUCTION_DELAY_LENGTH + AUCTION_BUMP_LENGTH;

    let params_packed

}

fn parse_flags(interactions: &str) -> InteractionFlags {
    let flags_hex = interactions[interactions.len() - FLAGS_LENGTH..interactions.len()].to_string();

    if flags_hex.len() != FLAGS_LENGTH {
        panic!("Invalid flags length");
    }

    let flags = u8::from_str_radix(&flags_hex, 16).expect("Cannot parse flags");

    let resolvers_count = (flags & RESOLVERS_LENGTH_MASK) >> RESOLVERS_LENGTH_OFFSET;

    if resolvers_count == 0 {
        panic!("Cannot have 0 resolvers");
    }

    let taking_fee_enabled = (flags & HAS_TAKING_FEE_FLAG) != 0;

    let points_count = flags & POINTS_LENGTH_MASK;

    InteractionFlags {
        taking_fee_enabled,
        resolvers_count,
        points_count,
    }
}
