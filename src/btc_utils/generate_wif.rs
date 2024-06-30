
use super::{decode_hex, sha256};
use std::num::ParseIntError;
use base58::ToBase58;

// Gera WIF a partir da chave privada
pub fn generate_wif(priv_key_int: &str) -> Result<String, ParseIntError> {
    let priv_key_bytes = decode_hex(priv_key_int)?;

    // Adiciona prefixo e sufixo
    let mut extended_key = vec![0x80];
    extended_key.extend_from_slice(&priv_key_bytes);
    extended_key.push(0x01);

    // Calcula o checksum
    let first_sha = sha256(&extended_key);
    let second_sha = sha256(&first_sha);
    let checksum = &second_sha[0..4];

    // Adiciona o checksum
    extended_key.extend_from_slice(checksum);

    // Codifica para base58
    Ok(extended_key.to_base58())
}
