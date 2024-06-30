
use ripemd160::{Ripemd160, Digest as RipemdDigest};
use sha2::{Sha256, Digest as ShaDigest};

// Calcula o hash RIPEMD160(SHA256(b))
pub fn hash160(data: &[u8]) -> Vec<u8> {
    let mut sha256 = Sha256::new();
    sha256.update(data);
    let sha256_hash = sha256.finalize();

    let mut ripemd160 = Ripemd160::new();
    ripemd160.update(sha256_hash);
    ripemd160.finalize().to_vec()
}
