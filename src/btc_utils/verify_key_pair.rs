use secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::util::key::PublicKey as BitcoinPublicKey;
use std::str::FromStr;

pub fn verify_key_pair(private_key_hex: &str, public_key_hex: &str) -> Result<bool, secp256k1::Error> {
    // Inicializa a instância da biblioteca secp256k1
    let secp = Secp256k1::new();

    // Converte a chave privada de hexadecimal para bytes
    let private_key_bytes = hex::decode(private_key_hex).map_err(|_| secp256k1::Error::InvalidSecretKey)?;

    // Cria a chave privada a partir dos bytes
    let secret_key = SecretKey::from_slice(&private_key_bytes)?;

    // Deriva a chave pública a partir da chave privada
    let derived_public_key = PublicKey::from_secret_key(&secp, &secret_key);

    // Converte a chave pública derivada para hexadecimal
    let derived_public_key_hex = hex::encode(derived_public_key.serialize());

    // Verifica se a chave pública derivada é igual à chave pública fornecida
    Ok(derived_public_key_hex == public_key_hex)
}