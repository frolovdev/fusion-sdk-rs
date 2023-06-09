use ethers::types::{Address, U256};

pub const NATIVE_CURRENCY: &str = "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
pub const ZERO_ADDRESS: Address = Address::zero();
pub fn zero_number() -> U256 {
    U256::from(0)
}

pub const ZX: &str = "0x";

#[derive(Debug)]
pub enum NetworkEnum {
    ETHEREUM = 1,
    POLYGON = 137,
    BINANCE = 56,
}

pub const WRAPPER_ADDRESS_MAP: [(NetworkEnum, &str); 3] = [
    (
        NetworkEnum::ETHEREUM,
        "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
    ),
    (
        NetworkEnum::BINANCE,
        "0xbb4cdb9cbd36b01bd1cbaebf2de08d9173bc095c",
    ),
    (
        NetworkEnum::POLYGON,
        "0x0d500b1d8e8ef31e21c99d1db9a6444d3adf1270",
    ),
];

pub const UNWRAPPER_CONTRACT_ADDRESS_MAP: [(NetworkEnum, &str); 3] = [
    (
        NetworkEnum::ETHEREUM,
        "0x08b067ad41e45babe5bbb52fc2fe7f692f628b06",
    ),
    (
        NetworkEnum::BINANCE,
        "0x0eee00137d807a461702e9e0640c599de663e7e4",
    ),
    (
        NetworkEnum::POLYGON,
        "0x18d410f651289bb978fc32f90d2d7e608f4f4560",
    ),
];

pub const SETTLEMENT_CONTRACT_ADDRESS_MAP: [(NetworkEnum, &str); 3] = [
    (
        NetworkEnum::ETHEREUM,
        "0xa88800cd213da5ae406ce248380802bd53b47647",
    ),
    (
        NetworkEnum::BINANCE,
        "0x1d0ae300eec4093cee4367c00b228d10a5c7ac63",
    ),
    (
        NetworkEnum::POLYGON,
        "0x1e8ae092651e7b14e4d0f93611267c5be19b8b9f",
    ),
];

pub const ONE_INCH_ROUTER_V5: &str = "0x1111111254eeb25477b68fb85ed929f73a960582";
