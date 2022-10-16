//! base function.
//! 基本功能函数
mod integer;
mod string;

pub use integer::{
    random_i128, random_i128_max, random_i128_min, random_i128_simple, random_i16, random_i16_max,
    random_i16_min, random_i16_simple, random_i32, random_i32_max, random_i32_min,
    random_i32_simple, random_i64, random_i64_max, random_i64_min, random_i64_simple, random_i8,
    random_i8_max, random_i8_min, random_i8_simple, random_u128, random_u128_max, random_u128_min,
    random_u128_simple, random_u16, random_u16_max, random_u16_min, random_u16_simple, random_u32,
    random_u32_max, random_u32_min, random_u32_simple, random_u64, random_u64_max, random_u64_min,
    random_u64_simple, random_u8, random_u8_max, random_u8_min, random_u8_simple,
};
pub use string::random_string_by_len;
