//! Generates a random f32 or f64.
//! 随机生成一个浮点数，32位浮点数(f32)或64位浮点数(f64)。
mod random_f32;
mod random_f64;

pub use random_f32::{random_f32, random_f32_max, random_f32_min, random_f32_simple};
pub use random_f64::{random_f64, random_f64_max, random_f64_min, random_f64_simple};
