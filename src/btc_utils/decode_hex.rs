use std::num::ParseIntError;

// Decodifica uma string hexadecimal para um vetor de bytes
pub fn decode_hex(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}
