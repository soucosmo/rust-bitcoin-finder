
use super::bigint_to_bytes_be;
use sha2::{Sha256, Digest};
use num_bigint::BigUint;
use bitcoin::base58;


pub fn generate_wif(priv_key: &BigUint) -> String {
    let mut priv_key_bytes = bigint_to_bytes_be(priv_key);

    // Adiciona o byte de versão no início
    let mut extended_key = vec![0x80];
    extended_key.append(&mut priv_key_bytes);

    // Adiciona byte de sufixo para indicar chave privada comprimida
    extended_key.push(0x01);

    // Calcula o checksum
    let first_sha = Sha256::digest(&extended_key);
    let second_sha = Sha256::digest(&first_sha);
    let checksum = &second_sha[..4];

    // Concatena tudo (versão + chave privada + sufixo + checksum)
    let mut final_key = extended_key;
    final_key.extend_from_slice(checksum);

    // Codifica para Base58
    base58::encode(&final_key)
}