use murmur3::murmur3_32;
use std::io::Cursor;

pub fn murmur_hash(data: &[u8]) -> u32 {
    murmur3_32(&mut Cursor::new(data), 0).expect("MurmurHash computation failed")
}
