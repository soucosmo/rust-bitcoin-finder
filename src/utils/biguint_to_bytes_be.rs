use num_bigint::BigUint;

pub fn bigint_to_bytes_be(num: &BigUint) -> Vec<u8> {
    let mut bytes = num.to_bytes_be();

    while bytes.len() < 32 {
        bytes.insert(0, 0);
    }

    bytes
}