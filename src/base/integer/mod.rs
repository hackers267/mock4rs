use rand::distributions::uniform::SampleUniform;
use rand::Rng;

mod random_i128;
mod random_i16;
mod random_i32;
mod random_i64;
mod random_i8;
mod random_u128;
mod random_u16;
mod random_u32;
mod random_u64;
mod random_u8;

pub use random_i128::{random_i128, random_i128_max, random_i128_min, random_i128_simple};
pub use random_i16::{random_i16, random_i16_max, random_i16_min, random_i16_simple};
pub use random_i32::{random_i32, random_i32_max, random_i32_min, random_i32_simple};
pub use random_i64::{random_i64, random_i64_max, random_i64_min, random_i64_simple};
pub use random_i8::{random_i8, random_i8_max, random_i8_min, random_i8_simple};
pub use random_u128::{random_u128, random_u128_max, random_u128_min, random_u128_simple};
pub use random_u16::{random_u16, random_u16_max, random_u16_min, random_u16_simple};
pub use random_u32::{random_u32, random_u32_max, random_u32_min, random_u32_simple};
pub use random_u64::{random_u64, random_u64_max, random_u64_min, random_u64_simple};
pub use random_u8::{random_u8, random_u8_max, random_u8_min, random_u8_simple};

fn random_integer<T>(min: T, max: T) -> T
where
    T: PartialOrd + SampleUniform,
{
    let range = min..=max;
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}
