use secp256k1::{PublicKey, Secp256k1, SecretKey};
use num_bigint::BigUint;

pub fn generate_public_key(priv_key_int: &BigUint) -> Vec<u8> {
    let priv_key_bytes = priv_key_int.to_bytes_be();

    let priv_key_bytes = if priv_key_bytes.len() < 32 {
        let mut padded = vec![0u8; 32 - priv_key_bytes.len()];
        padded.extend_from_slice(&priv_key_bytes);
        padded
    } else {
        priv_key_bytes
    };

    // Cria uma nova chave privada usando o pacote secp256k1
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&priv_key_bytes).expect("32 bytes, within curve order");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    // Obtém a chave pública correspondente no formato comprimido
    public_key.serialize().to_vec()
}