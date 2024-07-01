use num_bigint::BigInt;

pub fn increment_and_format_key(key: &mut BigInt) -> String {
    // Incrementa a chave
    *key += 1;

    // Converte a chave para hexadecimal e formata para 64 caracteres
    let mut pkey_hex = format!("{:x}", key);
    while pkey_hex.len() < 64 {
        pkey_hex = format!("0{}", pkey_hex);
    }

    pkey_hex
}