use num_bigint::BigInt;

pub fn increment_and_format_key(key: &BigInt) -> String {
    // Incrementa a chave
    let incremented_key = key + BigInt::from(1);

    // Converte para hexadecimal e formata para 64 caracteres
    format!("{:064x}", incremented_key)
}