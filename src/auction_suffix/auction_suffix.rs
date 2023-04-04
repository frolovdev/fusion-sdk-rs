use crate::{
    auction_suffix::encoder::{
        encode_auction_params, encode_flags, encode_public_resolving_deadline,
        encode_taking_fee_data, encode_whitelist,
    },
    constants::{zero_number, ZERO_ADDRESS},
    utils::to_sec,
};

use super::{
    constants::NO_PUBLIC_RESOLVING_DEADLINE,
    parser::parser::parse_interactions_suffix,
    types::{AuctionSuffix, SettlementSuffixData, TakingFee},
};

impl AuctionSuffix {
    pub fn new(suffix: SettlementSuffixData) -> Self {
        AuctionSuffix {
            points: suffix.points,
            whitelist: suffix.whitelist,
            public_resolving_deadline: suffix
                .public_resolving_deadline
                .unwrap_or_else(|| to_sec(NO_PUBLIC_RESOLVING_DEADLINE)),
            taker_fee_receiver: suffix.fee.as_ref().map_or_else(
                || ZERO_ADDRESS.to_string(),
                |f| f.taking_fee_receiver.to_owned(),
            ),
            taker_fee_ratio: suffix
                .fee
                .as_ref()
                .map_or_else(|| zero_number(), |f| f.taking_fee_ratio),
        }
    }

    pub fn decode(interactions: &str) -> Self {
        let suffix = parse_interactions_suffix(interactions);

        AuctionSuffix::new(SettlementSuffixData {
            public_resolving_deadline: Some(suffix.public_resolving_deadline),
            points: suffix.points,
            fee: Some(TakingFee {
                taking_fee_receiver: suffix.taker_fee_receiver,
                taking_fee_ratio: suffix.taker_fee_ratio,
            }),
            whitelist: suffix.whitelist,
        })
    }

    pub fn build(&self) -> String {
        let auction_params = encode_auction_params(&self.points);
        let whitelist = encode_whitelist(&self.whitelist);
        let public_resolving = encode_public_resolving_deadline(self.public_resolving_deadline);
        let taking_fee_data =
            encode_taking_fee_data(&self.taker_fee_receiver, &self.taker_fee_ratio);
        let flags = encode_flags(&self.whitelist, &self.points, &taking_fee_data);

        format!(
            "{}{}{}{}{}",
            auction_params, whitelist, public_resolving, taking_fee_data, flags
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::auction_suffix::parser::types::{AuctionPoint, AuctionWhitelistItem};

    use super::{AuctionSuffix, SettlementSuffixData};
    use pretty_assertions::assert_eq;
    #[test]
    fn should_create_suffix_with_required_params() {
        let suffix = AuctionSuffix::new(SettlementSuffixData {
            points: vec![AuctionPoint {
                coefficient: 20000,
                delay: 12,
            }],
            whitelist: vec![AuctionWhitelistItem {
                address: "0x00000000219ab540356cbb839cbe05303d7705fa".to_string(),
                allowance: 0,
            }],
            public_resolving_deadline: None,
            fee: None,
        });

        assert_eq!(
            suffix.build(),
            "000c004e200000000000000000219ab540356cbb839cbe05303d7705faf486570009"
        )
    }

    #[test]
    fn should_create_suffix_with_specified_public_resolving_deadline() {
        let suffix = AuctionSuffix::new(SettlementSuffixData {
            points: vec![AuctionPoint {
                coefficient: 20000,
                delay: 12,
            }],
            whitelist: vec![AuctionWhitelistItem {
                address: "0x00000000219ab540356cbb839cbe05303d7705fa".to_string(),
                allowance: 0,
            }],
            public_resolving_deadline: Some(1673549418),
            fee: None,
        });

        assert_eq!(
            suffix.build(),
            "000c004e200000000000000000219ab540356cbb839cbe05303d7705fa63c0566a09"
        );
    }

    #[test]
    fn should_decode_auction_suffix() {
        let encoded_suffix = "000c004e200000000000000000219ab540356cbb839cbe05303d7705fa63c0566a09";

        let suffix = AuctionSuffix::decode(encoded_suffix);

        assert_eq!(suffix.build(), encoded_suffix);
    }
}
