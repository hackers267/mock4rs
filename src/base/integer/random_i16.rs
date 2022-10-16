use crate::base::integer;

/// Generate a random i16;
/// 随机生成一个i16类型的值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_i16_simple;
/// let  i = random_i16_simple();
/// println!("i:{}", i);
/// ```
pub fn random_i16_simple() -> i16 {
    integer::random_integer(i16::MIN, i16::MAX)
}

/// Generate a random i16 based on the given maximum value.
/// 根据指定的最大值随机生成一个i16值
///
/// # Arguments
///
/// - `max`: 指定的最大值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_i16_max;
/// let  i = random_i16_max(100);
/// println!("i: {}", i);
/// ```
pub fn random_i16_max(max: i16) -> i16 {
    integer::random_integer(i16::MIN, max)
}

/// Generate a random i16 based on the given minimum value.
/// 根据指定的最小值随机生成一个i16值
///
/// # Arguments
///
/// - `min`: 指定的最小值
///
/// returns 随机值
///
/// # Example 示例
/// ```
/// use mock4rs::base::random_i16_min;
/// let  i = random_i16_min(100);
/// println!("i: {}", i);
/// ```
pub fn random_i16_min(min: i16) -> i16 {
    integer::random_integer(min, i16::MAX)
}

/// Generate a random i16 based on the given minimum and maximum value.
/// 根据指定的最小值和最大值随机生成一个i16值
///
/// # Arguments
///
/// - `min`: 指定的最小值
/// - `max`: 指定的最大值
///
/// returns 随机值
///
/// # Example 示例
///
/// ```
/// use mock4rs::base::random_i16;
/// let  i = random_i16(100,200);
/// println!("i: {}", i);
/// ```
pub fn random_i16(min: i16, max: i16) -> i16 {
    integer::random_integer(min, max)
}
