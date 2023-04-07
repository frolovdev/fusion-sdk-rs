use std::collections::BTreeMap;

use ethers::types::transaction::eip712::{Eip712, EIP712_DOMAIN_TYPE_HASH};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_json::json;
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray, Debug)]
pub struct LimitOrderV3Struct {
    salt: String,
    maker_asset: String,
    taker_asset: String,
    maker: String,
    receiver: String,
    allowed_sender: String,
    making_amount: String,
    taking_amount: String,
    offsets: String,
    interactions: String,
}

pub trait ToBtreeMap {
    fn to_btree_map(&self) -> BTreeMap<String, serde_json::Value>;
}

impl ToBtreeMap for LimitOrderV3Struct {
    fn to_btree_map(&self) -> BTreeMap<String, serde_json::Value> {
        let mut map = BTreeMap::new();

        map.insert("salt".to_string(), json!(self.salt));
        map.insert("makerAsset".to_string(), json!(self.maker_asset));
        map.insert("taker_asset".to_string(), json!(self.taker_asset));
        map.insert("maker".to_string(), json!(self.maker));
        map.insert("receiver".to_string(), json!(self.receiver));
        map.insert("allowed_sender".to_string(), json!(self.allowed_sender));
        map.insert("making_amount".to_string(), json!(self.making_amount));
        map.insert("taking_amount".to_string(), json!(self.taking_amount));
        map.insert("offsets".to_string(), json!(self.offsets));
        map.insert("interactions".to_string(), json!(self.interactions));

        if map.len() != LimitOrderV3Struct::FIELD_NAMES_AS_ARRAY.len() {
            panic!("Not all fields were serialized");
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let limit_order = LimitOrderV3Struct {
            salt: "kek".to_string(),
            maker_asset: "kek".to_string(),
            taker_asset: "kek".to_string(),
            maker: "kek".to_string(),
            receiver: "kek".to_string(),
            allowed_sender: "kek".to_string(),
            making_amount: "kek".to_string(),
            taking_amount: "kek".to_string(),
            offsets: "kek".to_string(),
            interactions: "kek".to_string(),
        };

        let map = limit_order.to_btree_map();

        let expectedMap = BTreeMap::from([
            ("salt".to_string(), json!("kek")),
            ("makerAsset".to_string(), json!("kek")),
            ("taker_asset".to_string(), json!("kek")),
            ("maker".to_string(), json!("kek")),
            ("receiver".to_string(), json!("kek")),
            ("allowed_sender".to_string(), json!("kek")),
            ("making_amount".to_string(), json!("kek")),
            ("taking_amount".to_string(), json!("kek")),
            ("offsets".to_string(), json!("kek")),
            ("interactions".to_string(), json!("kek")),
        ]);

        assert_eq!(map, expectedMap);
    }
}
