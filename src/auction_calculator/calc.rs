use ethers::types::U256;

pub fn linear_interpolation(t1: U256, t2: U256, v1: U256, v2: U256, t: U256) -> U256 {
    let v = (t - t1) * v2 + (t2 - t) * v1;
    v / (t2 - t1)
}
