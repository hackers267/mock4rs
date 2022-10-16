use crate::base::integer;

/// Generate a random i64;
/// 随机生成一个i64类型的值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_i64_simple;
/// let  i = random_i64_simple();
/// println!("i:{}", i);
/// ```
pub fn random_i64_simple() -> i64 {
    integer::random_integer(i64::MIN, i64::MAX)
}

/// Generate a random i64 based on the given maximum value;
/// 根据指定的最大值随机生成一个i64值
///
/// # Arguments
///
/// - `max`: 指定的最大值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_i64_max;
/// let  i = random_i64_max(100);
/// println!("i: {}", i);
/// ```
pub fn random_i64_max(max: i64) -> i64 {
    integer::random_integer(i64::MIN, max)
}

/// Generate a random i64 based on the given minimum value;
/// 根据指定的最小值随机生成一个i64值
///
/// # Arguments
///
/// - `min`: 指定的最小值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_i64_min;
/// let  i = random_i64_min(100);
/// println!("i: {}", i);
/// ```
pub fn random_i64_min(min: i64) -> i64 {
    integer::random_integer(min, i64::MAX)
}

/// Generate a random i64 based on the given minimum and maximum value;
/// 根据指定的最小值和最大值随机生成一个i64值
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
/// use mock4rs::base::random_i64;
/// let  i = random_i64(100,200);
/// println!("i: {}", i);
/// ```
pub fn random_i64(min: i64, max: i64) -> i64 {
    integer::random_integer(min, max)
}
