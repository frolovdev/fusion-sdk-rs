use ethers::types::U256;
use rand::Rng;

pub fn build_salt() -> U256 {
    let rand_int = rand::thread_rng().gen_range(0..1000000);

    U256::from(rand_int)
}
