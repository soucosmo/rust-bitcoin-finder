use std::fs::File;
use std::io::Write;

use num_bigint::BigInt;

const LAST_KEY_FILE: &str = "last_key.txt";

pub fn write_last_key(key: &BigInt) {
    let mut file = File::create(LAST_KEY_FILE).unwrap();
    let key_hex = format!("{:064x}", key);
    file.write_all(key_hex.as_bytes()).unwrap();
}