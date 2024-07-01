use std::fs::OpenOptions;
use std::io::Write;

pub fn write_to_file(filename: &str, content: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();
    writeln!(file, "{}", content).unwrap();
}