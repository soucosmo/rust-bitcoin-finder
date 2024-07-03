use sha2::{Sha256, Digest as ShaDigest};
use ripemd160::{Ripemd160, Digest};

pub fn hash160(data: &[u8]) -> Vec<u8> {
    let sha256_hash = Sha256::digest(data);
    let ripemd160_hash = Ripemd160::digest(&sha256_hash);
    ripemd160_hash.to_vec()
}
