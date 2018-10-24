extern crate sha3;
use self::sha3::{Digest, Sha3_256};

pub fn generate_hash_string(value: String) -> String {
    let mut hasher = Sha3_256::new();
    hasher.input(value.as_bytes());
    let result = hasher.result();
    to_hex_string(&result.as_slice())
}

pub fn to_hex_string(bytes: &[u8]) -> String {
    let strs: Vec<String> = bytes.iter()
                                .map(|b| format!("{:02X}", b))
                                .collect();
    strs.join("")
}