use super::murmur_hash;
use bitcoin::base58;

pub fn get_target_hash_from_address(address: &str) -> u32 {
    // Passo 1: Decode from Base58
    let decoded = base58::decode(address).expect("Base58 decoding failed");

    // Passo 2: Remove checksum and version byte
    let hash160 = &decoded[1..decoded.len() - 4];

    // Passo 3: Apply MurmurHash to the hash160
    murmur_hash(&hash160)
}
