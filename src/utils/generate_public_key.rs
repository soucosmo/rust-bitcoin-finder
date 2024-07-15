use secp256k1::{PublicKey, Secp256k1, SecretKey};
use super::bigint_to_bytes_be;
use num_bigint::BigUint;
use secp256k1::Context;


pub fn generate_public_key<C: Context + secp256k1::Signing>(priv_key_int: &BigUint, secp: &Secp256k1<C>) -> [u8; 33] {
    let priv_key_bytes = bigint_to_bytes_be(priv_key_int);

    // Cria uma nova chave privada usando o pacote secp256k1
    let secret_key = SecretKey::from_slice(&priv_key_bytes).expect("32 bytes, within curve order");
    let public_key = PublicKey::from_secret_key(secp, &secret_key);

    // Obtém a chave pública correspondente no formato comprimido
    public_key.serialize()
}