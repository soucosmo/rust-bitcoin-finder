use num_bigint::BigUint;

pub struct Wallet {
    pub min: BigUint,
    pub max: BigUint,
    pub address: String,
    pub status: u8,
}
