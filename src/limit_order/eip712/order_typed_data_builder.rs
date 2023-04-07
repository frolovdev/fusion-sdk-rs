use ethers::abi::AbiEncode;
use ethers::types::transaction::eip712::{hash_struct, Eip712, TypedData};
use ethers::types::transaction::eip712::{EIP712Domain, Eip712DomainType};
use ethers::types::{H160, U256};
use serde_json::json;
use std::collections::BTreeMap;
use std::str::FromStr;

use crate::constants::ONE_INCH_ROUTER_V5;
use crate::limit_order::types::{LimitOrderV3Struct, ToBtreeMap};

use super::domain::{eip712_domain_type, order_type, LIMIT_ORDER_V3_TYPE_DATA_NAME, LIMIT_ORDER_V3_TYPE_DATA_VERSION};

pub fn build_order_data(
    chain_id: &U256,
    verifying_contract: &str,
    name: &str,
    version: &str,
    order: LimitOrderV3Struct,
) -> TypedData {
    let domain = EIP712Domain {
        chain_id: Some(chain_id.clone()),
        verifying_contract: Some(H160::from_str(verifying_contract).unwrap()),
        name: Some(name.to_string()),
        version: Some(version.to_string()),
        salt: None,
    };
    let mut types: BTreeMap<String, Vec<Eip712DomainType>> = BTreeMap::new();

    types.insert("EIP712Domain".to_string(), eip712_domain_type());
    types.insert("Order".to_string(), order_type());

    let message = order.to_btree_map();

    TypedData {
        primary_type: "Order".to_string(),
        types,
        domain,
        message,
    }
}

pub fn get_order_hash(data: TypedData) -> String {
    "0x".to_string() + &data.encode_eip712().unwrap().encode_hex()
}

pub fn domain_separator(
    name: &str,
    version: &str,
    chain_id: &U256,
    verifying_contract: &str,
) -> String {
    let mut types: BTreeMap<String, Vec<Eip712DomainType>> = BTreeMap::new();

    types.insert("EIP712Domain".to_string(), eip712_domain_type());

    "0x".to_string()
        + &hash_struct(
            "EIP712Domain",
            &json!({
                "name": name,
                "version": version,
                "chainId": chain_id,
                "verifyingContract": verifying_contract
            }),
            &types,
        )
        .unwrap()
        .encode_hex()
}


pub fn get_limit_order_v3_domain(chain_id: &U256) -> EIP712Domain {
    EIP712Domain {
        chain_id: Some(chain_id.clone()),
        verifying_contract: Some(H160::from_str(ONE_INCH_ROUTER_V5).unwrap()),
        name: Some(LIMIT_ORDER_V3_TYPE_DATA_NAME.to_string()),
        version: Some(LIMIT_ORDER_V3_TYPE_DATA_VERSION.to_string()),
        salt: None,
    }
}