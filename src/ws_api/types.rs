use ethers::{
    abi::Hash,
    types::{Signature, U256},
};

use crate::limit_order::types::LimitOrderV3Struct;

#[derive(Debug, Clone, PartialEq)]
pub enum RpcMessage {
    OrderEvent(OrderEvent),
    // SystemEvent(None),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderEvent {
    Created(OrderEventCreated),
    Invalid {
        order_hash: Hash,
    },
    BalanceOrAllowanceChange {
        order_hash: Hash,
        remaining_maker_amount: U256,
        balance: U256,
        allowance: u32,
    },
    Filled {
        order_hash: Hash,
    },
    FilledPartially {
        order_hash: Hash,
        remaining_maker_amount: U256,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderEventCreated {
    pub order_hash: Hash,
    pub signature: Signature,
    pub order: LimitOrderV3Struct,
    pub deadline: u32,
    pub auction_start_date: u32,
    pub auction_end_date: u32,
    pub remaining_maker_amount: U256,
}
