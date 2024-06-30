use super::BASE58_ALPHABET;
use num_bigint::BigUint;
use num_traits::Zero;

pub fn decode(input: &str) -> Vec<u8> {
    let mut answer = BigUint::zero();
    let base = BigUint::from(58u32);

    for c in input.bytes() {
        let val = match BASE58_ALPHABET.iter().position(|&x| x == c) {
            Some(val) => val,
            None => return vec![],
        };
        answer = answer * &base + BigUint::from(val);
    }

    let mut result = answer.to_bytes_be();

    for c in input.bytes() {
        if c != BASE58_ALPHABET[0] {
            break;
        }
        result.insert(0, 0);
    }

    result
}
