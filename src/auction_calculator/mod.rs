pub mod constants;
pub mod types;

use ethers::types::U256;

use crate::auction_suffix::parser::types::AuctionPoint;

use types::AuctionCalculator;

// impl AuctionCalculator {
//     pub fn from_limit_order_v3_struct(order: LimitOrderV3Struct) -> Self {
//         let suffix = AuctionSuffix::decode(&order.interactions);
//         let salt = AuctionSalt::decode(&order.salt);

//         Self::from_auction_data(suffix, salt)
//     }
// }
