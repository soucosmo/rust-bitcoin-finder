use num_traits::{ToPrimitive, Euclid, Zero};
use super::BASE58_ALPHABET;
use num_bigint::BigUint;


pub fn encode(input: &[u8]) -> String {
    let mut x = BigUint::from_bytes_be(input);
    let base = BigUint::from(BASE58_ALPHABET.len() as u32);
    let zero = BigUint::zero();

    let mut result = Vec::new();

    while x > zero {
        let (div, rem) = x.div_rem_euclid(&base);
        result.push(BASE58_ALPHABET[rem.to_usize().unwrap()]);
        x = div;
    }

    result.reverse();

    for &b in input {
        if b != 0 {
            break;
        }
        result.insert(0, BASE58_ALPHABET[0]);
    }

    String::from_utf8(result).unwrap()
}
