use crate::base::integer;

/// Generate a random u64;
/// 随机生成一个u64类型的值
///
/// # Example 示例
/// ```
/// use mock::base::random_u64_simple;
/// let u = random_u64_simple();
/// println!("u:{}",u);
/// ```
pub fn random_u64_simple() -> u64 {
    integer::random_integer(u64::MIN, u64::MAX)
}

/// Generate a random u64 based on the given maximum value.
/// 根据指定的最大值随机生成一个u64值
/// # Arguments
///
/// * `max`: 指定的最大值
///
/// returns: u64 随机值
///
/// # Examples
///
/// ```
/// use mock::base::random_u64_max;
/// let u = random_u64_max(100);
/// println!("u:{}",u);
/// ```
pub fn random_u64_max(max: u64) -> u64 {
    integer::random_integer(u64::MIN, max)
}

/// Generate a random u64 based on the given minimum value.
/// 根据指定的最小值随机生成一个u64值
/// # Arguments
///
/// * `min`: 指定的最小值
///
/// returns: u64 随机值
///
/// # Examples
///
/// ```
/// use mock::base::random_u64_min;
/// let u64 = random_u64_min(100);
/// println!("u:{}",u64);
/// ```
pub fn random_u64_min(min: u64) -> u64 {
    integer::random_integer(min, u64::MAX)
}

/// Generates a random u64 based on the given minimum and maximum value.
/// 根据指定的最小值和最大值随机生成一个u64值
///
/// # Arguments
///
/// * `min`: 指定的最小值
/// * `max`: 指定的最大值
///
/// returns: u64
///
/// # Examples
///
/// ```
/// use mock::base::random_u64;
/// let u64 = random_u64(100,200);
/// println!("u:{}",u64);
/// ```
pub fn random_u64(min: u64, max: u64) -> u64 {
    integer::random_integer(min, max)
}
