//! Generates a random f32.
//! 随机生成一个f32值
use crate::base::random_range;

/// Generates a random f64 based on the given minimum and maximum values.
/// 根据指定的最小值和最大值随机生成一个f64值
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
/// use mock4rs::base::random_f64;
/// let  f = random_f64(100.0,200.0);
/// println!("f:{}",f);
/// ```
pub fn random_f64(min: f64, max: f64) -> f64 {
    random_range(min, max)
}

/// Generates a random f64 based on the given maximum value.
/// 根据指定的最大值随机生成一个f64值
///
/// # Arguments
///
/// - `max`: 指定的最大值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_f64_max;
/// let  max = random_f64_max(1000.0);
/// println!("max: {}", max);
/// ```
pub fn random_f64_max(max: f64) -> f64 {
    random_range(f64::MIN / 2.0, max)
}

/// Generates a random f32 based on the given minimum values.
/// 根据指定的最小值随机生成一个f64值
///
/// # Arguments
///
/// - `min`: 指定的最小值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_f64_min;
/// let  min = random_f64_min(200808.08);
/// println!("min: {}", min);
/// ```
pub fn random_f64_min(min: f64) -> f64 {
    random_range(min, f64::MAX / 2.0)
}

/// Generates a random f64.
/// 随机生成一个f64值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_f64_simple;
/// let  simple = random_f64_simple();
/// println!("simple: {}", simple);
/// ```
pub fn random_f64_simple() -> f64 {
    random_range(f64::MIN / 2.0, f64::MAX / 2.0)
}
