use crate::char::{random_char_bytes, CharType};

/// Generates a random string based on a given length.
/// 根据指定的类型和长度随机生成一个字符串
pub fn random_string_by_len(char_type: CharType, len: u32) -> String {
    let range = 1..=len;
    range.map(|_i| random_char_bytes(char_type)).collect()
}
