use crate::base::integer;

/// Generate a random i128;
/// 随机生成一个i128类型的值
///
/// # Example 示例
/// ```
/// use mock::base::random_i128_simple;
/// let  i = random_i128_simple();
/// println!("{}", i);
/// ```
pub fn random_i128_simple() -> i128 {
    integer::random_integer(i128::MIN, i128::MAX)
}

/// Generate a random i128 based on the given maximum value.
/// 根据指定的最大值随机生成一个i128值
///
/// # Arguments
///
/// - `max`: 指定的最大值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock::base::random_i128_max;
/// let  i = random_i128_max(100);
/// println!("i: {}", i);
/// ```
pub fn random_i128_max(max: i128) -> i128 {
    integer::random_integer(i128::MIN, max)
}

/// Generate a random i128 based on the given minimum value.
/// 根据指定的最小值随机生成一个i128值
///
/// # Arguments
///
/// - `min`: 指定的最小值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock::base::random_i128_min;
/// let  i = random_i128_min(100);
/// println!("i: {}", i);
/// ```
pub fn random_i128_min(min: i128) -> i128 {
    integer::random_integer(min, i128::MAX)
}

/// Generate a random i128 based on the given minimum and maximum value.
/// 根据指定的最小值和最大值随机生成一个i128值
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
/// use mock::base::random_i128;
/// let  i = random_i128(100,200);
/// println!("i: {}", i);
/// ```
pub fn random_i128(min: i128, max: i128) -> i128 {
    integer::random_integer(min, max)
}
