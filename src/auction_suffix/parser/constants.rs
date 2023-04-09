pub const AUCTION_BUMP_LENGTH: usize = 3; // _AUCTION_POINT_BUMP_BYTES_SIZE
pub const AUCTION_DELAY_LENGTH: usize = 2; // _AUCTION_POINT_DELAY_BYTES_SIZE

pub const ADDRESS_LENGTH: usize = 20; // _RESOLVER_ADDRESS_BYTES_SIZE
pub const ALLOWED_TIMESTAMP_LENGTH: usize = 4; // _RESOLVER_TIME_LIMIT_BYTES_SIZE

pub const PRIVATE_AUCTION_DEADLINE_LENGTH: usize = 4;

pub const TAKER_FEE_RECEIVER_LENGTH: usize = 20; // address
pub const TAKER_FEE_RATIO_LENGTH: usize = 12; // _TAKING_FEE_RATIO_OFFSET

pub const FLAGS_LENGTH: usize = 1;

pub const HAS_TAKING_FEE_FLAG: usize = 0x80; // _HAS_TAKING_FEE_FLAG
pub const RESOLVERS_LENGTH_MASK: usize = 0x78; // _RESOLVERS_LENGTH_MASK
pub const RESOLVERS_LENGTH_OFFSET: usize = 3; // _RESOLVERS_LENGTH_BIT_SHIFT
pub const POINTS_LENGTH_MASK: usize = 0x07; // _POINTS_LENGTH_MASK

pub const CONTRACT_TAKER_FEE_PRECISION: u64 = 1_000_000_000;
