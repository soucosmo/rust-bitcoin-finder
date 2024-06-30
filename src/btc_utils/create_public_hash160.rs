use secp256k1::{Secp256k1, SecretKey};
use super::{decode_hex, hash160};
use std::num::ParseIntError;


pub fn create_public_hash160(priv_key_int: &str) -> Result<Vec<u8>, ParseIntError> {
    let priv_key_bytes = decode_hex(priv_key_int)?;

    // Cria uma nova chave privada usando o pacote secp256k1
    let secp = Secp256k1::new();
    let priv_key = SecretKey::from_slice(&priv_key_bytes).expect("32 bytes, within curve order");

    // Obtém a chave pública correspondente em formato comprimido
    let pub_key = secp256k1::PublicKey::from_secret_key(&secp, &priv_key).serialize();

    // Gera um endereço Bitcoin a partir da chave pública
    Ok(hash160(&pub_key))
}
