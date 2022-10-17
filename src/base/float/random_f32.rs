//! Generates a random f32.
//! 随机生成一个f32值
use crate::base::random_range;

/// Generates a random f32 based on the given minimum and maximum values.
/// 根据指定的最小值和最大值随机生成一个f32值
///
/// # Arguments
///
/// - `min`: 指定的最小值
/// - `max`: 指定的最大值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_f32;
/// let  f = random_f32(100.0,200.0);
/// println!("f:{}",f);
/// ```
pub fn random_f32(min: f32, max: f32) -> f32 {
    random_range(min, max)
}

/// Generates a random f32 based on the given maximum value.
/// 根据指定的最大值随机生成一个f32值
///
/// # Arguments
///
/// - `max`: 指定的最大值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_f32_max;
/// let  f = random_f32_max(100.0);
/// println!("f:{}",f);
/// ```
pub fn random_f32_max(max: f32) -> f32 {
    random_range(f32::MIN / 2.0, max)
}

/// Generates a random f32 based on the given minimum values.
/// 根据指定的最小值随机生成一个f32值
///
/// # Arguments
///
/// - `min`: 指定的最小值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_f32_min;
/// let  f = random_f32_min(100.0);
/// println!("f:{}",f);
/// ```
pub fn random_f32_min(min: f32) -> f32 {
    random_range(min, f32::MAX / 2.0)
}

/// Generates a random f32.
/// 随机生成一个f32值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_f32_simple;
/// let  simple = random_f32_simple();
/// println!("simple: {}", simple);
/// ```
pub fn random_f32_simple() -> f32 {
    random_range(f32::MIN / 2.0, f32::MAX / 2.0)
}
