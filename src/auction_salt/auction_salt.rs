use rand::Rng;
use ruint::aliases::U256;
use std::str::FromStr;

use super::parser::{
    constants::salt_mask,
    parser::{get_duration, get_fee, get_initial_rate_bump, get_salt, get_start_time},
};

pub struct AuctionSalt {
    pub auction_start_time: u32,
    pub initial_rate_bump: u32,
    pub duration: u32,
    pub bank_fee: U256,
    pub salt: U256,
}

pub struct AuctionSaltData {
    auction_start_time: u32,
    initial_rate_bump: u32,
    duration: u32,
    bank_fee: U256,
    salt: Option<U256>,
}

trait AuctionSaltGeneratorTrait {
    fn generate(&self) -> U256;
}

struct AuctionSaltGeneratorRand {}

impl AuctionSaltGeneratorTrait for AuctionSaltGeneratorRand {
    fn generate(&self) -> U256 {
        let randInt = rand::thread_rng().gen_range(0..10000);

        U256::from(randInt)
    }
}

struct AuctionSaltGeneratorMock {}

impl AuctionSaltGeneratorTrait for AuctionSaltGeneratorMock {
    fn generate(&self) -> U256 {
        U256::from(1000)
    }
}

impl AuctionSalt {
    pub fn new(auction: AuctionSaltData, rng: Box<dyn AuctionSaltGeneratorTrait>) -> Self {
        let salt = if let Some(salt) = auction.salt {
            let salt_bn = salt;
            if salt_mask().lt(&salt_bn) {
                panic!("salt should be less than 18 bytes");
            }
            salt
        } else {
            rng.generate()
        };

        Self {
            salt,
            auction_start_time: auction.auction_start_time,
            initial_rate_bump: auction.initial_rate_bump,
            duration: auction.duration,
            bank_fee: auction.bank_fee,
        }
    }

    pub fn decode(salt: &str) -> Self {
        Self {
            salt: get_salt(salt),
            auction_start_time: get_start_time(salt).try_into().unwrap(),
            duration: get_duration(salt).try_into().unwrap(),
            bank_fee: get_fee(salt),
            initial_rate_bump: get_initial_rate_bump(salt).try_into().unwrap(),
        }
    }

    pub fn build(&self) -> String {
        let res = pad_start(&hex::encode(self.auction_start_time.to_be_bytes()), 8, '0')
          +  &pad_start(&hex::encode( self.duration.to_be_bytes()), 6, '0')
            + &pad_start(&hex::encode(self.initial_rate_bump.to_be_bytes()), 6, '0')
            + &pad_start(&hex::encode(self.bank_fee.to_be_bytes_vec()), 8, '0')
            + &pad_start(&hex::encode(&self.salt.to_be_bytes_vec()), 36, '0');

        assert_eq!(res.len(), 64, "Some inputs were out of allowed ranges");

        U256::from_str(&("0x".to_string() + &res))
            .unwrap()
            .to_string()
    }
}

pub fn pad_start(s: &str, width: usize, fill: char) -> String {
    if s.len() > width {
        return s[s.len() - width..s.len()].to_string();
    } else {
        let pad_len = width - s.len();
        let padded: String = std::iter::repeat(fill)
            .take(pad_len)
            .chain(s.chars())
            .collect();
        padded
    }

}
#[cfg(test)]
mod tests {
    use rand::Rng;
    use ruint::aliases::U256;
    use super::{AuctionSalt, AuctionSaltData, AuctionSaltGeneratorMock};

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
            Box::new(AuctionSaltGeneratorMock {}),
        );

        assert_eq!(
            salt.build(),
            "45118768841948961586167738353692277076075522015101619148498725069326976549864".to_string()
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
            Box::new(AuctionSaltGeneratorMock {}),
        );

        assert_eq!(
            salt.build(),
            "45118768841948961586167741099429671146420854337050268925130474518618971309032".to_string()
        )
    }
}
