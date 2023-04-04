use std::vec;

use ethers::types::U256;

use crate::{constants::ZERO_ADDRESS, utils::add_0x};

use super::constants::*;
use super::types::*;

pub fn parse_interactions_suffix(interactions: &str) -> InteractionAdditionalInfo {
    let flags = parse_flags(interactions);

    if interactions.len() < min_interactions_length(&flags) {
        panic!("Wrong interactions length")
    }

    let interactions_without_flags = &interactions[0..interactions.len() - FLAGS_LENGTH];

    let TakerFeeData {
        taker_fee_receiver,
        taker_fee_ratio,
        interactions: interactions_no_taking_fee,
    } = parse_taking_fee_and_return_remaining_interactions(&flags, interactions_without_flags);

    let PrivateAuctionDeadline {
        deadline,
        interactions: interactions_no_deadline,
    } = parse_private_auction_deadline(&interactions_no_taking_fee);

    let ResolverWhitelist {
        whitelist,
        interactions: interactions_no_whitelist,
    } = parse_resolver_white_list(&flags, &interactions_no_deadline);

    let ParsedAuctionParams { points, .. } =
        parse_auction_params(&flags, &interactions_no_whitelist);

    InteractionAdditionalInfo {
        whitelist,
        public_resolving_deadline: deadline,
        taker_fee_receiver,
        taker_fee_ratio,
        points,
    }
}

pub fn parse_taking_fee_and_return_remaining_interactions(
    flags: &InteractionFlags,
    interactions: &str,
) -> TakerFeeData {
    if !flags.taking_fee_enabled {
        return TakerFeeData {
            interactions: interactions.to_string(),
            taker_fee_receiver: ZERO_ADDRESS.to_string(),
            taker_fee_ratio: U256::from(0),
        };
    } else {
    }

    let taker_fee_data_len = TAKER_FEE_RECEIVER_LENGTH + TAKER_FEE_RATIO_LENGTH;

    let taker_fee_data =
        interactions[interactions.len() - taker_fee_data_len..interactions.len()].to_string();

    let taker_fee_receiver_hex = taker_fee_data[TAKER_FEE_RATIO_LENGTH..].to_string();

    let taker_fee_receiver = add_0x(&taker_fee_receiver_hex);

    if taker_fee_receiver == ZERO_ADDRESS {
        panic!("taker_fee_receiver cannot be zero address");
    }

    let taker_fee_ratio_hex = taker_fee_data[0..TAKER_FEE_RATIO_LENGTH].to_string();

    let taker_fee_ratio = U256::from_str_radix(&add_0x(&taker_fee_ratio_hex), 16).unwrap();

    if taker_fee_ratio > U256::from(CONTRACT_TAKER_FEE_PRECISION) {
        panic!("taker_fee_ratio cannot be greater than 100%");
    } else {
    }

    TakerFeeData {
        interactions: interactions[0..interactions.len() - taker_fee_data_len].to_string(),
        taker_fee_receiver,
        taker_fee_ratio,
    }
}

pub fn parse_private_auction_deadline(interactions: &str) -> PrivateAuctionDeadline {
    let private_auction_deadline_hex = interactions
        [interactions.len() - PRIVATE_AUCTION_DEADLINE_LENGTH..interactions.len()]
        .to_string();

    let private_auction_deadline = u64::from_str_radix(&private_auction_deadline_hex, 16)
        .expect("Invalid public resolving deadline in interactions");

    PrivateAuctionDeadline {
        deadline: private_auction_deadline,
        interactions: interactions[0..interactions.len() - PRIVATE_AUCTION_DEADLINE_LENGTH]
            .to_string(),
    }
}

pub fn parse_resolver_white_list(
    flags: &InteractionFlags,
    interactions: &str,
) -> ResolverWhitelist {
    let mut whitelist: Vec<AuctionWhitelistItem> = vec![];

    let allowed_ts_and_resolvers_len = ADDRESS_LENGTH + ALLOWED_TIMESTAMP_LENGTH;

    let addresses_packed = interactions[interactions.len()
        - (flags.resolvers_count as usize * allowed_ts_and_resolvers_len)
        ..interactions.len()]
        .to_string();

    if addresses_packed.len() % allowed_ts_and_resolvers_len != 0 {
        panic!("Invalid whitelist addresses in interactions");
    } else {
    }

    for i in (0..addresses_packed.len()).step_by(allowed_ts_and_resolvers_len) {
        let ts_and_address = addresses_packed[i..i + allowed_ts_and_resolvers_len].to_string();

        let timestamp_hex = ts_and_address[0..ALLOWED_TIMESTAMP_LENGTH].to_owned();

        let address = ts_and_address[ALLOWED_TIMESTAMP_LENGTH..].to_owned();

        let timestamp =
            u64::from_str_radix(&timestamp_hex, 16).expect("Invalid resolver allowance timestamp");

        whitelist.push(AuctionWhitelistItem {
            address: add_0x(&address).to_lowercase(),
            allowance: timestamp,
        });
    }

    ResolverWhitelist {
        whitelist,
        interactions: interactions[0..interactions.len()
            - (flags.resolvers_count as usize * allowed_ts_and_resolvers_len)]
            .to_string(),
    }
}

pub fn parse_auction_params(flags: &InteractionFlags, interactions: &str) -> ParsedAuctionParams {
    if flags.points_count == 0 {
        return ParsedAuctionParams {
            interactions: interactions.to_string(),
            points: vec![],
        };
    } else {
    }

    let mut points: Vec<AuctionPoint> = vec![];

    let auction_params_len = AUCTION_DELAY_LENGTH + AUCTION_BUMP_LENGTH;

    let params_packed = interactions[interactions.len()
        - (flags.points_count as usize * auction_params_len)
        ..interactions.len()]
        .to_string();

    if params_packed.len() % auction_params_len != 0 {
        panic!("Invalid auction params in interactions");
    } else {
    }

    for i in (0..params_packed.len()).step_by(auction_params_len) {
        let duration_and_bump = params_packed[i..i + auction_params_len].to_string();
        let duration_hex = duration_and_bump[0..AUCTION_DELAY_LENGTH].to_owned();

        let bump_hex = duration_and_bump[AUCTION_DELAY_LENGTH..].to_owned();

        let duration =
            u32::from_str_radix(&duration_hex, 16).expect("Invalid auction point duration");

        let bump = u32::from_str_radix(&bump_hex, 16).expect("Invalid auction point bump");

        points.push(AuctionPoint {
            delay: duration,
            coefficient: bump,
        });
    }

    ParsedAuctionParams {
        interactions: interactions[0..flags.points_count as usize * auction_params_len].to_string(),
        points,
    }
}

pub fn parse_flags(interactions: &str) -> InteractionFlags {
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

pub fn min_interactions_length(flags: &InteractionFlags) -> usize {
    let auction_points_len =
        flags.points_count as usize * (AUCTION_DELAY_LENGTH + AUCTION_BUMP_LENGTH);
    let whitelist_len =
        flags.resolvers_count as usize * (ALLOWED_TIMESTAMP_LENGTH + ADDRESS_LENGTH);

    let required_length =
        auction_points_len + whitelist_len + PRIVATE_AUCTION_DEADLINE_LENGTH + FLAGS_LENGTH;

    if flags.taking_fee_enabled {
        return required_length + TAKER_FEE_RECEIVER_LENGTH + TAKER_FEE_RATIO_LENGTH;
    }

    required_length
}
