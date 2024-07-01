use crate::config::KEYS_PATH;
use std::fs::OpenOptions;
use super::make_dir;
use std::io::Write;

pub fn write_to_file(filename: &str, content: &str) {
    make_dir(KEYS_PATH);

    let filename = format!("{}/{}", KEYS_PATH, filename);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();
    
    writeln!(file, "{}", content).unwrap();
}