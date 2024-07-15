use num_bigint::BigUint;

pub fn bigint_to_bytes_be(num: &BigUint) -> Vec<u8> {
    let bytes = num.to_bytes_be();

    let mut be_bytes: Vec<u8> = Vec::with_capacity(32);

    be_bytes.extend(vec![0; 32 - bytes.len()]);

    be_bytes.extend(bytes);

    be_bytes
}
