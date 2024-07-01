use crate::config::KEYS_PATH;
use num_bigint::BigInt;
use super::make_dir;
use std::path::Path;
use std::io::Write;
use std::fs::File;


pub fn write_last_key(key: &BigInt, user_choice: u8) {
    make_dir(KEYS_PATH);

    let file = format!(
        "{}/{}_last.txt",
        KEYS_PATH,
        user_choice,
    );

    let file = Path::new(&file);

    let mut file = File::create(file).unwrap();

    let key_hex = format!("{:064x}", key);

    file.write_all(key_hex.as_bytes()).unwrap();
}