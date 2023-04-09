use ethers::types::U256;

pub fn no_public_resolving_deadline() -> U256 {
    U256::from(4102444800 as u64)
}
