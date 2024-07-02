use num_bigint::BigInt;

pub struct Wallet {
    pub min: BigInt,
    pub max: BigInt,
    pub address: String,
    pub status: u8,
}
