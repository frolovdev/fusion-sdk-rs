pub const AUCTION_BUMP_LENGTH: usize = 6;
pub const AUCTION_DELAY_LENGTH: usize = 4;

pub const ADDRESS_LENGTH: usize = 40;
pub const ALLOWED_TIMESTAMP_LENGTH: usize = 8;

pub const PRIVATE_AUCTION_DEADLINE_LENGTH: usize = 8;

pub const TAKER_FEE_RECEIVER_LENGTH: usize = 40;
pub const TAKER_FEE_RATIO_LENGTH: usize = 24;

pub const FLAGS_LENGTH: usize = 2;

pub const HAS_TAKING_FEE_FLAG: u8 = 0x80;
pub const RESOLVERS_LENGTH_MASK: u8 = 0x78;
pub const RESOLVERS_LENGTH_OFFSET: u8 = 3;
pub const POINTS_LENGTH_MASK: u8 = 0x07;

pub const CONTRACT_TAKER_FEE_PRECISION: u64 = 1_000_000_000;
