pub mod eip712;
pub mod parser;
pub mod types;

pub struct LimitOrder {
    maker_asset: String,
    taker_asset: String,
    making_amount: String,
    taking_amount: String,
    from: String,
    allowed_sender: String,
    receiver: String,
    maker_asset_data: String,
    taker_asset_data: String,
    get_making_amount: String,
    get_taking_amount: String,
    predicate: String,
    permit: String,
    pre_interaction: String,
    post_interaction: String,
    salt: String,
}

pub struct InteractionsData {
    pub maker_asset_data: Option<String>,
    pub taker_asset_data: Option<String>,
    pub get_making_amount: Option<String>,
    pub get_taking_amount: Option<String>,
    pub predicate: Option<String>,
    pub permit: Option<String>,
    pub pre_interaction: Option<String>,
    pub post_interaction: Option<String>,
}

pub struct OrderInfoData {
    maker_asset: String,
    taker_asset: String,
    making_amount: String,
    taking_amount: String,
    maker: String,
    salt: Option<String>,
    allowed_sender: Option<String>,
    receiver: Option<String>,
}

impl LimitOrder {
    pub fn new(order_info: OrderInfoData, interactions: Option<InteractionsData>) -> Self {
        let interactions = interactions.unwrap_or_default();
        LimitOrder {
            maker_asset: order_info.maker_asset,
            taker_asset: order_info.taker_asset,
            making_amount: order_info.making_amount,
            taking_amount: order_info.taking_amount,
            salt: order_info.salt.unwrap_or_else(build_salt),
            from: order_info.maker,
            allowed_sender: order_info
                .allowed_sender
                .unwrap_or(ZERO_ADDRESS.to_string()),
            receiver: order_info.receiver.unwrap_or(ZERO_ADDRESS.to_string()),
            maker_asset_data: interactions.maker_asset_data.unwrap_or(ZX.to_string()),
            taker_asset_data: interactions.taker_asset_data.unwrap_or(ZX.to_string()),
            get_making_amount: interactions.get_making_amount.unwrap_or(ZX.to_string()),
            get_taking_amount: interactions.get_taking_amount.unwrap_or(ZX.to_string()),
            predicate: interactions.predicate.unwrap_or(ZX.to_string()),
            permit: interactions.permit.unwrap_or(ZX.to_string()),
            pre_interaction: interactions.pre_interaction.unwrap_or(ZX.to_string()),
            post_interaction: interactions.post_interaction.unwrap_or(ZX.to_string()),
        }
    }

    // Add other methods (e.g., `get_order_hash`, `get_typed_data`, `decode`, `build`, etc.) here.
}
