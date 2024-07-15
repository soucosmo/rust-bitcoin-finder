use super::{generate_public_key, hash160, murmur_hash};
use secp256k1::{Context, Secp256k1};
use num_bigint::BigUint;


pub fn verify_address_with_murmur<C: Context + secp256k1::Signing>(priv_key_int: &BigUint, target_hash: u32, secp: &Secp256k1<C>) -> bool {
    let pub_key = generate_public_key(priv_key_int, secp);
    let pub_key_hash160 = hash160(&pub_key);
    let pub_key_murmur = murmur_hash(&pub_key_hash160);

    pub_key_murmur == target_hash
}
