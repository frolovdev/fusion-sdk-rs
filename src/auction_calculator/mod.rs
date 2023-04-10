pub mod calc;
pub mod constants;
pub mod types;

use crate::{
    auction_salt::types::AuctionSalt,
    auction_suffix::{
        parser::{constants::CONTRACT_TAKER_FEE_PRECISION, types::AuctionPoint},
        types::AuctionSuffix,
    },
    limit_order::types::LimitOrderV3Struct,
};
use calc::linear_interpolation;
use constants::RATE_BUMP_DENOMINATOR;
use ethers::types::U256;
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
            taker_fee_ratio: suffix.taker_fee_ratio,
        }
    }

    pub fn calc_auction_taking_amount(&self, taking_amount: &U256, rate: u32) -> U256 {
        let auction_taking_amount = (taking_amount
            * (U256::from(rate) + U256::from(RATE_BUMP_DENOMINATOR)))
            / U256::from(RATE_BUMP_DENOMINATOR);

        if self.taker_fee_ratio == U256::zero() {
            auction_taking_amount
        } else {
            auction_taking_amount
                + (auction_taking_amount * self.taker_fee_ratio / CONTRACT_TAKER_FEE_PRECISION)
        }
    }

    // https://github.com/1inch/limit-order-settlement/blob/3c7cf9eacbaf7a60624d7a6f069c59d809f2204a/contracts/libraries/OrderSuffix.sol#L75
    pub fn calc_rate_bump(&self, time: u32) -> u32 {
        let mut cumulative_time = U256::from(self.start_time);
        let last_time = U256::from(self.duration) + self.start_time;

        let start_bump = U256::from(self.initial_rate_bump);

        let current_time = U256::from(time);

        if current_time <= cumulative_time {
            return self.initial_rate_bump;
        } else if current_time >= last_time {
            return 0;
        } else {
            let mut prev_coefficient = start_bump;
            let mut prev_cumulative_time = cumulative_time;

            for i in (0..self.points.len()).rev() {
                let AuctionPoint { coefficient, delay } = self.points[i];

                cumulative_time += U256::from(delay);
                let current_coefficient = U256::from(coefficient);

                if cumulative_time > current_time {
                    let rate: u32 = linear_interpolation(
                        prev_cumulative_time,
                        cumulative_time,
                        prev_coefficient,
                        current_coefficient,
                        current_time,
                    )
                    .try_into()
                    .unwrap();

                    return rate;
                }

                prev_cumulative_time = cumulative_time;
                prev_coefficient = current_coefficient;
            }

            let rate: u32 = linear_interpolation(
                prev_cumulative_time,
                last_time,
                prev_coefficient,
                U256::zero(),
                current_time,
            )
            .try_into()
            .unwrap();

            rate
        }
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use ethers::types::Address;
    use ethers::types::Bytes;
    use ethers::types::U256;
    use pretty_assertions::assert_eq;

    use crate::auction_suffix::types::AuctionSuffix;
    use crate::limit_order::types::LimitOrderV3Struct;

    use super::types::AuctionCalculator;
    #[test]
    fn should_calculate_auction_rate_and_taking_amount() {
        let calculator = AuctionCalculator::from_limit_order_v3_struct(LimitOrderV3Struct {
            allowed_sender: Address::from_str("0x0000000000000000000000000000000000000000")
                .unwrap(),
            interactions: Bytes::from_str(
                "0x000c004e200000000000000000219ab540356cbb839cbe05303d7705faf486570009",
            )
            .unwrap(),
            maker: Address::from_str("0x00000000219ab540356cbb839cbe05303d7705fa").unwrap(),
            maker_asset: Address::from_str("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2").unwrap(),
            making_amount: U256::from(1000000000000000000 as u64),
            offsets: U256::zero(),
            receiver: Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
            salt: U256::from_dec_str(
                "45118768841948961586167738353692277076075522015101619148498725069326976558864",
            )
            .unwrap(),
            taker_asset: Address::from_str("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap(),
            taking_amount: U256::from(1420000000),
        });

        let rate = calculator.calc_rate_bump(1673548209);

        let auction_taking_amount =
            calculator.calc_auction_taking_amount(&U256::from(1420000000), rate);

        assert_eq!(rate, 14285);
        assert_eq!(auction_taking_amount, U256::from(1422028470));
    }

    #[test]
    fn should_be_created_successfully_from_suffix_and_salt() {
        let suffix = AuctionSuffix::decode(
            &Bytes::from_str(
                "0x000c004e200000000000000000219ab540356cbb839cbe05303d7705faf486570009",
            )
            .unwrap(),
        );
        let salt = crate::auction_salt::types::AuctionSalt::decode(
            &U256::from_dec_str(
                "45118768841948961586167738353692277076075522015101619148498725069326976558864",
            )
            .unwrap(),
        );

        let calculator = AuctionCalculator::from_auction_data(&suffix, &salt);

        let rate = calculator.calc_rate_bump(1673548209);

        let auction_taking_amount =
            calculator.calc_auction_taking_amount(&U256::from(1420000000), rate);

        assert_eq!(rate, 14285);
        assert_eq!(auction_taking_amount, U256::from(1422028470));
    }
}
