use std::fs::File;
use num_bigint::BigInt;
use num_traits::Num;
use std::io::Read;

const LAST_KEY_FILE: &str = "last_key.txt";

pub fn read_last_key() -> Option<BigInt> {
    if let Ok(mut file) = File::open(LAST_KEY_FILE) {
        let mut key_hex = String::new();
        if file.read_to_string(&mut key_hex).is_ok() {
            match BigInt::from_str_radix(&key_hex, 16) {
                Ok(key) => return Some(key),
                Err(_) => return None,
            }
        }
    }
    None
}