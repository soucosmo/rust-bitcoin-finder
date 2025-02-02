use crate::config::KEYS_PATH;
use num_bigint::BigUint;
use std::path::Path;
use num_traits::Num;
use super::make_dir;
use std::fs::File;
use std::io::Read;

pub fn read_last_key(user_choice: u8) -> Option<BigUint> {
    make_dir(KEYS_PATH);

    let file = format!(
        "{}/{}_last.txt",
        KEYS_PATH,
        user_choice,
    );

    let file = Path::new(&file);

    if let Ok(mut file) = File::open(file) {
        let mut key_hex = String::new();
        if file.read_to_string(&mut key_hex).is_ok() {
            match BigUint::from_str_radix(&key_hex, 16) {
                Ok(key) => return Some(key),
                Err(_) => return None,
            }
        }
    }
    None
}