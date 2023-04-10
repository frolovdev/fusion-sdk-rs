pub mod constants;
pub mod types;

use crate::{auction_suffix::{types::AuctionSuffix}, limit_order::types::LimitOrderV3Struct, auction_salt::types::AuctionSalt};

use types::AuctionCalculator;

impl AuctionCalculator {
    pub fn from_limit_order_v3_struct(order: LimitOrderV3Struct) -> Self {
        let suffix = AuctionSuffix::decode(&order.interactions);
        let salt = AuctionSalt::decode(&order.salt);

        AuctionCalculator::from_auction_data(&suffix, &salt)
    }

    pub fn from_auction_data(suffix: &AuctionSuffix, salt: &AuctionSalt) -> Self {
        Self {
            start_time: salt.auction_start_time,
            duration: salt.duration,
            initial_rate_bump: salt.initial_rate_bump,
            points: suffix.points.clone(),
            taker_fee_ratio: suffix.taker_fee_ratio
        }
    }
}
