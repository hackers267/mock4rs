use crate::base::integer;

/// Generate a random u128;
/// 随机生成一个u128类型的值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_u128_simple;
/// let u = random_u128_simple();
/// println!("u:{}",u);
/// ```
pub fn random_u128_simple() -> u128 {
    integer::random_integer(u128::MIN, u128::MAX)
}

/// Generate a random u128 based on the given maximum value.
/// 根据指定的最大值随机生成一个u128值
///
/// # Arguments
///
/// - `max`: 指定的最大值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_u128_max;
/// let u = random_u128_max(100);
/// println!("u:{}",u);
/// ```
pub fn random_u128_max(max: u128) -> u128 {
    integer::random_integer(u128::MIN, max)
}

/// Generate a random u128 based on the given minimum value.
/// 根据指定的最小值随机生成一个u128值
///
/// # Arguments
///
///  - `min`: 指定的最小值
///
/// returns 随机值
///
/// # Examples 示例
/// ```
/// use mock4rs::base::random_u128_min;
/// let u = random_u128_min(100);
/// println!("u:{}",u);
/// ```
pub fn random_u128_min(min: u128) -> u128 {
    integer::random_integer(min, u128::MAX)
}

/// Generate a random u128 based on the given minimum and maximum value.
/// 根据指定的最小值和最大值随机生成一个u128值
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
/// use mock4rs::base::random_u128;
/// let u = random_u128(100,200);
/// println!("u:{}",u);
/// ```
pub fn random_u128(min: u128, max: u128) -> u128 {
    integer::random_integer(min, max)
}
