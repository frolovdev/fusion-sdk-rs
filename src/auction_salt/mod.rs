pub mod parser;
pub mod types;

use crate::{constants::ZX, salt::build_salt, utils::PadStart};
use ethers::{abi::AbiEncode, types::U256};
use parser::{constants::salt_mask, *};
use std::{borrow::Borrow, str::FromStr};
use types::{AuctionSalt, AuctionSaltData};

impl AuctionSalt {
    pub fn new<F>(auction: AuctionSaltData, salt_generator: Option<F>) -> Self
    where
        F: Fn() -> U256,
    {
        let salt = if let Some(salt) = auction.salt {
            if salt_mask().lt(&salt) {
                panic!("salt should be less than 18 bytes");
            }
            salt
        } else {
            if let Some(salt_generator) = salt_generator {
                salt_generator()
            } else {
                build_salt()
            }
        };

        Self {
            salt,
            auction_start_time: auction.auction_start_time,
            initial_rate_bump: auction.initial_rate_bump,
            duration: auction.duration,
            bank_fee: auction.bank_fee,
        }
    }

    pub fn decode(salt: &U256) -> Self {
        Self {
            salt: get_salt(salt),
            auction_start_time: get_start_time(salt).try_into().unwrap(),
            duration: get_duration(salt).try_into().unwrap(),
            bank_fee: get_fee(salt),
            initial_rate_bump: get_initial_rate_bump(salt).try_into().unwrap(),
        }
    }

    pub fn build(&self) -> String {
        assert_eq!(
            self.duration < (2 as u32).pow(24),
            true,
            "duration is too big, should be less than 2^24"
        );
        assert_eq!(
            self.initial_rate_bump < (2 as u32).pow(24),
            true,
            "initial_rate_bump is too big, should be less than 2^24"
        );

        let res = self.auction_start_time.encode_hex().pad_start(8, '0')
            + self.duration.encode_hex().pad_start(6, '0').borrow()
            + self
                .initial_rate_bump
                .encode_hex()
                .pad_start(6, '0')
                .borrow()
            + self.bank_fee.encode_hex().pad_start(8, '0').borrow()
            + self.salt.encode_hex().pad_start(36, '0').borrow();

        U256::from_str(&(ZX.to_string() + &res))
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::auction_salt::types::AuctionSalt;

    use super::AuctionSaltData;
    use ethers::core::types::U256;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_create_salt() {
        let salt = AuctionSalt::new(
            AuctionSaltData {
                auction_start_time: 1673548149,
                initial_rate_bump: 50000,
                duration: 180,
                bank_fee: U256::from(0),
                salt: None,
            },
            Some(|| U256::from(1000)),
        );

        assert_eq!(
            salt.build(),
            "45118768841948961586167738353692277076075522015101619148498725069326976549864"
                .to_string()
        )
    }

    #[test]
    fn should_create_salt_with_non_zero_bank_fee() {
        let salt = AuctionSalt::new(
            AuctionSaltData {
                auction_start_time: 1673548149,
                initial_rate_bump: 50000,
                duration: 180,
                bank_fee: U256::from(123123123),
                salt: None,
            },
            Some(|| U256::from(1000)),
        );

        assert_eq!(
            salt.build(),
            "45118768841948961586167741099429671146420854337050268925130474518618971309032"
                .to_string()
        )
    }

    #[test]
    #[should_panic(expected = "initial_rate_bump is too big, should be less than 2^24")]
    fn should_fail_to_create_salt_due_to_initial_rate_bump_out_of_range() {
        let salt = AuctionSalt::new(
            AuctionSaltData {
                auction_start_time: 1673548149,
                initial_rate_bump: 16_777_215 + 1,
                duration: 180,
                bank_fee: U256::from(123123123),
                salt: None,
            },
            Some(|| U256::from(1000)),
        );

        salt.build();
    }

    #[test]
    #[should_panic(expected = "duration is too big, should be less than 2^24")]
    fn should_fail_to_create_salt_due_to_duration_out_of_range() {
        let salt = AuctionSalt::new(
            AuctionSaltData {
                auction_start_time: 1673548149,
                initial_rate_bump: 50000,
                duration: 16777215 + 1,
                bank_fee: U256::from(123123123),
                salt: None,
            },
            Some(|| U256::from(1000)),
        );

        salt.build();
    }

    #[test]
    fn should_decode_salt() {
        let encoded_salt = U256::from_dec_str(
            "45118768841948961586167741099429671146420854337050268925130474518618971309032",
        )
        .unwrap();

        let salt = AuctionSalt::decode(&encoded_salt);

        assert_eq!(salt.build(), encoded_salt.to_string());
    }
}
