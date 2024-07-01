use hex;
use num_bigint::BigInt;
use num_traits::FromPrimitive;

pub fn increment_and_format_key(keys: &mut [BigInt]) -> Vec<String> {
    let mut formatted_keys = Vec::new();

    for key in keys.iter_mut() {
        // Incrementa a chave em 1
        *key += BigInt::from_u8(1).unwrap();

        // Converte os bytes para hexadecimal e formata para 64 caracteres
        let key_bytes = key.to_signed_bytes_le(); // Obtenha os bytes, considerando o sinal
        let key_bytes = &key_bytes[1..]; // Obtenha somente os bytes, ignorando o sinal

        let mut pkey_hex = hex::encode(&key_bytes);
        while pkey_hex.len() < 64 {
            pkey_hex = format!("0{}", pkey_hex);
        }

        formatted_keys.push(pkey_hex);
    }

    formatted_keys
}