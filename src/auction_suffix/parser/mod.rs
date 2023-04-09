pub mod constants;
pub mod types;

use std::vec;

use ethers::types::{Address, Bytes, U256};

use crate::constants::ZERO_ADDRESS;

use constants::*;
use types::*;

pub fn parse_interactions_suffix(interactions: &Bytes) -> InteractionAdditionalInfo {
    let flags = parse_flags(interactions);

    println!("flags: {:?}", flags);
    if interactions.len() < min_interactions_length(&flags) {
        panic!("Wrong interactions length")
    }

    let interactions_without_flags = &interactions[0..interactions.len() - FLAGS_LENGTH];

    let TakerFeeData {
        taker_fee_receiver,
        taker_fee_ratio,
        interactions: interactions_no_taking_fee,
    } = parse_taking_fee_and_return_remaining_interactions(&flags, interactions_without_flags);

    println!("taker_fee_receiver: {:?}", taker_fee_receiver);
    println!("taker_fee_ratio: {}", taker_fee_ratio);
    println!(
        "interactions_no_taking_fee: {:?}",
        &interactions_no_taking_fee
    );

    let PrivateAuctionDeadline {
        deadline,
        interactions: interactions_no_deadline,
    } = parse_private_auction_deadline(&interactions_no_taking_fee);

    println!("deadline: {}", deadline);
    println!("interactions_no_deadline: {:?}", &interactions_no_deadline);

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
    interactions: &[u8],
) -> TakerFeeData {
    if !flags.taking_fee_enabled {
        return TakerFeeData {
            interactions: Bytes::from(interactions.to_vec()),
            taker_fee_receiver: ZERO_ADDRESS,
            taker_fee_ratio: U256::from(0),
        };
    } else {
    }

    let taker_fee_data_len = TAKER_FEE_RECEIVER_LENGTH + TAKER_FEE_RATIO_LENGTH;

    let taker_fee_data = &interactions[interactions.len() - taker_fee_data_len..interactions.len()];

    let taker_fee_receiver_bytes: &[u8] = &taker_fee_data[TAKER_FEE_RATIO_LENGTH..];

    let taker_fee_receiver = Address::from_slice(taker_fee_receiver_bytes);

    if taker_fee_receiver == ZERO_ADDRESS {
        panic!("taker_fee_receiver cannot be zero address");
    }

    let taker_fee_ratio_bytes = &taker_fee_data[0..TAKER_FEE_RATIO_LENGTH];

    let taker_fee_ratio = U256::from(taker_fee_ratio_bytes);

    if taker_fee_ratio > U256::from(CONTRACT_TAKER_FEE_PRECISION) {
        panic!("taker_fee_ratio cannot be greater than 100%");
    } else {
    }

    TakerFeeData {
        interactions: Bytes::from(
            interactions[0..interactions.len() - taker_fee_data_len].to_owned(),
        ),
        taker_fee_receiver,
        taker_fee_ratio,
    }
}

pub fn parse_private_auction_deadline(interactions: &[u8]) -> PrivateAuctionDeadline {
    let private_auction_deadline_bytes =
        &interactions[interactions.len() - PRIVATE_AUCTION_DEADLINE_LENGTH..interactions.len()];

    let private_auction_deadline = U256::from(private_auction_deadline_bytes);

    println!("private_auction_deadline: {:?}", &private_auction_deadline);

    PrivateAuctionDeadline {
        deadline: private_auction_deadline,
        interactions: Bytes::from(
            interactions[0..interactions.len() - PRIVATE_AUCTION_DEADLINE_LENGTH].to_vec(),
        ),
    }
}

pub fn parse_resolver_white_list(
    flags: &InteractionFlags,
    interactions: &[u8],
) -> ResolverWhitelist {
    let mut whitelist: Vec<AuctionWhitelistItem> = vec![];

    let allowed_ts_and_resolvers_len = ADDRESS_LENGTH + ALLOWED_TIMESTAMP_LENGTH;

    let addresses_packed = &interactions[interactions.len()
        - (flags.resolvers_count * allowed_ts_and_resolvers_len)
        ..interactions.len()];

    if addresses_packed.len() % allowed_ts_and_resolvers_len != 0 {
        panic!("Invalid whitelist addresses in interactions");
    } else {
    }

    for i in (0..addresses_packed.len()).step_by(allowed_ts_and_resolvers_len) {
        let ts_and_address = &addresses_packed[i..i + allowed_ts_and_resolvers_len];

        let timestamp_bytes = &ts_and_address[0..ALLOWED_TIMESTAMP_LENGTH];

        let address = ts_and_address[ALLOWED_TIMESTAMP_LENGTH..].to_owned();

        let timestamp: u32 = U256::from(timestamp_bytes).try_into().unwrap();

        whitelist.push(AuctionWhitelistItem {
            address: Address::from_slice(&address),
            allowance: timestamp,
        });
    }

    ResolverWhitelist {
        whitelist,
        interactions: Bytes::from(
            interactions
                [0..interactions.len() - (flags.resolvers_count * allowed_ts_and_resolvers_len)]
                .to_vec(),
        ),
    }
}

pub fn parse_auction_params(flags: &InteractionFlags, interactions: &[u8]) -> ParsedAuctionParams {
    if flags.points_count == 0 {
        return ParsedAuctionParams {
            interactions: Bytes::from(interactions.to_vec()),
            points: vec![],
        };
    } else {
    }

    let mut points: Vec<AuctionPoint> = vec![];

    let auction_params_len = AUCTION_DELAY_LENGTH + AUCTION_BUMP_LENGTH;

    let params_packed = &interactions
        [interactions.len() - (flags.points_count * auction_params_len)..interactions.len()];

    if params_packed.len() % auction_params_len != 0 {
        panic!("Invalid auction params in interactions");
    } else {
    }

    for i in (0..params_packed.len()).step_by(auction_params_len) {
        let duration_and_bump = &params_packed[i..i + auction_params_len];
        let duration_bytes = &duration_and_bump[0..AUCTION_DELAY_LENGTH];

        let bump_bytes = &duration_and_bump[AUCTION_DELAY_LENGTH..];

        let duration: u16 = U256::from(duration_bytes).try_into().unwrap();

        let bump: u32 = U256::from(bump_bytes).try_into().unwrap();

        points.push(AuctionPoint {
            delay: duration,
            coefficient: bump,
        });
    }

    ParsedAuctionParams {
        interactions: Bytes::from(
            interactions[0..flags.points_count * auction_params_len].to_vec(),
        ),
        points,
    }
}

pub fn parse_flags(interactions: &Bytes) -> InteractionFlags {
    let flags_bytes = &interactions[interactions.len() - 1..interactions.len()];

    if flags_bytes.len() != 1 {
        panic!("Invalid flags length");
    }

    let flags = U256::from(flags_bytes).as_usize();

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
    let auction_points_len = flags.points_count * (AUCTION_DELAY_LENGTH + AUCTION_BUMP_LENGTH);
    let whitelist_len = flags.resolvers_count * (ALLOWED_TIMESTAMP_LENGTH + ADDRESS_LENGTH);

    let required_length =
        auction_points_len + whitelist_len + PRIVATE_AUCTION_DEADLINE_LENGTH + FLAGS_LENGTH;

    if flags.taking_fee_enabled {
        return required_length + TAKER_FEE_RECEIVER_LENGTH + TAKER_FEE_RATIO_LENGTH;
    }

    required_length
}
