use crate::char::{random_char_bytes, CharType};

pub fn random_string(char_type: CharType, len: u32) -> String {
    let range = 1..=len;
    range.map(|_i| random_char_bytes(char_type)).collect()
}
