use serde_json::json;
use std::collections::BTreeMap;
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray, Debug, PartialEq)]
pub struct LimitOrderV3Struct {
    pub salt: String,
    pub maker_asset: String,
    pub taker_asset: String,
    pub maker: String,
    pub receiver: String,
    pub allowed_sender: String,
    pub making_amount: String,
    pub taking_amount: String,
    pub offsets: String,
    pub interactions: String,
}

pub trait ToBtreeMap {
    fn to_btree_map(&self) -> BTreeMap<String, serde_json::Value>;
}

impl ToBtreeMap for LimitOrderV3Struct {
    fn to_btree_map(&self) -> BTreeMap<String, serde_json::Value> {
        let mut map = BTreeMap::new();

        map.insert("salt".to_string(), json!(self.salt));
        map.insert("makerAsset".to_string(), json!(self.maker_asset));
        map.insert("takerAsset".to_string(), json!(self.taker_asset));
        map.insert("maker".to_string(), json!(self.maker));
        map.insert("receiver".to_string(), json!(self.receiver));
        map.insert("allowedSender".to_string(), json!(self.allowed_sender));
        map.insert("makingAmount".to_string(), json!(self.making_amount));
        map.insert("takingAmount".to_string(), json!(self.taking_amount));
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

        let expected_map = BTreeMap::from([
            ("salt".to_string(), json!("kek")),
            ("makerAsset".to_string(), json!("kek")),
            ("takerAsset".to_string(), json!("kek")),
            ("maker".to_string(), json!("kek")),
            ("receiver".to_string(), json!("kek")),
            ("allowedSender".to_string(), json!("kek")),
            ("makingAmount".to_string(), json!("kek")),
            ("takingAmount".to_string(), json!("kek")),
            ("offsets".to_string(), json!("kek")),
            ("interactions".to_string(), json!("kek")),
        ]);

        assert_eq!(map, expected_map);
    }
}
